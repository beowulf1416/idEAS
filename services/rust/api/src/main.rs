extern crate log;

mod classes;
mod services;
mod middleware;
mod endpoints;


use log::{
    info,
    debug,
    error
};

use actix_web::{
    App,
    HttpServer,
    web
};

use config::{
    get_configuration
};

#[actix_web::main]
async fn main()  -> std::io::Result<()> {
    env_logger::init();
    info!("starting up");


    if let Some(config) = get_configuration() {
        debug!("parsed config: {:?}", config);

        let bind_host = config.auth.bind_host.clone();
        let bind_port = config.auth.bind_port.clone();

        let server = HttpServer::new(move || {
            let token = token::Token::new(&config.token.secret);

            App::new()
                .app_data(web::Data::new(config.clone()))
                .app_data(pg::Db::new(&config.clone()))

                // .app_data(web::Data::new(crate::services::auth_token::AuthToken::new(token.clone())))
                .app_data(web::Data::new(token.clone()))

                .wrap(crate::middleware::cors::CORS::new())
                .wrap(crate::middleware::user::User::new(token.clone()))

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
