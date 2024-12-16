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
        short,
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
    //TF核心
    #[arg(
        short,
        long,
        help = "\
    参数内容为：将锻造指针由初始值打造为对齐的步骤
    例子：TFUU 为从初始值进行 轻击-镦锻-收缩-收缩
    填写对照表：
    轻击 (Tapping): T    击打 (Hammering): H
    重击 (HeavyHammering): X  牵拉 (Drawing): D
    冲压 (Stamping): S    弯曲 (Bending): B
    镦锻 (Forging): F     收缩 (Upsetting): U
    "
    )]
    pub alignment_step: Option<String>,
}
impl ApplicationArgs {}
pub static APPLICATION_ARGS: LazyLock<ApplicationArgs> = LazyLock::new(|| ApplicationArgs::parse());
