use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::component::button::{Button, SubmitButton};
use crate::component::icon::IconUserMinus;
use crate::component::input::TextInput;
use crate::component::template::{ErrorComponent, ListLoadingComponent, ListNotFoundComponent};
use crate::user_block::model::UserBlock;
use crate::util::misc::ListResponse;
use crate::util::param::{extract_page, extract_param, extract_size};
use crate::util::validation_error::{extract_other_errors, get_non_field_errors};

#[cfg(feature = "ssr")]
use crate::{auth::model::User, auth::service::get_request_user, error::Error, setup::get_pool};

#[server(endpoint = "user-block-listed")]
pub async fn get_user_block_list(
    search: String,
    order: String,
    size: i64,
    page: i64,
) -> Result<ListResponse<UserBlock>, ServerFnError> {
    let user = get_request_user()?;
    let pool = get_pool()?;
    let count = UserBlock::count(&pool, &user.username, &search, "1").await?;
    let results =
        UserBlock::filter(&pool, &user.username, &search, "1", &order, size, page).await?;
    Ok(ListResponse { count, results })
}

#[server(endpoint = "create-user-block")]
pub async fn create_user_block(username: String) -> Result<(), ServerFnError> {
    let user = get_request_user()?;
    let pool = get_pool()?;
    if user.username == username {
        return Err(ServerFnError::new("You cannot block yourself"));
    }
    let target_user = User::get_by_username(&pool, &username)
        .await?
        .ok_or(Error::NotFound)?;
    let results = UserBlock::create(&pool, user.id, target_user.id, 1).await?;
    if results == 0 {
        return Err(ServerFnError::new("Error creating user block"))?;
    }
    Ok(())
}

#[server(endpoint = "update-user-block")]
pub async fn update_user_block(username: String) -> Result<(), ServerFnError> {
    let user = get_request_user()?;
    let pool = get_pool()?;
    if user.username == username {
        return Err(ServerFnError::new("You cannot block/unblock yourself"));
    }
    let user = User::get_by_username(&pool, &username)
        .await?
        .ok_or(Error::NotFound)?;
    let results =
        UserBlock::update_by_username_pair(&pool, &user.username, &user.username, 0).await?;
    if results == 0 {
        return Err(ServerFnError::new("Error updating user block"))?;
    }
    Ok(())
}

#[component]
pub fn UserBlockListPage() -> impl IntoView {
    let action_create_block = Action::<CreateUserBlock, _>::server();
    let action_update_block = Action::<UpdateUserBlock, _>::server();

    let query = use_query_map();
    let search = move || extract_param(&query, "search");
    let order = move || extract_param(&query, "order");
    let size = move || extract_size(&query);
    let page = move || extract_page(&query);

    let resource = Resource::new(
        move || {
            (
                search(),
                order(),
                size(),
                page(),
                action_create_block.version().get(),
                action_update_block.version().get(),
            )
        },
        |(search, order, size, page, _, _)| get_user_block_list(search, order, size, page),
    );

    let response = move || {
        resource.and_then(|data| {
            let count = data.count;
            let results = &data.results;
            if count == 0 {
                view! { <ListNotFoundComponent/> }
            } else {
                results.iter()
                    .map(|data| view! { <UserBlockListItem data=data.clone() action=action_update_block/> })
                    .collect_view()
            }
        })
    };
    let user_block_count = move || {
        resource.with(|opt| {
            opt.as_ref()
                .map(|res| res.as_ref().map(|res| res.count).unwrap_or_default())
        })
    };

    let action_value = action_update_block.value();
    let action_error = move || extract_other_errors(action_value, &["username"]);
    let non_field_errors = move || get_non_field_errors(action_value);

    view! {
        <Title text="Blocked Users"/>
        <div class="grid grid-cols-4 gap-4 md:grid-cols-12">
            <div class="col-span-4 md:col-span-8">
                <div class="p-4 bg-white border">
                    <header class="mb-4">
                        <h2 class="mb-2 text-base font-bold">
                            "Blocked Users"
                            <Transition fallback=|| {
                                "Loading".into_view()
                            }>" (" {user_block_count} ")"</Transition>
                        </h2>
                    </header>
                    <section>
                        <div class="mb-4 text-red-500 font-bold">{action_error}</div>
                        <div class="mb-4 text-red-500 font-bold">{non_field_errors}</div>
                    </section>
                    <Transition fallback=ListLoadingComponent>
                        <ErrorBoundary fallback=|errors| {
                            view! { <ErrorComponent errors/> }
                        }>{response}</ErrorBoundary>
                    </Transition>
                </div>
            </div>
            <div class="col-span-4">
                <div class="p-4 bg-white border">
                    <header class="mb-4">
                        <h2 class="mb-2 text-base font-bold">"Block User"</h2>
                    </header>
                    <div class="max-w-md">
                        <UserBlockCreateForm action=action_create_block/>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn UserBlockListItem(
    data: UserBlock,
    action: Action<UpdateUserBlock, Result<(), ServerFnError>>,
) -> impl IntoView {
    let created_at = data.blocked_at.format("%a %d %b %Y, %H:%M:%S").to_string();
    let blocked_user_href = format!("/users/{}", data.blocked_username);
    let blocked_username = data.blocked_username.clone();
    view! {
        <div class="flex items-start p-2 mb-4 bg-gray-100">
            <div class="flex-1">
                <div>
                    <A href=blocked_user_href class="text-blue-500 hover:underline">
                        {&data.blocked_username}
                    </A>
                </div>
                <div class="text-xs text-gray-500">{created_at}</div>
            </div>
            <div class="flex gap-2">
                <UserBlockUpdateButton blocked_username=blocked_username action/>
            </div>
        </div>
    }
}

#[component]
pub fn UserBlockUpdateButton(
    blocked_username: String,
    action: Action<UpdateUserBlock, Result<(), ServerFnError>>,
) -> impl IntoView {
    view! {
        <ActionForm action class="contents">
            <input type="hidden" name="username" value=blocked_username/>
            <Button label="Unblock User">
                <IconUserMinus/>
            </Button>
        </ActionForm>
    }
}

#[component]
pub fn UserBlockCreateForm(
    action: Action<CreateUserBlock, Result<(), ServerFnError>>,
) -> impl IntoView {
    let action_loading = action.pending();
    let action_value = action.value();
    let action_error = move || extract_other_errors(action_value, &["username"]);
    let non_field_errors = move || get_non_field_errors(action_value);
    view! {
        <div class="mb-4 text-red-500 font-bold">{action_error}</div>
        <div class="mb-4 text-red-500 font-bold">{non_field_errors}</div>
        <ActionForm action class="contents">
            <TextInput
                action_value
                name="username"
                label="Username"
                autocomplete="new-password"
                placeholder="Enter the username of who you wish to block"
            />
            <SubmitButton loading=action_loading label="Block User"/>
        </ActionForm>
    }
}
