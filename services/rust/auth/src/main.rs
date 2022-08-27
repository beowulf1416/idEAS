extern crate log;

mod classes;
mod middleware;
mod services;
mod endpoints;


use log::{ 
    info,
    debug,
    error
};

use serde::{
    Serialize,
    Deserialize
};

use actix_web::{ 
    HttpServer, 
    App, 
    web
};

use config::{
    ApplicationConfig,
    get_configuration
};



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    info!("starting up");


    if let Some(config) = get_configuration() {
        debug!("parsed config: {:?}", config);

        let bind_host = config.bind_host.clone();
        let bind_port = config.bind_port.clone();

        let server = HttpServer::new(move || {
            App::new()
                .app_data(web::Data::new(config.clone()))
                .app_data(web::Data::new(crate::services::data::Data::new(&config.clone())))

                .service(web::scope("/status").configure(crate::endpoints::status::config))
                .service(web::scope("/auth").configure(crate::endpoints::auth::config))
        })
        .bind(format!("{}:{}", bind_host, bind_port))?
        .run();

        info!("Server is listening at https://{}:{}", bind_host, bind_port);
        return server.await;
    } else {
        error!("unable to retrieve configuration");
    }

    info!("Exiting...");
    return Ok(());
}