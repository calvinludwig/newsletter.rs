use newsletter::run;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = match TcpListener::bind("127.0.0.1:8000") {
        Ok(v) => v,
        Err(e) => {
            panic!("Error: {e:?}")
        }
    };

    run(listener)?.await
}
