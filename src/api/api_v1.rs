use actix_session::SessionMiddleware;
use actix_session::storage::CookieSessionStore;
use actix_web::cookie::Key;
use actix_web::dev::HttpServiceFactory;
use actix_web::web;
use crate::api::whoami::whoami;
use crate::settings::SETTINGS;

pub fn api_v1_service() -> impl HttpServiceFactory {
    web::scope("/api/v1")
        .wrap(SessionMiddleware::new(
            CookieSessionStore::default(),
            Key::from(&SETTINGS.auth.auth_cookie_key),
        ))
        .service(web::resource("/whoami").route(web::get().to(whoami)))
}