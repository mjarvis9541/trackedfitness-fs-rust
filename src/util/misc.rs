use serde::{Deserialize, Serialize};
// use wasm_bindgen::JsCast;
// use web_sys::{Event, TouchEvent};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ListResponse<T> {
    pub count: i64,
    pub results: Vec<T>,
}

// pub fn get_touch_coordinates(event: &Event) -> Option<(f64, f64)> {
//     if let Some(touch_event) = event.dyn_ref::<TouchEvent>() {
//         if let Some(touch) = touch_event.touches().get(0) {
//             return Some((touch.client_x() as f64, touch.client_y() as f64));
//         }
//     }
//     None
// }

// pub fn extract_response_error<T, E: std::fmt::Display>(
//     option_result: Option<Result<T, ServerFnError<E>>>,
// ) -> Option<String> {
//     if let Some(Err(error)) = option_result {
//         match error {
//             ServerFnError::WrappedServerError(custom_err) => Some(custom_err.to_string()),
//             ServerFnError::Registration(msg)
//             | ServerFnError::Request(msg)
//             | ServerFnError::Response(msg)
//             | ServerFnError::ServerError(msg)
//             | ServerFnError::Deserialization(msg)
//             | ServerFnError::Serialization(msg)
//             | ServerFnError::Args(msg)
//             | ServerFnError::MissingArg(msg) => Some(msg),
//         }
//     } else {
//         None
//     }
// }
