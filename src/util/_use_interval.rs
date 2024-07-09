// use std::time::Duration;

// use leptos::leptos_dom::helpers::IntervalHandle;
// use leptos::*;

// pub fn use_interval<T, F>(interval_millis: T, f: F)
// where
//     F: Fn() + Clone + 'static,
//     T: Into<MaybeSignal<u64>> + 'static,
// {
//     let interval_millis = interval_millis.into();

//     create_effect(move |prev_handle: Option<IntervalHandle>| {
//         // effects get their previous return value as an argument
//         // each time the effect runs, it will return the interval handle
//         // so if we have a previous one, we cancel it
//         if let Some(prev_handle) = prev_handle {
//             prev_handle.clear();
//         };

//         // here, we return the handle
//         set_interval_with_handle(
//             f.clone(),
//             // this is the only reactive access, so this effect will only
//             // re-run when the interval changes
//             Duration::from_millis(interval_millis.get()),
//         )
//         .expect("could not create interval")
//     });
// }
