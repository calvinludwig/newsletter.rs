use crate::helpers::spawn_app;

#[tokio::test]
async fn health_check_works() {
	let app = spawn_app().await;
	let client = reqwest::Client::new();

	let response = match client
		.get(&format!("{}/health_check", &app.address))
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
