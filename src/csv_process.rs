use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::cli::CsvOpts;

/// 数据记录
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DataRecord {
    name: String,
    position: String,
    #[serde(rename = "DOB")]
    dob: String,
    nationality: String,
    #[serde(rename = "Kit Number")]
    kit_number: u8,
}

/// 转换数据
pub fn parse(opts: &CsvOpts) -> Result<()> {
    // 读取文件
    let mut reader = csv::ReaderBuilder::new()
        .delimiter(opts.delimiter)
        .from_path(&opts.input)?;
    // 结果集
    let mut data_result = vec![];

    for result in reader.deserialize() {
        let record: DataRecord = result?;
        data_result.push(record);
    }

    let json = serde_json::to_string_pretty(&data_result)?;

    // 将结果写入文件
    std::fs::write(&opts.output, json)?;

    Ok(())
}
