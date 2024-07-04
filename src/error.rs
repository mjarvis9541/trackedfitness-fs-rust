use leptos::ServerFnError;

#[cfg(feature = "ssr")]
use {crate::auth::email_service::EmailError, http::status::StatusCode, leptos::use_context};

use crate::util::validation_error::ValidationError;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Validation(ValidationError),
    Forbidden,
    NotFound,
    Unauthorized,
    InvalidCredentials,
    InternalServer,
    Email(String),
    Other(String),
    InvalidToken,
    FileUpload,
}

impl From<ValidationError> for Error {
    fn from(error: ValidationError) -> Self {
        Error::Validation(error)
    }
}

#[cfg(feature = "ssr")]
impl From<EmailError> for Error {
    fn from(error: EmailError) -> Self {
        Error::Email(error.to_string())
    }
}

#[cfg(feature = "ssr")]
impl From<bcrypt::BcryptError> for Error {
    fn from(error: bcrypt::BcryptError) -> Self {
        Error::Other(error.to_string())
    }
}

#[cfg(feature = "ssr")]
impl From<http::header::InvalidHeaderValue> for Error {
    fn from(error: http::header::InvalidHeaderValue) -> Self {
        Error::Other(error.to_string())
    }
}

#[cfg(feature = "ssr")]
impl From<jsonwebtoken::errors::Error> for Error {
    fn from(error: jsonwebtoken::errors::Error) -> Self {
        Error::Other(error.to_string())
    }
}

impl From<Error> for ServerFnError {
    fn from(error: Error) -> Self {
        dbg!(&error);

        match error {
            Error::Validation(val_error) => {
                #[cfg(feature = "ssr")]
                use_context::<leptos_axum::ResponseOptions>()
                    .map(|res| res.set_status(StatusCode::BAD_REQUEST));
                ServerFnError::ServerError(val_error.to_json_string())
            }
            Error::Unauthorized => {
                #[cfg(feature = "ssr")]
                use_context::<leptos_axum::ResponseOptions>()
                    .map(|res| res.set_status(StatusCode::UNAUTHORIZED));
                ServerFnError::new("Unauthorized")
            }
            Error::InvalidCredentials => {
                #[cfg(feature = "ssr")]
                use_context::<leptos_axum::ResponseOptions>()
                    .map(|res| res.set_status(StatusCode::UNAUTHORIZED));
                ServerFnError::new("Invalid credentials")
            }
            Error::Forbidden => {
                #[cfg(feature = "ssr")]
                use_context::<leptos_axum::ResponseOptions>()
                    .map(|res| res.set_status(StatusCode::FORBIDDEN));
                ServerFnError::new("You do not have permission to perform this action")
            }
            Error::NotFound => {
                #[cfg(feature = "ssr")]
                use_context::<leptos_axum::ResponseOptions>()
                    .map(|res| res.set_status(StatusCode::NOT_FOUND));
                ServerFnError::new("Not found")
            }
            Error::InternalServer => ServerFnError::new("Internal server error"),
            Error::Email(msg) => ServerFnError::ServerError(msg),
            Error::Other(msg) => ServerFnError::ServerError(msg),
            Error::InvalidToken => ServerFnError::new("Invalid token"),
            Error::FileUpload => ServerFnError::new("File upload error"),
        }
    }
}

#[cfg(feature = "ssr")]
impl From<sqlx::Error> for Error {
    fn from(err: sqlx::Error) -> Self {
        Error::Other(err.to_string())
    }
}

#[cfg(feature = "ssr")]
pub fn handle_sqlx_contraint_error(
    err: sqlx::Error,
    table: &str,
    field: &str,
    constraint_keys: &[&str],
) -> Error {
    if let sqlx::Error::Database(db_err) = &err {
        if let Some(constraint) = db_err.constraint() {
            let mut errors = ValidationError::new();
            if constraint_keys.iter().any(|key| constraint.contains(key)) {
                errors.add_error(
                    field,
                    format!("{} with this {} already exists.", table, field),
                );
            } else {
                errors.add_non_field_error("An unexpected error occurred.");
            }
            return Error::from(errors);
        }
    }
    Error::from(err)
}
