use leptos::*;

#[component]
pub fn Line(
    #[prop(into, optional)] x1: MaybeSignal<f64>,
    #[prop(into, optional)] y1: MaybeSignal<f64>,
    #[prop(into, optional)] x2: MaybeSignal<f64>,
    #[prop(into, optional)] y2: MaybeSignal<f64>,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
) -> impl IntoView {
    view! { <line {..attrs} x1=x1 y1=y1 x2=x2 y2=y2 class="stroke-gray-400 stroke-dashed"></line> }
}

#[component]
pub fn Circle(
    #[prop(into)] cx: MaybeSignal<f64>,
    #[prop(into)] cy: MaybeSignal<f64>,
    #[prop(into)] r: MaybeSignal<f64>,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
) -> impl IntoView {
    view! { <circle {..attrs} cx=cx cy=cy r=r class="fill-red-500"></circle> }
}

#[component]
pub fn Polyline(
    #[prop(into)] points: MaybeSignal<String>,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
) -> impl IntoView {
    view! { <polyline {..attrs} points=points></polyline> }
}

#[component]
pub fn Text(
    #[prop(into)] text: MaybeSignal<String>,
    #[prop(into, optional)] x: MaybeSignal<f64>,
    #[prop(into, optional)] y: MaybeSignal<f64>,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
) -> impl IntoView {
    view! {
        <text {..attrs} x=x y=y class="text-sm stroke-gray-800">
            {text}
        </text>
    }
}
