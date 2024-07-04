use leptos::*;

use http::{header, HeaderMap, HeaderValue};

use crate::config::get_config;
use crate::error::{Error, Result};

pub fn get_cookie(headers: &HeaderMap, name: &str) -> Option<String> {
    headers.get(header::COOKIE).and_then(|cookie_header| {
        cookie_header.to_str().ok().and_then(|cookie_str| {
            cookie_str
                .split(';')
                .map(|cookie| cookie.trim())
                .find(|cookie| cookie.starts_with(name))
                .and_then(|cookie| {
                    let parts: Vec<&str> = cookie.splitn(2, '=').collect();
                    if parts.len() == 2 {
                        Some(parts[1].to_string())
                    } else {
                        None
                    }
                })
        })
    })
}

pub fn set_cookie(name: &str, value: &str) -> Result<()> {
    let response = use_context::<leptos_axum::ResponseOptions>().ok_or(Error::InternalServer)?;
    let cookie = format!("{name}={value}; SameSite=Lax; path=/; HttpOnly");
    let header_value = HeaderValue::try_from(cookie)?;
    response.insert_header(header::SET_COOKIE, header_value);
    Ok(())
}

pub fn delete_cookie(name: &str) -> Result<()> {
    let response = use_context::<leptos_axum::ResponseOptions>().ok_or(Error::InternalServer)?;
    let cookie = format!("{name}=; SameSite=Lax; path=/; expires=Thu, 01 Jan 1970 00:00:00 GMT");
    let header_value = HeaderValue::try_from(cookie)?;
    response.insert_header(header::SET_COOKIE, header_value);
    Ok(())
}

// pub fn get_jwt_cookie(headers: &HeaderMap) -> Result<Option<String>> {
//     let config = get_config();
//     let cookie = get_cookie(headers, &config.auth_cookie_name);
//     Ok(cookie)
// }

pub fn set_jwt_cookie(value: &str) -> Result<()> {
    let config = get_config();
    set_cookie(&config.auth_cookie_name, value)?;
    Ok(())
}

pub fn delete_jwt_cookie() -> Result<()> {
    let config = get_config();
    delete_cookie(&config.auth_cookie_name)?;
    Ok(())
}
