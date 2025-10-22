use std::process;

use mds_backend::server_run;

#[tokio::main]
async fn main() {
    if let Err(e) = server_run().await {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
