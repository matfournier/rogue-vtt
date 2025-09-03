use axum::{
    extract::{Path, Request},
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use sqlx::types::Uuid;
use std::str::FromStr;
use tower_cookies::{Cookies, Key, Cookie};

#[derive(Debug, Clone)]
pub struct RoomAuth {
    pub room_id: Uuid,
}

const COOKIE_NAME: &str = "room_session";

pub async fn room_auth_middleware(
    cookies: Cookies,
    Path(room_id): Path<Uuid>,
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // Get the room session cookie
    let session_cookie = cookies
        .get(COOKIE_NAME)
        .ok_or(StatusCode::UNAUTHORIZED)?;

    // Parse the room_id from the cookie value
    let cookie_room_id = Uuid::from_str(session_cookie.value())
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    // Verify the cookie room_id matches the path room_id
    if cookie_room_id != room_id {
        return Err(StatusCode::UNAUTHORIZED);
    }

    // Add room auth to request extensions
    request.extensions_mut().insert(RoomAuth { room_id });

    Ok(next.run(request).await)
}

pub fn create_room_session_cookie(room_id: Uuid) -> Cookie<'static> {
    Cookie::build((COOKIE_NAME, room_id.to_string()))
        .http_only(true)
        .same_site(tower_cookies::cookie::SameSite::Lax) // Changed from Strict to Lax for redirects
        .secure(false) // Set to true in production with reverse proxy detection
        .max_age(tower_cookies::cookie::time::Duration::hours(2))
        .path("/") // Explicitly set path to root
        .build()
}

pub fn validate_room_session(cookies: &Cookies, key: &Key, expected_room_id: Uuid) -> Result<(), &'static str> {
    // Get signed cookie
    let cookie = cookies.signed(key)
        .get(COOKIE_NAME)
        .ok_or("No valid session")?;
    
    // Parse room_id from cookie
    let cookie_room_id = Uuid::from_str(cookie.value())
        .map_err(|_| "Invalid session format")?;
    
    // Verify room_id matches expected
    if cookie_room_id != expected_room_id {
        return Err("Access denied");
    }
    
    Ok(())
}