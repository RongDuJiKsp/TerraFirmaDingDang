use crate::frontend::args::ApplicationArgs;
use crate::storage::rec_save::RecordSaver;

pub fn init() {
    //check args
    //load-config 和 save-as 不能同时使用
    if ApplicationArgs::instance().load_config.is_some()
        && ApplicationArgs::instance().save_as.is_some()
    {
        panic!("load-config 和 save-as 不能同时使用");
    }
    //init storage
    //若用户指定了load-config or save-as 才初始化配置记录器
    if ApplicationArgs::instance().load_config.is_some()
        || ApplicationArgs::instance().save_as.is_some()
    {
        if ApplicationArgs::instance().global {
            //读取全局配置文件
            RecordSaver::instance().load_user();
        } else {
            //读取局部配置文件
            RecordSaver::instance().load_exec();
        }
    }
}
pub fn run() {}
