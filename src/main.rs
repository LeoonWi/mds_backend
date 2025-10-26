use std::error::Error;

use mds_backend::run;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    run().await
}
