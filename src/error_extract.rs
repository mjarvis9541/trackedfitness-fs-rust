use leptos::*;
use serde_json::{from_str, Value};

pub fn extract_error_message<I, O>(action: &Action<I, Result<O, ServerFnError>>) -> Option<String>
where
    I: 'static,
    O: 'static,
{
    action.value().with(|option| {
        option
            .as_ref()
            .and_then(|result| result.as_ref().err().map(|err| err.to_string()))
    })
}

pub fn extract_field_errors(json_str: &str, key: &str) -> Option<Vec<String>> {
    let json_part = json_str.find('{').and_then(|index| json_str.get(index..))?;
    let v: Value = from_str(json_part).ok()?;
    v.get(key)?
        .as_array()?
        .iter()
        .map(|value| value.as_str().map(|s| s.to_string()))
        .collect::<Option<Vec<String>>>()
}

pub fn process_non_field_errors<F>(error: F) -> Option<View>
where
    F: Fn() -> Option<String>,
{
    error().and_then(|err| {
        extract_field_errors(&err, "non_field_errors").map(|err| {
            err.iter()
                .map(|err| view! { <div class="my-4 font-bold text-red-500">{err.to_string()}</div> })
                .collect_view()
        })
    })
}
