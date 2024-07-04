use leptos::*;
use leptos_router::*;

use crate::component::template::{
    DetailPageTemplate, ErrorComponent, LoadingComponent, UpdateDeleteButtonRow,
};
use crate::util::datetime::format_datetime;
use crate::util::param::get_slug;

use super::model::BrandQuery;

#[cfg(feature = "ssr")]
use crate::{auth::service::get_request_user, error::Error, setup::get_pool};

#[server(endpoint = "brand-detail")]
pub async fn get_brand_detail(slug: String) -> Result<BrandQuery, ServerFnError> {
    let user = get_request_user()?;
    let pool = get_pool()?;

    let object = BrandQuery::get_by_slug(&pool, &slug)
        .await?
        .ok_or(Error::NotFound)?;
    object.can_view(&user)?;

    Ok(object)
}

#[component]
pub fn BrandDetailPage() -> impl IntoView {
    let params = use_params_map();
    let slug = move || get_slug(&params);

    let resource = Resource::new(slug, get_brand_detail);
    let response = move || resource.and_then(|data| view! { <BrandDetail data=data.clone()/> });

    view! {
        <DetailPageTemplate title="Brand">
            <Transition fallback=LoadingComponent>
                <ErrorBoundary fallback=|errors| {
                    view! { <ErrorComponent errors/> }
                }>{response}</ErrorBoundary>
            </Transition>
        </DetailPageTemplate>
    }
}

#[component]
pub fn BrandDetail(data: BrandQuery) -> impl IntoView {
    let created_at = format_datetime(&Some(data.created_at));
    let updated_at = format_datetime(&data.updated_at);

    view! {
        <header class="mb-4">
            <h1 class="text-xl font-bold">{data.name}</h1>
            <p class="text-gray-500">{data.id.to_string()}</p>
        </header>
        <table class="mb-4 w-full border-collapse">
            <tbody>
                <tr>
                    <th class="p-2 w-1/2 text-left border">"Slug"</th>
                    <td class="p-2 w-1/2 text-right border">{data.slug}</td>
                </tr>
                <tr>
                    <th class="p-2 w-1/2 text-left border">"Created by"</th>
                    <td class="p-2 w-1/2 text-right border">{data.created_by}</td>
                </tr>
                <tr>
                    <th class="p-2 w-1/2 text-left border">"Updated by"</th>
                    <td class="p-2 w-1/2 text-right border">{data.updated_by}</td>
                </tr>
                <tr>
                    <th class="p-2 w-1/2 text-left border">"Image url"</th>
                    <td class="p-2 w-1/2 text-right border">
                        {data.image_url.unwrap_or_default()}
                    </td>
                </tr>
                <tr>
                    <th class="p-2 w-1/2 text-left border">"Created"</th>
                    <td class="p-2 w-1/2 text-right border">{created_at}</td>
                </tr>
                <tr>
                    <th class="p-2 w-1/2 text-left border">"Updated"</th>
                    <td class="p-2 w-1/2 text-right border">{updated_at}</td>
                </tr>
            </tbody>
        </table>
        <div class="my-4">
            <A class="text-blue-500 hover:underline" href="upload">
                "Upload Image"
            </A>
        </div>
        <UpdateDeleteButtonRow/>
    }
}
