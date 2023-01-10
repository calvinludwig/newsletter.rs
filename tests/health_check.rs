use std::net::TcpListener;

#[tokio::test]
async fn health_check_works() {
    let address = spawn_app();
    let client = reqwest::Client::new();

    let response = match client
        .get(&format!("{}/health_check", &address))
        .send()
        .await
    {
        Ok(v) => v,
        Err(e) => {
            panic!("Error: {e:?}");
        }
    };

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let server = match newsletter::run(listener) {
        Ok(v) => v,
        Err(e) => {
            panic!("Error: {e:?}");
        }
    };
    tokio::spawn(server);
    format!("http://127.0.0.1:{port}")
}