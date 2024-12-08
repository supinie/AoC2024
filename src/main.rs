mod util;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let day_1 = util::get_input(1).await?;
    Ok(())
}
