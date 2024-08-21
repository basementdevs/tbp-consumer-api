pub mod app;

use serde::{Deserialize, Serialize};
use std::thread;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct App {
  pub name: String,
  pub version: String,
  pub url: String,
  pub port: u16,
  pub platform_secret: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Database {
  pub nodes: Vec<String>,
  pub username: String,
  pub password: String,
  pub cached_queries: usize,
  pub keyspace: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Tls {
  pub enabled: bool,
  pub cert: String,
  pub key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Http {
  pub workers: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
  pub app: App,
  pub tls: Tls,
  pub database: Database,
  pub http: Http,
}

impl Config {
  pub fn new() -> Self {
    Config {
      app: App {
        name: dotenvy::var("APP_NAME").unwrap(),
        version: dotenvy::var("APP_VERSION").unwrap(),
        url: dotenvy::var("APP_URL").unwrap(),
        port: dotenvy::var("APP_PORT").unwrap().parse::<u16>().unwrap(),
        platform_secret: dotenvy::var("APP_PLATFORM_SECRET")
          .unwrap()
          .parse::<String>()
          .unwrap(),
      },
      tls: Tls {
        enabled: dotenvy::var("APP_TLS_ENABLED").unwrap() == "true",
        cert: dotenvy::var("APP_TLS_CERT").unwrap(),
        key: dotenvy::var("APP_TLS_KEY").unwrap(),
      },
      http: Http {
        workers: dotenvy::var("MAX_WORKERS")
          .ok()
          .and_then(|s| s.parse::<usize>().ok())
          .unwrap_or_else(|| {
            thread::available_parallelism()
              .map(|n| n.get())
              .unwrap_or(1)
          }),
      },
      database: Database {
        nodes: dotenvy::var("SCYLLA_NODES")
          .unwrap()
          .split(',')
          .map(|s| s.to_string())
          .collect(),
        username: dotenvy::var("SCYLLA_USERNAME").unwrap(),
        password: dotenvy::var("SCYLLA_PASSWORD").unwrap(),
        cached_queries: dotenvy::var("SCYLLA_CACHED_QUERIES")
          .unwrap()
          .parse::<usize>()
          .unwrap(),
        keyspace: dotenvy::var("SCYLLA_KEYSPACE").unwrap(),
      },
    }
  }
}

impl Default for Config {
  fn default() -> Self {
    Self::new()
  }
}
