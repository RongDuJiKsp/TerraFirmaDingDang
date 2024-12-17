use crate::tf_serde::operator::TFOperator;
use std::error::Error;
use std::io::Write;

pub fn display_ops<W: Write>(v: &Vec<TFOperator>, w: &mut W) -> Result<(), Box<dyn Error>> {
    writeln!(w, "------操作步骤如下：-------")?;
    for (idx, e) in v.iter().enumerate() {
        writeln!(w, "第{}步：{}", idx, e.chinese())?;
    }
    writeln!(w, "------操作完成------------")?;
    Ok(())
}
