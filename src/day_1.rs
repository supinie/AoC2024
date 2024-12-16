use crate::util;

pub async fn answer() -> Result<u32, anyhow::Error> {
    let input = util::get_input(1).await?;
    Ok(1)
}
