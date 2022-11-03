use zero2prod::configuration::{get_configuration, Settings};
use zero2prod::startup::run;
use zero2prod::telemetry::{get_subscriber, init_subscriber};
use secrecy::ExposeSecret;
use sqlx::{PgPool, Pool, Postgres};
use std::net::TcpListener;

// cargo watch -x check -x run -x test
// export ts=$(date +%s); curl -i v -X POST -d "email=thomas_mann_$ts@hostmail.com&name=Tom$ts" http://127.0.0.1:8000/subscriptions

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("pod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration: Settings = get_configuration().expect("Failed to read configuration");

    let db_pool: Pool<Postgres> = PgPool::connect_lazy(
        &configuration.database.connection_string().expose_secret()
    ).expect("Failed to create Postgres connection pool.");

    let addr = format!("{}:{}",configuration.application.host, configuration.application.port);
    let listener = TcpListener::bind(addr)?;
    run(listener, db_pool)?.await
}