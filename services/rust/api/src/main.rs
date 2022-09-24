extern crate log;

mod classes;
mod middleware;
mod endpoints;


use log::{
    info,
    debug,
    error
};

use actix_web::{
    App,
    HttpServer
};

use config::{
    get_configuration
};

#[actix_web::main]
async fn main()  -> std::io::Result<()> {
    env_logger::init();
    info!("starting up ...");


    if let Some(config) = get_configuration() {
        debug!("parsed config: {:?}", config);

        // let bind_host = config.bind_host.clone();
        // let bind_port = config.bind_port.clone();
        let bind_host = config.api.bind_host.clone();
        let bind_port = config.api.bind_port.clone();

        let server = HttpServer::new(move || {
            App::new()
                .app_data(web::Data::new(config.clone()))
                .app_data(pg::Db::new(&config.clone()))
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
