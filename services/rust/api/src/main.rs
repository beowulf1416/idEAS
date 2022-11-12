extern crate log;

mod classes;
mod extractors;
mod services;
mod guards;
mod middleware;
mod endpoints;


use log::{
    info,
    debug,
    error
};

use std::time::Duration;
use std::sync::{ Mutex };

use actix_web::{
    App,
    HttpServer,
    web
};

use kafka::producer::{
    Producer, 
    Record, 
    RequiredAcks
};

use config::{
    ProviderType,
    get_configuration
};

#[actix_web::main]
async fn main()  -> std::io::Result<()> {
    env_logger::init();
    info!("starting up");


    if let Some(config) = get_configuration() {
        debug!("parsed config: {:?}", config);

        let bind_host = config.api.bind_host.clone();
        let bind_port = config.api.bind_port.clone();

        let server = HttpServer::new(move || {
            let tokenizer = token::Token::new(&config.token.secret);

            let hosts: Vec<String> = config.providers.iter()
                .filter(|x| matches!(x.provider_type, ProviderType::Kafka) && x.name == "queue")
                .map(|r| r.url.clone())
                .flatten()
                .collect();

            let producer = kafka::producer::Producer::from_hosts(hosts)
                .with_ack_timeout(Duration::from_secs(1))
                .with_required_acks(RequiredAcks::One)
                .create()
                .unwrap();

            App::new()
                .app_data(web::Data::new(config.clone()))
                .app_data(web::Data::new(tokenizer.clone()))
                .app_data(web::Data::new(pg::Db::new(&config.clone())))
                .app_data(web::Data::new(Mutex::new(producer)))

                .wrap(crate::middleware::cors::CORS::new())
                .wrap(crate::middleware::user::User::new(tokenizer.clone()))

                .service(web::scope("/status").configure(crate::endpoints::status::config))
                
                .service(web::scope("/countries").configure(crate::endpoints::common::countries::config))
                .service(web::scope("/currencies").configure(crate::endpoints::common::currencies::config))

                .service(web::scope("/auth").configure(crate::endpoints::auth::config))
                .service(web::scope("/user").configure(crate::endpoints::user::config))
                
                .service(web::scope("/admin/users").configure(crate::endpoints::admin::users::config))
                .service(web::scope("/admin/user").configure(crate::endpoints::admin::user::config))

                .service(web::scope("/clients").configure(crate::endpoints::clients::config))
                .service(web::scope("/roles").configure(crate::endpoints::admin::roles::config))
                .service(web::scope("/permissions").configure(crate::endpoints::admin::permissions::config))

                .service(web::scope("/inventory/item").configure(crate::endpoints::inventory::item::item::config))
                .service(web::scope("/inventory/item/category").configure(crate::endpoints::inventory::item::category::config))
        })
        .workers(2) // for testing only
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
