use std::{
    sync::Arc,
    time::Duration,
};

use anyhow::{Context, Result};
use redis::{aio::Connection, AsyncCommands, Client};
use tokio::{time::sleep, sync::Mutex};

use crate::{definitions::rest::Authorization, settings::WebServerSettings};

pub type RedisConnection = Arc<Mutex<Connection>>;

pub async fn build(settings: &WebServerSettings) -> Result<RedisConnection> {
    let client = Client::open(settings.services.redis.url())
        .context("Failed to connect to open a connection to Redis.")?;

    let connection = client
        .get_async_connection()
        .await
        .context("Failed to connect to Redis.")?;
    log::debug!("Successfully established a connection to Redis.");

    let connection = Arc::from(Mutex::new(connection));
    let conn = connection.clone();
    let settings = settings.clone();

    // Generate an internal identity every five minutes.
    tokio::spawn(async move {
        loop_into_key_generation(&settings, &conn).await;
    });

    Ok(connection)
}

pub async fn loop_into_key_generation(settings: &WebServerSettings, connection: &Mutex<Connection>) {
    let delay = Duration::from_secs(5 * 60);
    let mut connection = connection.lock().await;

    loop {
        let authorization = Authorization::new_internal();
        let identity = authorization
            .encode(settings)
            .expect("Failed to encode the internal identity.");
        let _: () = connection
            .set("INTERNAL_IDENTITY", &identity)
            .await
            .unwrap();
        log::debug!(
            "Successfully altered the internal identity to {}.",
            identity
        );
        sleep(delay).await;
    }
}
