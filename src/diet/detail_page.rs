use leptos::*;
use leptos_router::*;

use uuid::Uuid;

use crate::component::template::{
    DetailPageTemplate, ErrorComponent, LoadingSpinner, UpdateDeleteButtonRow,
};
use crate::food::model::Nutrition;
use crate::util::param::UuidParam;

use super::component::NutritionInformation;
use super::model::DietFoodQuery;

#[cfg(feature = "ssr")]
use crate::{auth::model::User, auth::service::get_request_user, error::Error, setup::get_pool};

#[server]
pub async fn get_diet_detail(id: Uuid) -> Result<DietFoodQuery, ServerFnError> {
    let user = get_request_user()?;
    let pool = get_pool()?;
    let object = DietFoodQuery::get_by_id(&pool, id)
        .await?
        .ok_or(Error::NotFound)?;
    let target_user = User::get_by_id(&pool, object.user_id)
        .await?
        .ok_or(Error::NotFound)?;
    User::check_view_permission(&pool, &user, &target_user.username).await?;
    Ok(object)
}

#[component]
pub fn DietDetailPage() -> impl IntoView {
    let params = use_params::<UuidParam>();
    let id = move || params.with(|p| p.as_ref().map_or_else(|_| Uuid::default(), |p| p.id));
    let resource = Resource::new(id, get_diet_detail);
    let response =
        move || resource.and_then(|data| view! { <DietDetailPageComponent data=data.clone()/> });
    view! {
        <DetailPageTemplate title="Diet">
            <Transition fallback=LoadingSpinner>
                <ErrorBoundary fallback=|errors| {
                    view! { <ErrorComponent errors/> }
                }>{response}</ErrorBoundary>
            </Transition>
        </DetailPageTemplate>
    }
}

#[component]
fn DietDetailPageComponent(data: DietFoodQuery) -> impl IntoView {
    let title = data.title();
    view! {
        <section>
            <a href=data.food_detail_url() class="block mb-2 text-xl font-bold">
                {title}
            </a>
            <a href=data.brand_detail_url() class="block mb-4 font-bold capitalize">
                {&data.brand_name}
            </a>
        </section>
        <NutritionInformation data=data/>
        <UpdateDeleteButtonRow/>
    }
}

#[component]
pub fn NutritionTable<'a>(data: &'a Nutrition, per: String) -> impl IntoView {
    view! {
        <h3 class="mb-2 font-bold">"Nutrition Information"</h3>
        <table class="mb-4 w-full border-collapse">
            <thead>
                <tr>
                    <th class="p-2 w-1/2 text-left border">"Typical Values"</th>
                    <th class="p-2 w-1/2 text-right border">{per}</th>
                </tr>
            </thead>
            <tbody>
                <tr>
                    <th class="p-2 text-left border">"Energy (kcal)"</th>
                    <td class="p-2 text-right border">{format!("{:.0}", data.energy)} "kcal"</td>
                </tr>
                <tr>
                    <th class="p-2 text-left border">"Protein"</th>
                    <td class="p-2 text-right border">{format!("{:.1}", data.protein)}</td>
                </tr>
                <tr>
                    <th class="p-2 text-left border">"Carbohydrate"</th>
                    <td class="p-2 text-right border">{format!("{:.1}", data.carbohydrate)} "g"</td>
                </tr>
                <tr>
                    <th class="p-2 text-left border">"Fat"</th>
                    <td class="p-2 text-right border">{format!("{:.1}", data.fat)} "g"</td>
                </tr>
                <tr>
                    <th class="p-2 text-left border">"Sat. Fat"</th>
                    <td class="p-2 text-right border">{format!("{:.1}", data.saturates)} "g"</td>
                </tr>
                <tr>
                    <th class="p-2 text-left border">"Sugars"</th>
                    <td class="p-2 text-right border">{format!("{:.1}", data.sugars)} "g"</td>
                </tr>
                <tr>
                    <th class="p-2 text-left border">"Fibre"</th>
                    <td class="p-2 text-right border">{format!("{:.1}", data.fibre)} "g"</td>
                </tr>
                <tr>
                    <th class="p-2 text-left border">"Salt"</th>
                    <td class="p-2 text-right border">{format!("{:.2}", data.salt)} "g"</td>
                </tr>
            </tbody>
        </table>
    }
}
