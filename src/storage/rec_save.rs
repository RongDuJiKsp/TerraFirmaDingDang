use anyhow::anyhow;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::Path;
use std::sync::{LazyLock, Mutex, MutexGuard};
type AnyResult<T> = Result<T, Box<dyn Error>>;
type ReadResult<T> = AnyResult<Option<T>>;
const SPLIT_STR: &str = "</>";
const STORAGE_NAME: &str = "TerraFirmaDingDang.db";
const HEADER_SIZE: usize = 3;
pub struct KVScanner;

impl KVScanner {
    //KV对的存储器，存储 Record<String,String>
    //结构[block size 3Bit] [block...]
    //size块的第一个字节必须为0
    //Unsafe：文件指针不是由顺序读取而来会导致未定义行为
    pub unsafe fn read_as_next_block(file: &mut File) -> ReadResult<Vec<u8>> {
        let mut pre = [0u8; HEADER_SIZE]; //first headers
        let pre_size = file.read(&mut pre)?;
        let block_size = if pre_size == 0 {
            return Ok(None);
        } else if pre_size < HEADER_SIZE || pre[0] != 0u8 {
            return Err(anyhow::anyhow!("文件已损坏：文件头损坏").into());
        } else {
            ((pre[1] as usize) << 8) + (pre[2] as usize)
        };
        let mut buffer = vec![0u8; block_size];
        let read_size = file.read(&mut buffer)?;
        if read_size < block_size {
            Err(anyhow!(
                "文件已损坏：文件被截断。expect size {} ,recv size {}",
                block_size,
                read_size
            )
            .into())
        } else {
            Ok(Some(buffer))
        }
    }
    pub fn get_kv_from_block(block: &[u8]) -> AnyResult<(String, String)> {
        let str_block = String::from_utf8(block.to_vec())?;
        let mut s = str_block.split(SPLIT_STR);
        let (key, val) = (
            s.next().ok_or("文件已损坏：KV Split Error")?.to_string(),
            s.next().ok_or("文件已损坏：KV Split Error")?.to_string(),
        );
        Ok((key, val))
    }
    //Unsafe：文件指针不是由顺序读取而来会导致未定义行为
    pub unsafe fn find_next_v_by_k(file: &mut File, key: &str) -> ReadResult<String> {
        //持续读块到文件尾部
        while let Some(block) = Self::read_as_next_block(file)? {
            let (k, v) = Self::get_kv_from_block(&block)?;
            if key == k.as_str() {
                return Ok(Some(v));
            }
        }
        Ok(None)
    }
    pub fn find_first_v_by_k(file: &mut File, key: &str) -> ReadResult<String> {
        //文件指向文件头
        file.seek(SeekFrom::Start(0))?;
        unsafe { Ok(Self::find_next_v_by_k(file, key)?) }
    }
    pub fn find_all_v_by_k(file: &mut File, key: &str) -> AnyResult<Vec<String>> {
        file.seek(SeekFrom::Start(0))?;
        let mut vs = Vec::new();
        unsafe {
            while let Some(v) = Self::find_next_v_by_k(file, key)? {
                vs.push(v)
            }
        }
        Ok(vs)
    }
    //Unsafe：文件指针没有指向末尾导致未定义行为
    pub unsafe fn append_block(file: &mut File, block: &[u8]) -> AnyResult<()> {
        let useful = block.len() as u16;
        if useful as usize > block.len() {
            return Err(anyhow!("Block过大").into());
        }
        let pre: [u8; HEADER_SIZE] = [0, ((useful >> 8) & 0xff) as u8, (useful & 0xff) as u8];
        file.write_all(&pre)?;
        file.write_all(block)?;
        Ok(())
    }
    pub fn append_kv(file: &mut File, k: &str, v: &str) -> AnyResult<()> {
        file.seek(SeekFrom::End(0))?;
        unsafe {
            Ok(Self::append_block(
                file,
                format!("{}{}{}", k, SPLIT_STR, v).as_bytes(),
            )?)
        }
    }
}
pub struct RecordSaver {
    output: Option<File>,
}
impl RecordSaver {
    //创建一个未初始化记录器
    pub fn un_init() -> Self {
        RecordSaver { output: None }
    }
    //扫描文件读取首个kv
    pub fn read_kv_first(&mut self, k: &str) -> Option<String> {
        if let Some(f) = &mut self.output {
            return KVScanner::find_first_v_by_k(f, k).expect("在查找存储的数据时发生错误");
        }
        None
    }
    //扫描文件读取所有kv
    pub fn read_kv_all(&mut self, k: &str) -> Vec<String> {
        if let Some(f) = &mut self.output {
            return KVScanner::find_all_v_by_k(f, k).expect("在查找存储的数据时发生错误");
        }
        vec![]
    }
    pub fn append_kv(&mut self, k: &str, v: &str) {
        if let Some(f) = &mut self.output {
            KVScanner::append_kv(f, k, v).expect("在写入配置项数据时发生错误");
        }
    }

    //在可执行文件目录加载配置
    pub fn load_exec(&mut self) {
        self.open_or_init(
            &env::current_exe()
                .expect("解析可执行文件路径时失败")
                .parent()
                .expect("解析可执行文件路径时失败"),
        )
    }
    //在用户目录加载配置
    pub fn load_user(&mut self) {
        self.open_or_init(
            #[allow(deprecated)]
            &env::home_dir().expect("解析用户目录时发生错误。请不要在Cygwin等环境下执行"),
        )
    }
    //尝试在给定的目录下搜索配置文件，有则打开，无则创建
    fn open_or_init(&mut self, path: &Path) {
        let file = File::options()
            .read(true)
            .write(true)
            .create(true)
            .open(path.join(STORAGE_NAME))
            .expect("在创建或打开配置文件时失败");
        self.output.replace(file);
    }
    pub fn instance() -> MutexGuard<'static, RecordSaver> {
        REC_SAVER.lock().expect("对Config服务加锁时失败")
    }
}

static REC_SAVER: LazyLock<Mutex<RecordSaver>> =
    LazyLock::new(|| Mutex::new(RecordSaver::un_init()));
