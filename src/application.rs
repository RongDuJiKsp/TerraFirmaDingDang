use crate::frontend::args::APPLICATION_ARGS;
use crate::storage::rec_save::REC_SAVER;

pub fn init() {
    //check args
    //load-config 和 save-as 不能同时使用
    if APPLICATION_ARGS.load_config.is_some() && APPLICATION_ARGS.save_as.is_some() {
        panic!("load-config 和 save-as 不能同时使用");
    }
    //init storage
    //若用户指定了load-config or save-as 才初始化配置记录器
    if APPLICATION_ARGS.load_config.is_some() || APPLICATION_ARGS.save_as.is_some() {
        if APPLICATION_ARGS.global {
            //读取全局配置文件
            REC_SAVER.lock().unwrap().load_user();
        } else {
            //读取局部配置文件
            REC_SAVER.lock().unwrap().load_exec();
        }
    }
}
pub fn run() {}
