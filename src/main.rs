use newsletter::{configuration, email_client::EmailClient, startup, telemetry};
use sqlx::postgres::PgPoolOptions;
use std::{net::TcpListener, time::Duration};

#[tokio::main]
async fn main() -> std::io::Result<()> {
	let subscriber = telemetry::get_subscriber("newsletter".into(), "info".into(), std::io::stdout);
	telemetry::init_subscriber(subscriber);

	let configuration = configuration::get_configuration().expect("Failed to read configuration.");
	let connection_pool = PgPoolOptions::new()
		.connect_timeout(Duration::from_secs(2))
		.connect_lazy_with(configuration.database.with_db());

	let sender_email = configuration
		.email_client
		.sender()
		.expect("Invalid sender email address.");
	let email_client = EmailClient::new(configuration.email_client.base_url, sender_email);

	let listener = TcpListener::bind(format!(
		"{}:{}",
		configuration.application.host, configuration.application.port
	))?;

	startup::run(listener, connection_pool, email_client)?.await?;
	Ok(())
}
