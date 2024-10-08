use std::fs::File;
use std::io::BufReader;

use actix_cors::Cors;
use actix_web::web::Data;
use actix_web::{App, HttpServer};
use dotenvy::dotenv;
use log::debug;

use self::http::v1;
use crate::config::app::AppState;
use crate::http::v0;

mod config;
mod http;
mod models;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  dotenv().ok();
  colog::init();
  let app_data = AppState::new().await;

  let addr = (app_data.config.app.url.clone(), app_data.config.app.port);
  let tls_enabled = app_data.config.tls.enabled;
  let max_workers = app_data.config.http.workers;

  debug!("Web Server Online!");

  let tls_config = if tls_enabled {
    // TLS setup.
    rustls::crypto::aws_lc_rs::default_provider()
      .install_default()
      .unwrap();

    let mut certs_file = BufReader::new(File::open(app_data.config.tls.cert.clone()).unwrap());
    let mut key_file = BufReader::new(File::open(app_data.config.tls.key.clone()).unwrap());

    // load TLS certs and key
    // to create a self-signed temporary cert for testing:
    // `openssl req -x509 -newkey rsa:4096 -nodes -keyout key.pem -out cert.pem -days 365 -subj '/CN=localhost'`
    let tls_certs = rustls_pemfile::certs(&mut certs_file)
      .collect::<Result<Vec<_>, _>>()
      .unwrap();
    let tls_key = rustls_pemfile::pkcs8_private_keys(&mut key_file)
      .next()
      .unwrap()
      .unwrap();

    // set up TLS config options
    Some(
      rustls::ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(tls_certs, rustls::pki_types::PrivateKeyDer::Pkcs8(tls_key))
        .unwrap(),
    )
  } else {
    None
  };

  let server = HttpServer::new(move || {
    let cors = Cors::default()
      .allow_any_origin()
      .allow_any_method()
      .allow_any_header()
      .max_age(3600);

    App::new()
      .wrap(cors)
      .app_data(Data::new(app_data.clone()))
      .service(actix_files::Files::new("/static", "./static").use_last_modified(true))
      .service(http::welcome)
      .service(v0::settings_controller::put_settings)
      .service(v0::settings_controller::get_settings)
      .service(v1::settings_controller::put_settings)
      .service(v1::settings_controller::get_settings)
      .service(v1::metrics_controller::post_heartbeat)
      .service(v1::metrics_controller::get_user_metrics)
      .service(v1::auth_controller::post_user_authentication)
  })
  .workers(max_workers);

  match tls_enabled {
    true => {
      server
        .bind_rustls_0_23(addr, tls_config.unwrap())?
        .run()
        .await
    }
    false => server.bind(addr)?.run().await,
  }
}
