use anyhow::Result;
use serde::{Deserialize, Serialize};

/// 球员信息
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Player {
    /// 名称
    name: String,
    /// 位置
    position: String,
    /// 生日
    #[serde(rename = "DOB")]
    dob: String,
    /// 国籍
    nationality: String,
    /// 球衣号码
    #[serde(rename = "Kit Number")]
    kit: u8,
}

/// 转换数据
pub fn process_csv(input: &str, output: &str) -> Result<()> {
    // 读取文件
    let mut reader = csv::ReaderBuilder::new().from_path(input)?;
    // 结果集
    let mut record = vec![];

    // 遍历结果
    for row in reader.deserialize::<Player>() {
        record.push(row?);
    }
    // 转换为JSON
    let json = serde_json::to_string_pretty(&record)?;

    // 将结果写入文件
    std::fs::write(output, json)?;

    Ok(())
}
