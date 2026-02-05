use dioxus::logger::tracing;
use dioxus::prelude::*;

#[cfg(feature = "server")]
#[derive(serde::Deserialize)]
struct DatabaseConfig {
    postgres: PostgresConfig,
}

#[cfg(feature = "server")]
#[derive(serde::Deserialize)]
struct PostgresConfig {
    host: String,
    #[serde(default = "default_port")]
    port: u16,
    user: String,
    password: String,
    dbname: String,
}

#[cfg(feature = "server")]
fn default_port() -> u16 {
    5432
}

#[cfg(feature = "server")]
fn load_config() -> DatabaseConfig {
    let cwd = std::env::current_dir().unwrap_or_else(|e| panic!("Failed to get current working directory: {e}"));
    let config_path = cwd.join("config.toml");

    let config_contents = std::fs::read_to_string(&config_path)
        .unwrap_or_else(|e| panic!("Failed to read config file at {:?}: {}", config_path, e));

    toml::from_str(&config_contents).unwrap_or_else(|e| panic!("Failed to parse config file: {}", e))
}

#[server]
pub async fn save_dog(image: String) -> Result<()> {
    tracing::info!("hotdog backend: saving dog image: {image}");
    let db = get_db().await;
    db.execute("INSERT INTO dogs (url) VALUES ($1)", &[&image])
        .await
        .map_err(|e| {
            error!("DB insert error: {e}");
            e
        })?;
    Ok(())
}

#[server]
pub async fn list_dogs() -> Result<Vec<(i32, String)>, ServerFnError> {
    tracing::info!("hotdog backend: listing dogs");
    let db = get_db().await;
    let rows = db
        .query("SELECT id, url FROM dogs ORDER BY id DESC LIMIT 10", &[])
        .await
        .context("failed to query dogs")?;

    let dogs = rows
        .into_iter()
        .map(|row| Ok::<(i32, String), tokio_postgres::Error>((row.get("id"), row.get("url"))))
        .collect::<Result<Vec<_>, _>>()
        .context("failed to read dogs rows")?;

    Ok(dogs)
}

// The database is only available to server code
#[cfg(feature = "server")]
use tokio_postgres::NoTls;

#[cfg(feature = "server")]
use tokio::sync::OnceCell;

#[cfg(feature = "server")]
static DB: OnceCell<tokio_postgres::Client> = OnceCell::const_new();

#[cfg(feature = "server")]
async fn init_db() -> tokio_postgres::Client {
    let config = load_config();
    let pg = &config.postgres;

    let conn_str = format!(
        "host={} port={} user={} password={} dbname={}",
        pg.host, pg.port, pg.user, pg.password, pg.dbname
    );

    tracing::info!(
        "hotdog backend: connecting to PostgreSQL database at {}:{}",
        pg.host,
        pg.port
    );

    // Connect to PostgreSQL
    let (client, connection) = tokio_postgres::connect(&conn_str, NoTls)
        .await
        .unwrap_or_else(|e| panic!("Failed to connect to PostgreSQL: {e}"));

    // Spawn the connection handler
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            error!("PostgreSQL connection error: {e}");
        }
    });

    // Create the "dogs" table if it doesn't already exist
    client
        .execute(
            "CREATE TABLE IF NOT EXISTS dogs (
                id SERIAL PRIMARY KEY,
                url TEXT NOT NULL
            )",
            &[],
        )
        .await
        .unwrap_or_else(|e| panic!("Failed to create 'dogs' table: {e}"));

    tracing::info!("hotdog backend: database connection established");

    client
}

#[cfg(feature = "server")]
async fn get_db() -> &'static tokio_postgres::Client {
    DB.get_or_init(init_db).await
}
