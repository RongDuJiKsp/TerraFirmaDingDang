use crate::alogrithm::search::SearchSolver;
use crate::frontend::args::ApplicationArgs;
use crate::storage::rec_save::RecordSaver;
use crate::tf_serde::operator::{TFConditionOp, TFOperator};
use crate::tf_serde::stringify::SerializedList;

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
pub fn run() {
    if let Some(key) = &ApplicationArgs::instance().load_config {
        let make_new_stems_list = (if ApplicationArgs::instance().multi_key {
            RecordSaver::instance()
                .read_kv_all(key.as_str())
                .iter()
                .map(|x| TFOperator::unmarshal(x))
                .collect::<Result<Vec<_>, _>>()
        } else {
            let mut list = vec![];
            if let Some(e) = RecordSaver::instance().read_kv_first(key) {
                list.push(e);
            }
            list.iter()
                .map(|x| TFOperator::unmarshal(x))
                .collect::<Result<Vec<_>, _>>()
        })
        .expect("生成的转储文件已经损坏，请删除后重新启动");
        return;
    }
    //解析条件
    let cmd = ApplicationArgs::instance().tfc_cmd_or_unwrap();
    let start = TFOperator::unmarshal(&cmd.last_steps).expect("在反序列化起始步骤时失败");
    let condition =
        TFConditionOp::unmarshal(&cmd.alignment_step).expect("在反序列化约束步骤时失败");
    //创建求解器
    let solver = SearchSolver::with_condition([condition[0], condition[1], condition[2]]);
    //将已经对齐的铁打完
    let save_this_steps = solver.search_solve(0);
    //将没有锻造的铁打完
    let make_new_steps = solver.search_solve(
        -start
            .iter()
            .map(|x| <TFOperator as Into<i32>>::into(*x))
            .sum::<i32>(),
    );
}
