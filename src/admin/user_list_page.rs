use std::collections::HashSet;

use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::auth::model::User;
use crate::component::bulk_delete::BulkDeleteForm;
use crate::component::checkbox::CheckboxListItem;
use crate::component::input::FilterInput;
use crate::component::paginator::Paginator;
use crate::component::select::FilterSelect;
use crate::component::template::{
    AutoListHeader, ErrorComponent, ListLoadingComponent, ListNotFoundComponent,
    ListPageHeaderWithCreate, Loading,
};
use crate::util::datetime::format_datetime;
use crate::util::misc::ListResponse;
use crate::util::param::{extract_page, extract_param, extract_size};

#[server]
async fn get_admin_user_list(
    search: String,
    active: String,
    verified: String,
    staff: String,
    superuser: String,
    privacy: String,
    order: String,
    size: i64,
    page: i64,
) -> Result<ListResponse<User>, ServerFnError> {
    use crate::util::database::Filter;
    crate::auth::service::extract_superuser_from_request()?;
    let pool = crate::setup::get_pool()?;

    let mut qbc = sqlx::QueryBuilder::new("SELECT COUNT(*) FROM users_user WHERE TRUE");
    qbc.filter("username", "ilike", &search);
    let count = qbc.build_query_scalar().fetch_one(&pool).await?;

    let mut qb = sqlx::QueryBuilder::new("SELECT * FROM users_user WHERE TRUE");
    qb.filter("username", "ilike", &search);

    if !active.is_empty() {
        let active_bool = active.to_lowercase() == "true";
        qb.push(" AND is_active = ");
        qb.push_bind(active_bool);
    };
    if !verified.is_empty() {
        let verified_bool = verified.to_lowercase() == "true";
        qb.push(" AND email_verified = ");
        qb.push_bind(verified_bool);
    }
    if !staff.is_empty() {
        let staff_bool = staff.to_lowercase() == "true";
        qb.push(" AND is_staff = ");
        qb.push_bind(staff_bool);
    }
    if !superuser.is_empty() {
        let superuser_bool = superuser.to_lowercase() == "true";
        qb.push(" AND is_superuser = ");
        qb.push_bind(superuser_bool);
    }
    if !privacy.is_empty() {
        let privacy = privacy.parse::<i32>().unwrap_or_default();
        qb.push(" AND privacy_level = ");
        qb.push_bind(privacy);
    }

    qb.order("last_login desc, created_at desc", &order);
    qb.paginate(size, page);

    let results = qb.build_query_as().fetch_all(&pool).await?;

    Ok(ListResponse { count, results })
}

#[component]
pub fn AdminUserListPage() -> impl IntoView {
    let action_bulk_delete = Action::server();

    let query = use_query_map();
    let search = move || extract_param(&query, "search");
    let verified = move || extract_param(&query, "verified");
    let active = move || extract_param(&query, "active");
    let staff = move || extract_param(&query, "staff");
    let superuser = move || extract_param(&query, "superuser");
    let privacy = move || extract_param(&query, "privacy");
    let order = move || extract_param(&query, "order");
    let size = move || extract_size(&query);
    let page = move || extract_page(&query);
    let resource = Resource::new(
        move || {
            (
                search(),
                active(),
                verified(),
                staff(),
                superuser(),
                privacy(),
                order(),
                size(),
                page(),
                action_bulk_delete.version().get(),
            )
        },
        |(search, active, verified, staff, superuser, privacy, order, size, page, _)| {
            get_admin_user_list(
                search, active, verified, staff, superuser, privacy, order, size, page,
            )
        },
    );
    let all_items = RwSignal::new(HashSet::new());
    let checked_items = RwSignal::new(HashSet::new());
    let response = move || {
        resource.and_then(|data| {
            let count = &data.count;
            let results = &data.results;
            if *count == 0 {
                view! { <ListNotFoundComponent/> }
            } else {
                let ids: HashSet<String> = results.iter().map(|item| item.id.to_string()).collect();
                all_items.update(|set| set.extend(ids));
                results
                    .iter()
                    .map(|data| {
                        view! { <AdminUserListItem data=data.clone() checked_items/> }
                    })
                    .collect_view()
            }
        })
    };

    let count = move || {
        resource.with(|opt| {
            opt.as_ref()
                .and_then(|res| res.as_ref().ok().map(|res| res.count))
        })
    };
    view! {
        <Title text="Admin - Users"/>
        <main class="lg:p-4">

            <div class="p-4 mb-2 bg-white">

                <ListPageHeaderWithCreate title="Admin - Users" create_href="create">
                    <Transition fallback=Loading>{count}</Transition>
                </ListPageHeaderWithCreate>

                <div class="flex flex-wrap gap-2 mb-2 whitespace-nowrap">
                    <Form method="GET" action="" class="contents">
                        <FilterInput name="search" value=Signal::derive(search)/>
                        <input type="hidden" name="size" value=size/>
                        <input type="hidden" name="page" value=1/>
                        <FilterSelect
                            name="verified"
                            label="Email Verified"
                            value=Signal::derive(verified)
                            options=&crate::component::select::BOOLEAN_OPTIONS
                        />
                        <FilterSelect
                            name="active"
                            value=Signal::derive(order)
                            options=&crate::component::select::BOOLEAN_OPTIONS
                        />
                        <FilterSelect
                            name="staff"
                            value=Signal::derive(staff)
                            options=&crate::component::select::BOOLEAN_OPTIONS
                        />
                        <FilterSelect
                            name="superuser"
                            value=Signal::derive(superuser)
                            options=&crate::component::select::BOOLEAN_OPTIONS
                        />
                        <FilterSelect
                            name="privacy"
                            value=Signal::derive(privacy)
                            options=&crate::component::select::USER_PRIVACY_OPTIONS
                        />
                        <FilterSelect
                            name="order"
                            value=Signal::derive(order)
                            options=&crate::component::select::USER_SORT_OPTIONS
                        />
                    </Form>
                </div>
            </div>

            <section class="grid overflow-auto p-4 whitespace-nowrap bg-white grid-cols-checkbox-12">
                <AutoListHeader all_items checked_items>
                    "Username"
                    "Name"
                    "Email"
                    "Verified"
                    "Active"
                    "Staff"
                    "Superuser"
                    "Privacy"
                    "Last Login"
                    "Created"
                    "Updated"
                    "Profile"
                </AutoListHeader>
                <Transition fallback=ListLoadingComponent>
                    <ErrorBoundary fallback=|errors| {
                        view! { <ErrorComponent errors/> }
                    }>{response}</ErrorBoundary>
                </Transition>
            </section>

            <div class="flex flex-wrap justify-between p-4 bg-white">
                <div>
                    <BulkDeleteForm table="users_user" action=action_bulk_delete checked_items/>
                </div>

                <div>
                    <Form method="GET" action="" class="contents">
                        <input type="hidden" name="search" value=search/>
                        <input type="hidden" name="order" value=order/>
                        <input type="hidden" name="page" value=page/>
                        <Transition>
                            <Paginator count/>
                        </Transition>
                    </Form>
                </div>
            </div>

        </main>
    }
}

#[component]
pub fn AdminUserListItem(data: User, checked_items: RwSignal<HashSet<String>>) -> impl IntoView {
    let created_at = format_datetime(&Some(data.created_at));
    let updated_at = format_datetime(&data.updated_at);
    let last_login = format_datetime(&data.last_login);

    let user_href = format!("/users/{}", data.username);
    view! {
        <div class="contents group">
            <div class="flex items-center p-2 group-hover:bg-gray-200 group-odd:bg-gray-50">
                <CheckboxListItem id=data.id.to_string() checked_items/>
            </div>
            <div class="flex items-center p-2 group-hover:bg-gray-200 group-odd:bg-gray-50">
                <A href=format!("/admin/users/{}", data.id)>{&data.username}</A>
            </div>
            <div class="flex items-center p-2 group-hover:bg-gray-200 group-odd:bg-gray-50">
                {data.name}
            </div>
            <div class="flex items-center p-2 group-hover:bg-gray-200 group-odd:bg-gray-50 truncate">
                {data.email}
            </div>
            <div class="flex items-center p-2 group-hover:bg-gray-200 group-odd:bg-gray-50">
                {data.email_verified.to_string()}
            </div>
            <div class="flex items-center p-2 group-hover:bg-gray-200 group-odd:bg-gray-50">
                {data.is_active.to_string()}
            </div>
            <div class="flex items-center p-2 group-hover:bg-gray-200 group-odd:bg-gray-50">
                {data.is_staff.to_string()}
            </div>
            <div class="flex items-center p-2 group-hover:bg-gray-200 group-odd:bg-gray-50">
                {data.is_superuser.to_string()}
            </div>
            <div class="flex items-center p-2 group-hover:bg-gray-200 group-odd:bg-gray-50">
                {data.privacy_level.to_string()}
            </div>
            <div class="flex items-center p-2 group-hover:bg-gray-200 group-odd:bg-gray-50 truncate">
                {last_login}
            </div>
            <div class="flex items-center p-2 group-hover:bg-gray-200 group-odd:bg-gray-50 truncate">
                {created_at}
            </div>
            <div class="flex items-center p-2 group-hover:bg-gray-200 group-odd:bg-gray-50 truncate">
                {updated_at}
            </div>
            <div class="flex justify-center items-center p-2 group-hover:bg-gray-200 group-odd:bg-gray-50">
                <A href=user_href class="text-blue-500 hover:underline">
                    "View"
                </A>
            </div>
        </div>
    }
}
