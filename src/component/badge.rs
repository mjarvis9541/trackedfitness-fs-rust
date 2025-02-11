use leptos::*;
use rust_decimal::Decimal;

#[derive(Debug)]
pub enum BadgeVariant {
    Primary,
}

impl BadgeVariant {
    pub fn into_css(&self) -> &'static str {
        match self {
            Self::Primary => {
                "select-none whitespace-nowrap rounded bg-gray-700 px-2 py-1 text-xs text-gray-200"
            }
        }
    }
}

#[component]
pub fn Badge(
    #[prop(optional, into)] label: MaybeSignal<String>,
    #[prop(optional, into)] value: MaybeSignal<i64>,
    #[prop(default = BadgeVariant::Primary)] variant: BadgeVariant,
) -> impl IntoView {
    view! { <div class=variant.into_css()>{value} " " {label}</div> }
}

#[component]
pub fn Badgei64(title: &'static str, value: i64) -> impl IntoView {
    let has_value = value > 0;
    view! {
        <div class="flex-1 text-end">
            <div class=("font-bold", move || has_value)>{value}</div>
            <div class="text-xs text-gray-500">{title}</div>
        </div>
    }
}

#[component]
pub fn BadgeDiet(
    title: &'static str,
    label: &'static str,
    value: Decimal,
    scale: usize,
) -> impl IntoView {
    let has_value = move || value > Decimal::from(0);
    view! {
        <div class="flex-1 text-end">
            <div class=("font-bold", has_value)>{format!("{:.*}", scale, value)} {label}</div>
            <div class="text-xs text-gray-500">{title}</div>
        </div>
    }
}

#[component]
pub fn BadgeProgress(
    title: &'static str,
    label: &'static str,
    value: Decimal,
    scale: usize,
) -> impl IntoView {
    let has_value = move || value > Decimal::from(0);
    view! {
        <div class="flex flex-1 justify-between items-center">
            <div class="text-xs text-gray-500">{title}</div>
            <div class=("font-bold", has_value)>{format!("{:.*}", scale, value)} {label}</div>
        </div>
    }
}

#[component]
pub fn MonthSummaryMetic(
    #[prop(optional, into)] title: &'static str,
    #[prop(optional, into)] label: &'static str,
    #[prop(optional, into)] value: Decimal,
    #[prop(optional, into)] scale: usize,
) -> impl IntoView {
    let has_value = move || value > Decimal::from(0);
    view! {
        <div class="flex flex-1 justify-between items-center">
            <div class="text-xs text-gray-500">{title}</div>
            <div class=("font-bold", has_value)>{format!("{:.*}", scale, value)} {label}</div>
        </div>
    }
}
