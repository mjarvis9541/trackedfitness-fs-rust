// use chrono::DateTime;
// use leptos::*;

use axum::body::Body;
use axum::extract::Request;
use axum::middleware::Next;
use axum::response::Response;
// use http::header;
// use http::status::StatusCode;

use crate::auth::cookie::get_cookie;
use crate::auth::model::RequestUser;
use crate::auth::token::JwtManager;
use crate::config::get_config;

pub async fn auth_token_middleware(mut req: Request<Body>, next: Next) -> Response {
    let path = req.uri().path();
    // tracing::info!("Running middleware: {}", path);
    // Skip middleware for static assets
    if path.starts_with("/pkg") || path.starts_with("/favicon.ico") || path.starts_with("/images") {
        return next.run(req).await;
    }

    let config = get_config();
    let headers = req.headers();
    if let Some(auth_cookie) = get_cookie(headers, &config.auth_cookie_name) {
        // tracing::info!("auth cookie found: {}", auth_cookie);
        if let Ok(auth_token) = JwtManager::validate_auth_token(&auth_cookie) {
            let user = RequestUser::from(auth_token);
            // tracing::info!("User added to request extension: {:?}", user);
            req.extensions_mut().insert(user);
        }
    }
    next.run(req).await
}

// pub async fn auth_token_middleware(mut req: Request<Body>, next: Next) -> Response {
//     let path = req.uri().path();
//     tracing::info!("Running middleware: {}", path);

//     // Skip middleware for static assets
//     if path.starts_with("/pkg") || path.starts_with("/favicon.ico") || path.starts_with("/images") {
//         return next.run(req).await;
//     }

//     let config = get_config();
//     let headers = req.headers();

//     match get_cookie(headers, &config.auth_cookie_name) {
//         Some(auth_cookie) => {
//             tracing::info!("auth cookie found: {}", auth_cookie);
//             match JwtManager::validate_auth_token(&auth_cookie) {
//                 Ok(auth_token) => {
//                     let naive = DateTime::from_timestamp(auth_token.exp, 0);
//                     tracing::info!("Time now: {:?}\n", chrono::Utc::now());
//                     tracing::info!("Expires: {:?}\n\n", naive);
//                     let user = RequestUser::from(auth_token);
//                     tracing::info!("User added to request extension: {:?}", user);
//                     req.extensions_mut().insert(user);
//                     return next.run(req).await;
//                 }
//                 Err(err) => {
//                     tracing::error!("Token found, but invalid {:?}", err);
//                     let cookie = format!(
//                         "{}=; SameSite=Lax; path=/; expires=Thu, 01 Jan 1970 00:00:00 GMT",
//                         &config.auth_cookie_name
//                     );
//                     if path.starts_with("/login") {
//                         return next.run(req).await;
//                     }
//                     return Response::builder()
//                         .status(StatusCode::FOUND)
//                         .header(header::LOCATION, "/login")
//                         .header(header::SET_COOKIE, cookie)
//                         .body(Body::empty())
//                         .unwrap();
//                 }
//             }
//         }
//         None => {
//             if path.starts_with("/login")
//                 || path.starts_with("/api/authenticate")
//                 || path.starts_with("/api/login")
//             {
//                 return next.run(req).await;
//             } else {
//                 return Response::builder()
//                     .status(StatusCode::FOUND)
//                     .header(header::LOCATION, "/login")
//                     .body(Body::empty())
//                     .unwrap();
//             }
//         }
//     }

//     // next.run(req).await
// }

// async fn redirect(req: Request<Body>, next: Next) -> Response {
//     let path = req.uri().path();

//     if path.starts_with("/settings") || path.starts_with("/editor") {
//         // authenticated routes
//         Response::builder()
//             .status(StatusCode::FOUND)
//             .header(header::LOCATION, "/login")
//             .header(header::SET_COOKIE, REMOVE_COOKIE)
//             .body(axum::body::Body::empty())
//             .unwrap()
//     } else {
//         next.run(req).await
//     }
// }
