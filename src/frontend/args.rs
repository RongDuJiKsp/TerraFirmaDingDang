use clap::Parser;
use std::sync::LazyLock;

#[derive(Parser, Debug)]

pub struct ApplicationArgs {
    //记录存储相关
    #[arg(
        short,
        long,
        help = "若提供这个参数，则以该参数为key将结构保存到结构文件中"
    )]
    pub save_as: Option<String>,
    #[arg(
        short = 'c',
        long,
        help = "若提供这个参数，则从文件中以这个key读取结构，同时忽略后面给出的具体参数"
    )]
    pub load_config: Option<String>,
    #[arg(
        short,
        long,
        help = "若提供这个参数，则从用户目录中读取结构文件，而不是可执行文件目录"
    )]
    pub global: bool,
    #[arg(
        short,
        long,
        help = "若提供这个参数，指读取key指向的所有结构，而不是这个key的首个结果"
    )]
    pub multi_key: bool,
    #[arg(
        short,
        long,
        help = "\
        若提供这个参数，则不输出提示性文字，而是与输入序列相同的格式的字符串
        输出结果按照以下格式：
        若数据来源为实时计算，则输出两行，第一行是挽救对齐工件的步骤，第二行是从头开始的步骤：
        若数据来源为从保存的文件中读取，则输出1或n行，每行均为匹配到key的从头开始的步骤
        "
    )]
    pub pipe: bool,
    //TF核心
    #[arg(
        short,
        long,
        help = "\
    参数内容为：将锻造指针由初始值打造为对齐的步骤
    例子：TFUU 为从初始值进行 轻击-镦锻-收缩-收缩
    填写对照表：
    轻击 (Tapping): T         击打 (Hammering): H
    重击 (HeavyHammering): X  牵拉 (Drawing): D
    冲压 (Stamping): S        弯曲 (Bending): B
    镦锻 (Forging): F         收缩 (Upsetting): U
    "
    )]
    pub alignment_step: Option<String>,
    #[arg(
        short,
        long,
        help = "\
    参数内容为：砧给出的最后步骤的格子，按照从左到右计算
    例子：
    LTSFTU  为三个格子分别为 最后一步轻击-倒数第二部镦锻-倒数第三步收缩
    若第三个格子为空 则填写为 LTSFZ
    填写对照表：
    最后一步为X (Last): L         倒数第二步为X (LastSecond): S
    倒数第三步为X (LastThird): T
    非最后步骤为X (NotLast): N    任意步骤为X (Any): A
    空 (None): Z
    "
    )]
    pub last_steps: Option<String>,
}
#[allow(dead_code)]
impl ApplicationArgs {
    pub fn instance() -> &'static ApplicationArgs {
        &*APPLICATION_ARGS
    }
    pub fn is_configuration_conflicts(&self) -> bool {
        self.load_config.is_some() && self.save_as.is_some()
    }
    pub fn should_load_storage(&self) -> bool {
        self.load_config.is_some() || self.save_as.is_some()
    }
    pub fn has_all_tf_configs(&self) -> bool {
        self.alignment_step.is_some()
    }
    pub fn load_config_or_unwrap(&self) -> String {
        self.load_config
            .clone()
            .expect("未提供加载预定义配方的key --load-config")
    }
    pub fn save_as_or_unwrap(&self) -> String {
        self.save_as
            .clone()
            .expect("未提供存储预定义配方的key --save-as")
    }
    pub fn tfc_cmd_or_unwrap(&self) -> TFCommands {
        TFCommands {
            alignment_step: self
                .alignment_step
                .clone()
                .expect("未提供对齐步骤 --alignment_step"),
            last_steps: self
                .last_steps
                .clone()
                .expect("未提供最后步骤 --last_steps"),
        }
    }
}
static APPLICATION_ARGS: LazyLock<ApplicationArgs> = LazyLock::new(|| ApplicationArgs::parse());
pub struct TFCommands {
    pub alignment_step: String,
    pub last_steps: String,
}
