extern crate log;

// use std::env;
// use rustls:: {
//     Certificate,
//     PrivateKey,
//     ServerConfig
// };
// use rustls_pemfile::{
//     certs,
//     // pkcs8_private_keys,
//     rsa_private_keys
// };


mod data;
mod models;
mod utils;
mod middleware;
mod guards;
mod endpoints;



use log::{ info  };
// use std::{fs::File, io::BufReader};

use actix_web::{ HttpServer, App, web, HttpResponse, Responder };





async fn status() -> impl Responder {
    info!("status()");
    HttpResponse::Ok().body("Status")
}


// fn load_tls_config() -> rustls::ServerConfig {
//     let config = ServerConfig::builder()
//         .with_safe_defaults()
//         .with_no_client_auth()
//     ;

//     let cert_file = env::var("CERT_FILE").unwrap();
//     let key_file = env::var("KEY_FILE").unwrap();

//     info!("using certificate: {}", cert_file);
//     info!("using key: {}", key_file);

//     let cert_file_buf = &mut BufReader::new(File::open(cert_file).unwrap());
//     let key_file_buf = &mut BufReader::new(File::open(key_file).unwrap());

//     let cert_chain = certs(cert_file_buf)
//         .unwrap()
//         .into_iter()
//         .map(Certificate)
//         .collect()
//     ;
//     let mut keys: Vec<PrivateKey> = rsa_private_keys(key_file_buf)
//         .unwrap()
//         .into_iter()
//         .map(PrivateKey)
//         .collect()
//     ;

//     if keys.is_empty() {
//         error!("could not initialize secure server");
//         std::process::exit(1);
//     }

//     return config.with_single_cert(cert_chain, keys.remove(0)).unwrap(); 
// }


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    info!("starting up ...");

    // let config = load_tls_config();

    let server = HttpServer::new(move || {
        App::new()
            .configure(crate::data::db::configure)
            .configure(users::jwt::configure)
            .configure(crate::utils::bus::configure)
            .wrap(crate::middleware::cors::CORS::new())
            .wrap(crate::middleware::user::User::new())
            .service(web::scope("/status").configure(crate::endpoints::status::config))
            .service(web::scope("/user").configure(crate::endpoints::user::config))
            // .service(web::scope("/tenant").configure(crate::endpoints::tenant::admin::config))
            .route("/status", web::get().to(status))
    })
    .bind("0.0.0.0:8081")?
    // .bind_rustls("0.0.0.0:8081", config)?
    .run();

    info!("Server running at https://0.0.0.0:8081");
    server.await
}
