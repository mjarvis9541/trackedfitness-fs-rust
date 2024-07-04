use leptos::*;
use leptos_router::*;
use server_fn::codec::{MultipartData, MultipartFormData};

use wasm_bindgen::JsCast;
use web_sys::{FormData, HtmlFormElement, SubmitEvent};

use crate::component::button::SubmitButton;
use crate::component::template::{DetailPageTemplate, ErrorComponent, LoadingComponent};
use crate::util::param::get_slug;

use super::detail_page::get_brand_detail;

#[cfg(feature = "ssr")]
use {
    crate::{auth::service::get_request_user, brand::model::Brand, setup::get_pool},
    std::fs::File,
    std::io::Write,
    std::path::Path,
    std::{env, fs},
};

#[server(input = MultipartFormData)]
pub async fn upload_file_action(data: MultipartData) -> Result<(), ServerFnError> {
    let user = get_request_user()?;
    let pool = get_pool()?;

    let mut data = data.into_inner().unwrap();

    let current_dir = env::current_dir().map_err(|e| ServerFnError::new(e.to_string()))?;
    let upload_dir = current_dir.join("public/images/brands");
    if !upload_dir.exists() {
        fs::create_dir_all(upload_dir.clone()).map_err(|e| ServerFnError::new(e.to_string()))?;
    }

    let mut slug = String::new();
    let mut file_extension = String::new();
    let mut file_chunks = Vec::new();

    while let Ok(Some(mut field)) = data.next_field().await {
        let field_name = field.name().unwrap_or_default().to_string();
        if field_name == "brand_slug" {
            slug = field.text().await.unwrap_or_default();
            dbg!(slug.clone());
        } else if field_name == "file_to_upload" {
            let file_name = field.file_name().unwrap_or_default().to_string();
            file_extension = Path::new(&file_name)
                .extension()
                .unwrap_or_default()
                .to_str()
                .unwrap_or_default()
                .to_string();

            while let Ok(Some(chunk)) = field.chunk().await {
                file_chunks.push(chunk);
            }
        }
    }

    let new_file_name = format!("{}.{}", slug, file_extension);
    if !file_chunks.is_empty() {
        let file_path = upload_dir.join(&new_file_name);
        let mut file = File::create(&file_path).map_err(|e| ServerFnError::new(e.to_string()))?;

        for chunk in file_chunks {
            file.write_all(&chunk)
                .map_err(|e| ServerFnError::new(e.to_string()))?;
        }
    }

    Brand::update_image_url(&pool, &slug, &new_file_name, user.id).await?;
    Ok(())
}

#[component]
pub fn BrandImageUploadPage() -> impl IntoView {
    let params = use_params_map();
    let slug = move || get_slug(&params);

    let upload_action = create_action(|data: &FormData| {
        let data = data.clone();
        upload_file_action(data.into())
    });

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        let target = ev.target().unwrap().unchecked_into::<HtmlFormElement>();
        let form_data = FormData::new_with_form(&target).unwrap();
        upload_action.dispatch(form_data);
    };

    let resource = Resource::new(slug, get_brand_detail);
    let response = move || {
        resource.and_then(|data| {
            let id = data.id.to_string();
            let image = data
                .image_url
                .clone()
                .map(|image| view! { <img src=image alt=""/> });

            view! {
                <div>"here:" {image}</div>
                <div>{id}</div>
            }
        })
    };

    view! {
        <DetailPageTemplate title="Upload Brand Image">
            <Transition fallback=LoadingComponent>
                <ErrorBoundary fallback=|errors| {
                    view! { <ErrorComponent errors/> }
                }>{response}</ErrorBoundary>
            </Transition>

            <form on:submit=on_submit>
                <input type="hidden" name="brand_slug" value=slug/>
                <input
                    name="file_to_upload"
                    type="file"
                    class="block w-full rounded border px-3 py-1.5 shadow-sm focus:border-blue-500 focus:outline-none focus:ring-2 focus:ring-blue-500"
                />

                <SubmitButton/>
            </form>
        </DetailPageTemplate>
        <section class="mt-4 mx-auto max-w-lg border p-4">
            <h2>"Image..."</h2>

        </section>
    }
}
