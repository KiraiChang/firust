mod services;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let result = services::crawlers::exchange::grab("2024/10/01", "2024/10/31").await.unwrap();
    println!("exchange result: {:?}", result);

    Ok(())
}