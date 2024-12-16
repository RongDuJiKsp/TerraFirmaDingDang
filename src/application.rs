use crate::frontend::args::ApplicationArgs;
use crate::storage::rec_save::RecordSaver;

pub fn init() {
    //check args
    //load-config 和 save-as 不能同时使用
    if ApplicationArgs::instance().is_configuration_conflicts() {
        panic!("load-config 和 save-as 不能同时使用");
    }
    //init storage
    //若用户指定了load-config or save-as 才初始化配置记录器
    if ApplicationArgs::instance().should_load_storage() {
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
