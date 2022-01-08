use std::{ffi::OsStr, fs, net::SocketAddr, str::FromStr};

use askama::Template;
use axum::{extract::Extension, routing::get, AddExtensionLayer, Router};
use itertools::Itertools;
use serde::Deserialize;
use tracing::Level;

use util::{Error, HtmlTemplate};

mod util;

#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
struct Config {
    mobi_dir: String,
    show_dir_num: usize,
    download_base_url: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            mobi_dir: "mobi".to_string(),
            show_dir_num: 10,
            download_base_url: "/".to_string(),
        }
    }
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    files: Vec<String>,
    base_url: String,
}

async fn index(Extension(config): Extension<Config>) -> Result<HtmlTemplate<IndexTemplate>, Error> {
    let files = fs::read_dir(config.mobi_dir)?
        .filter_ok(|entry| {
            entry.path().is_file()
                && (entry.path().extension() == Some(OsStr::new("azw3"))
                    || entry.path().extension() == Some(OsStr::new("mobi")))
        })
        .map_ok(|entry| entry.file_name().to_string_lossy().to_string())
        .take(config.show_dir_num)
        .collect::<Result<Vec<_>, _>>()?;
    return Ok(HtmlTemplate(IndexTemplate {
        files,
        base_url: config.download_base_url,
    }));
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();

    let config: Config =
        toml::from_str(&fs::read_to_string("config.toml").expect("config.toml not found"))
            .expect("error parsing config.toml");

    tracing::info!("config: {:?}", config);

    let app = Router::new()
        .route("/", get(index))
        .layer(AddExtensionLayer::new(config));

    let addr = SocketAddr::from_str("127.0.0.1:3000").unwrap();
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
