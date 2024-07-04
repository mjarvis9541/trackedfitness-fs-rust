use std::collections::HashMap;

use leptos::{RwSignal, ServerFnError, SignalWith};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ValidationError {
    field_errors: HashMap<String, Vec<String>>,
}

impl ValidationError {
    pub fn new() -> Self {
        ValidationError {
            field_errors: HashMap::new(),
        }
    }

    pub fn add_error(&mut self, field: &str, message: String) {
        self.field_errors
            .entry(field.to_string())
            .or_insert_with(Vec::new)
            .push(message);
    }

    pub fn get_errors(&self, field: &str) -> Option<&Vec<String>> {
        self.field_errors.get(field)
    }

    pub fn add_non_field_error(&mut self, message: impl Into<String>) {
        self.field_errors
            .entry("non_field_errors".to_string())
            .or_insert_with(Vec::new)
            .push(message.into());
    }

    pub fn get_non_field_errors(&self) -> Option<&Vec<String>> {
        self.field_errors.get("non_field_errors")
    }

    pub fn to_json_string(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }

    pub fn is_empty(&self) -> bool {
        self.field_errors.is_empty()
    }
}

impl From<ValidationError> for ServerFnError {
    fn from(error: ValidationError) -> Self {
        #[cfg(feature = "ssr")]
        leptos::use_context::<leptos_axum::ResponseOptions>()
            .map(|res| res.set_status(http::status::StatusCode::BAD_REQUEST));

        ServerFnError::new(error.to_json_string())
    }
}

pub fn get_field_errors<T>(
    action_value: RwSignal<Option<Result<T, ServerFnError>>>,
    field: &str,
) -> Option<Vec<String>> {
    action_value.with(|option| {
        option
            .as_ref()
            .and_then(|result| result.as_ref().err())
            .and_then(|err| {
                if let ServerFnError::ServerError(err_str) = err {
                    serde_json::from_str::<ValidationError>(err_str)
                        .ok()
                        .and_then(|val_err| val_err.get_errors(field).cloned())
                } else {
                    None
                }
            })
    })
}

pub fn get_non_field_errors<T>(
    action_value: RwSignal<Option<Result<T, ServerFnError>>>,
) -> Option<String> {
    action_value.with(|option| {
        option
            .as_ref()
            .and_then(|result| result.as_ref().err())
            .and_then(|err| {
                if let ServerFnError::ServerError(err_str) = err {
                    serde_json::from_str::<ValidationError>(err_str)
                        .ok()
                        .and_then(|val_err| val_err.get_non_field_errors().cloned())
                        .map(|errors| errors.join(", "))
                } else {
                    None
                }
            })
    })
}

pub fn extract_other_errors<T>(
    action_value: RwSignal<Option<Result<T, ServerFnError>>>,
    known_field_errors: &[&str],
) -> Option<String> {
    action_value.with(|option| {
        option.as_ref().and_then(|result| {
            if let Err(ServerFnError::ServerError(err_str)) = result {
                if let Ok(val_err) = serde_json::from_str::<ValidationError>(err_str) {
                    for field in known_field_errors {
                        if val_err.field_errors.contains_key(*field) {
                            return None;
                        }
                    }
                }
                Some(err_str.clone())
            } else {
                result.as_ref().err().map(|err| err.to_string())
            }
        })
    })
}
