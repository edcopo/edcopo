use std::env;
use tokio_postgres::{Error, NoTls};

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv::dotenv().ok(); // Load .env file
                           // Load the DATABASE_URL from the environment
    let database_url =
        env::var("DATABASE_URL").expect("DATABASE_URL environment variable is not set");

    // Connect to the PostgreSQL server
    let (client, connection) = tokio_postgres::connect(&database_url, NoTls).await?;

    // Spawn a separate task to handle the connection
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Connection error: {}", e);
        }
    });

    // Test the connection with a simple query
    let rows = client.query("SELECT 1", &[]).await?;

    // Verify the result
    if !rows.is_empty() {
        println!("PostgreSQL connection successful!");
    } else {
        println!("Connected, but the query returned no results.");
    }

    Ok(())
}
