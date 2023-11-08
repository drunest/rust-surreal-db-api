use actix_session::{config::PersistentSession, storage::CookieSessionStore, SessionMiddleware};
use actix_web::cookie::{time::Duration as CookieDuration, Key};

pub fn make_session() -> SessionMiddleware<CookieSessionStore> {
    let SECRET = std::env::var("SECRET").unwrap();
    let session =
        SessionMiddleware::builder(CookieSessionStore::default(), Key::from(SECRET.as_bytes()))
            .session_lifecycle(PersistentSession::default().session_ttl(CookieDuration::days(1)))
            .cookie_name("auth".to_owned())
            .cookie_secure(false)
            .cookie_http_only(true)
            .cookie_path("/".to_owned())
            .cookie_same_site(actix_web::cookie::SameSite::None)
            .build();
    session
}
