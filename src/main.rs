use newsletter::{configuration, startup, telemetry};
use secrecy::ExposeSecret;
use sqlx::PgPool;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = telemetry::get_subscriber("newsletter".into(), "info".into(), std::io::stdout);
    telemetry::init_subscriber(subscriber);

    let configuration = configuration::get_configuration().expect("Failed to read configuration.");
    let connection_pool = PgPool::connect(configuration.database.connection_string().expose_secret())
        .await
        .expect("Failed to connect to Database");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    startup::run(listener, connection_pool)?.await
}
