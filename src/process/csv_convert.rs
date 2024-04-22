use anyhow::Result;
use serde::Serialize;

use crate::cli::OutputFormat;

#[derive(Debug, Serialize)]
pub struct TomlWrapperRecord {
    pub data: Vec<serde_json::Value>,
}

/// 转换数据
pub fn process_csv(input: &str, output: &str, format: OutputFormat) -> Result<()> {
    // 读取文件
    let mut reader = csv::ReaderBuilder::new().from_path(input)?;
    // 结果集
    let mut record = vec![];
    // 获取headers
    let headers = reader.headers()?.clone();
    // 遍历结果
    for row in reader.records() {
        // 使用迭代器 合并创建元组对象 (header,column)
        let row = headers
            .iter()
            .zip(row?.iter())
            .collect::<serde_json::Value>();
        // 添加到结果集
        record.push(row);
    }
    //  转换数据格式
    let content = match format {
        OutputFormat::Json => serde_json::to_string_pretty(&record)?,
        OutputFormat::Yaml => serde_yaml::to_string(&record)?,
        OutputFormat::Toml => toml::to_string_pretty(&TomlWrapperRecord { data: record })?,
    };

    // 将结果写入文件
    std::fs::write(output, content)?;

    Ok(())
}
