use anyhow::Result;
/// 获取Reader
pub fn get_reader(input: &str) -> Result<Box<dyn std::io::Read>> {
    match input {
        "-" => Ok(Box::new(std::io::stdin())),
        _ => Ok(Box::new(std::fs::File::open(input)?)),
    }
}
