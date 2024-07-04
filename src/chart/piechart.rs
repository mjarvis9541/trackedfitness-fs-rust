use leptos::*;

use rust_decimal::Decimal;

use crate::util::text::humanize_decimal;

// Calories: Yellow - Often associated with energy and vibrancy, reflecting the energy provided by calories.
// Proteins: Blue - Represents strength and stability, as proteins are crucial for muscle building and repair.
// Carbohydrates: Green - Linked to plant-based foods which are rich in carbs, symbolizing health and vitality.
// Fat: Orange - Reflects richness and fullness, indicative of the energy density and satiating nature of fats.

#[derive(Debug, Default)]
pub enum PieChartVariant {
    #[default]
    Energy,
    Protein,
    Carbs,
    Fat,
}

impl PieChartVariant {
    pub fn into_css(&self) -> &'static str {
        use PieChartVariant::*;
        match self {
            Energy => "stroke-amber-400",
            Protein => "stroke-blue-600",
            Carbs => "stroke-emerald-400",
            Fat => "stroke-orange-400",
        }
    }
}

#[component]
pub fn PieChart(
    #[prop(into)] title: MaybeSignal<String>,
    #[prop(into)] total: MaybeSignal<Decimal>,
    #[prop(into)] target: MaybeSignal<Decimal>,
    #[prop(into)] metric: MaybeSignal<String>,
    #[prop(optional)] variant: PieChartVariant,
) -> impl IntoView {
    let total = total.get();
    let target = target.get();
    let metric = metric.get();

    let target_title = format!("Target {:.0}{}", target, metric);
    let remaining_title = format!("Remaining {:.0}{}", target - total, metric);

    let total_str = format!("{}{}", humanize_decimal(&total), metric);
    let target_str = format!("{}{}", humanize_decimal(&target), metric);

    let percentage_consumed = if target.is_zero() {
        Decimal::ZERO
    } else {
        (total / target) * Decimal::from(100)
    };

    let stroke_dasharray1 = percentage_consumed;
    let stroke_dasharray2 = Decimal::new(100, 0) - percentage_consumed;
    let dasharray = format!("{:.2} {:.2}", stroke_dasharray1, stroke_dasharray2);

    view! {
        <div class="p-2 border">
            <h1 class="text-lg font-bold">{title}</h1>
            <p class="text-sm text-gray-600">{target_title}</p>
            <div class="flex relative justify-center items-center w-48 h-48">

                <svg width="100%" height="100%" viewBox="0 0 42 42">

                    <circle cx="21" cy="21" r="15.91549431" fill="#fff"></circle>
                    <circle
                        class="stroke-gray-200"
                        cx="21"
                        cy="21"
                        r="15.91549431"
                        fill="transparent"
                        stroke-width="4"
                    ></circle>
                    <circle
                        class=variant.into_css()
                        cx="21"
                        cy="21"
                        r="15.91549431"
                        fill="transparent"
                        stroke="#00aaff"
                        stroke-width="4"
                        stroke-dasharray=dasharray
                        stroke-dashoffset="25"
                    ></circle>
                </svg>
                <div class="absolute text-center">
                    <div class="text-xl font-bold text-gray-700">{total_str}</div>
                    <div class="text-gray-600">{target_str}</div>
                </div>
            </div>
            <p class="text-sm text-gray-600">{remaining_title}</p>
        </div>
    }
}
