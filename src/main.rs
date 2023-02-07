use newsletter::startup::Application;
use newsletter::{configuration, telemetry};

#[tokio::main]
async fn main() -> std::io::Result<()> {
	let subscriber = telemetry::get_subscriber("newsletter".into(), "info".into(), std::io::stdout);
	telemetry::init_subscriber(subscriber);

	let configuration = configuration::get_configuration().expect("Failed to read configuration.");
	let application = Application::build(configuration).await?;
	application.run_until_stopped().await?;

	Ok(())
}
