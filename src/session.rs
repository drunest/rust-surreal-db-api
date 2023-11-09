use actix_session::{config::PersistentSession, storage::CookieSessionStore, SessionMiddleware};
use actix_web::cookie::{time::Duration as CookieDuration, Key};

use crate::app_config;

pub fn make_session(app_config: &app_config::AppConfig) -> SessionMiddleware<CookieSessionStore> {
    let session = SessionMiddleware::builder(
        CookieSessionStore::default(),
        Key::from(app_config.session_secret.as_bytes()),
    )
    .session_lifecycle(PersistentSession::default().session_ttl(CookieDuration::days(15)))
    .cookie_name("auth".to_owned())
    .cookie_domain(None)
    .cookie_secure(true)
    .cookie_http_only(true)
    .cookie_path("/".to_owned())
    .cookie_same_site(actix_web::cookie::SameSite::None)
    .build();
    session
}
