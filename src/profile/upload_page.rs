use leptos::*;
use server_fn::codec::{MultipartData, MultipartFormData};

use wasm_bindgen::JsCast;
use web_sys::{FormData, HtmlFormElement, SubmitEvent};

use crate::component::button::SubmitButton;
use crate::component::template::DetailPageTemplate;

#[cfg(feature = "ssr")]
use ::{chrono::prelude::*, std::env, std::fs::File, std::io::Write, std::path::Path, uuid::Uuid};

#[cfg(feature = "ssr")]
pub fn generate_profile_image_filename(user_id: Uuid, file_extension: &str) -> String {
    let timestamp = Utc::now().format("%Y%m%dT%H%M%S").to_string();
    format!("profile_{}_{}.{}", user_id, timestamp, file_extension)
}

#[cfg(feature = "ssr")]
use crate::{auth::service::get_request_user, profile::model::ProfileImage, setup::get_pool};

#[server(input = MultipartFormData)]
pub async fn profile_image_upload(data: MultipartData) -> Result<(), ServerFnError> {
    let user = get_request_user()?;
    let pool = get_pool()?;

    let mut data = data.into_inner().unwrap();

    let current_dir = env::current_dir().map_err(|e| ServerFnError::new(e.to_string()))?;
    let upload_dir = current_dir.join("public/images/profile");

    let mut file_extension = String::new();
    let mut file_chunks = Vec::new();

    while let Ok(Some(mut field)) = data.next_field().await {
        let field_name = field.name().unwrap_or_default().to_string();
        if field_name == "file_upload" {
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

    let new_file_name = generate_profile_image_filename(user.id, &file_extension);
    // let file_path = upload_dir.join(&new_file_name);
    // println!("file_name: {}", &new_file_name);
    // println!("file_extension: {:?}", &file_path);

    if !file_chunks.is_empty() {
        let file_path = upload_dir.join(&new_file_name);
        let mut file = File::create(&file_path).map_err(|e| ServerFnError::new(e.to_string()))?;
        for chunk in file_chunks {
            file.write_all(&chunk)
                .map_err(|e| ServerFnError::new(e.to_string()))?;
        }
    }

    ProfileImage::update_profile_image(&pool, user.id, &new_file_name, user.id).await?;
    Ok(())
}

#[component]
pub fn ProfileImageUploadPage() -> impl IntoView {
    let upload_action = create_action(|data: &FormData| {
        let data = data.clone();
        profile_image_upload(data.into())
    });
    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        let target = ev.target().unwrap().unchecked_into::<HtmlFormElement>();
        let form_data = FormData::new_with_form(&target).unwrap();
        upload_action.dispatch(form_data);
    };

    view! {
        <DetailPageTemplate title="Upload Profile Picture">
            <form on:submit=on_submit>
                <input type="hidden" name="username" value="michael"/>
                <label class="block my-4">
                    <div class="mb-1">"Upload"</div>
                    <input
                        name="file_upload"
                        type="file"
                        class="block w-full rounded border px-3 py-1.5 shadow-sm focus:border-blue-500 focus:outline-none focus:ring-2 focus:ring-blue-500"
                    />
                </label>

                <SubmitButton/>
            </form>
        </DetailPageTemplate>
    }
}
