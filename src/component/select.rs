use leptos::*;

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::util::text::capitalize_and_replace;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SelectUuidName {
    pub id: Uuid,
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SelectSlugName {
    pub slug: String,
    pub name: String,
}

pub fn generate_macronutrient_sort_options() -> Vec<(&'static str, &'static str)> {
    vec![
        ("-energy", "Calories (High-Low)"),
        ("energy", "Calories (Low-High)"),
        ("-protein", "Protein (High-Low)"),
        ("protein", "Protein (Low-High)"),
        ("-carbohydrate", "Carbs (High-Low)"),
        ("carbohydrate", "Carbs (Low-High)"),
        ("-fat", "Fat (High-Low)"),
        ("fat", "Fat (Low-High)"),
        ("-saturates", "Saturates (High-Low)"),
        ("saturates", "Saturates (Low-High)"),
        ("-sugars", "Sugars (High-Low)"),
        ("sugars", "Sugars (Low-High)"),
        ("-fibre", "Fibre (High-Low)"),
        ("fibre", "Fibre (Low-High)"),
        ("-salt", "Salt (High-Low)"),
        ("salt", "Salt (Low-High)"),
    ]
}

pub fn generate_created_updated_sort_options() -> Vec<(&'static str, &'static str)> {
    vec![
        ("-created_at", "Created (Desc)"),
        ("created_at", "Created (Asc)"),
        ("-updated_at", "Updated (Desc)"),
        ("updated_at", "Updated (Asc)"),
    ]
}
pub fn generate_diet_sort_options() -> Vec<(&'static str, &'static str)> {
    vec![
        ("-last_added_quantity", "Last Added Quantity (Desc)"),
        ("last_added_quantity", "Last Added Quantity (Asc)"),
        ("-last_added_date", "Last Added Date (Desc)"),
        ("last_added_date", "Last Added Date (Asc)"),
    ]
}

lazy_static! {
    pub static ref BOOLEAN_OPTIONS: Vec<(&'static str, &'static str)> =
        vec![("", "All"), ("true", "True"), ("false", "False"),];
    pub static ref SERVING_OPTIONS: Vec<(&'static str, &'static str)> = vec![
        ("", "All"),
        ("g", "100g"),
        ("ml", "100ml"),
        ("srv", "1 Serving"),
    ];
    pub static ref FOLLOWER_STATUS_OPTIONS: Vec<(&'static str, &'static str)> = vec![
        ("", "All"),
        ("0", "Pending"),
        ("1", "Accepted"),
        ("2", "Declined"),
    ];
    pub static ref FORM_BLOCKED_STATUS_OPTIONS: Vec<(&'static str, &'static str)> =
        vec![("0", "Unblocked"), ("1", "Blocked"),];
    pub static ref BLOCKED_STATUS_OPTIONS: Vec<(&'static str, &'static str)> =
        vec![("", "All"), ("0", "Unblocked"), ("1", "Blocked"),];
    pub static ref FOLLOWER_STATUS_FORM_OPTIONS: Vec<(&'static str, &'static str)> =
        vec![("0", "Pending"), ("1", "Accepted"), ("2", "Declined")];
    pub static ref USER_PRIVACY_OPTIONS: Vec<(&'static str, &'static str)> = vec![
        ("", "All"),
        ("0", "N/A"),
        ("1", "Public"),
        ("2", "Followers Only"),
        ("3", "Private"),
    ];
    pub static ref USER_PRIVACY_FORM_OPTIONS: Vec<(&'static str, &'static str)> = vec![
        ("0", "N/A - All users can view your profile"),
        ("1", "Public - All users can view your profile"),
        ("2", "Followers Only - Only followers can view your profile"),
        ("3", "Private - No users can view your profile"),
    ];
    pub static ref FOOD_SORT_OPTIONS: Vec<(&'static str, &'static str)> = {
        let mut options = vec![
            ("name", "Food (A-z)"),
            ("-name", "Food (Z-a)"),
            ("brand_name", "Brand (A-z)"),
            ("-brand_name", "Brand (Z-a)"),
        ];
        options.extend(generate_macronutrient_sort_options());
        options.extend(generate_created_updated_sort_options());
        options
    };
    pub static ref BRAND_SORT_OPTIONS: Vec<(&'static str, &'static str)> = {
        let mut options = vec![
            ("name", "Name (A-z)"),
            ("-name", "Name (Z-a)"),
            ("-food_count", "Food Count (High-Low)"),
            ("food_count", "Food Count (Low-High)"),
        ];
        options.extend(generate_created_updated_sort_options());
        options
    };
    pub static ref MEAL_SORT_OPTIONS: Vec<(&'static str, &'static str)> = {
        let mut options = vec![
            ("name", "Name (A-z)"),
            ("-name", "Name (Z-a)"),
            ("-food_count", "Food Count (High-Low)"),
            ("food_count", "Food Count (Low-High)"),
        ];
        options.extend(generate_macronutrient_sort_options());
        options.extend(generate_created_updated_sort_options());
        options
    };
    pub static ref MEAL_OF_DAY_SORT_OPTIONS: Vec<(&'static str, &'static str)> = vec![
        ("name", "Name (A-z)"),
        ("-name", "Name (Z-a)"),
        ("ordering", "Order (Asc)"),
        ("-ordering", "Order (Desc)"),
        ("created_at", "Created (Asc)"),
        ("-created_at", "Created (Desc)"),
        ("updated_at", "Updated (Asc)"),
        ("-updated_at", "Updated (Desc)"),
    ];
    pub static ref DIET_FOOD_SORT_OPTIONS: Vec<(&'static str, &'static str)> = {
        let mut options = vec![
            ("name", "Food (A-z)"),
            ("-name", "Food (Z-a)"),
            ("brand_name", "Brand (A-z)"),
            ("-brand_name", "Brand (Z-a)"),
        ];
        options.extend(generate_diet_sort_options());
        options.extend(generate_macronutrient_sort_options());
        options.extend(generate_created_updated_sort_options());
        options
    };
    pub static ref FOLLOWER_SORT_OPTIONS: Vec<(&'static str, &'static str)> = vec![
        ("username", "Username (A-z)"),
        ("-username", "Username (Z-a)"),
        ("follower", "Follower (A-z)"),
        ("-follower", "Follower (Z-a)"),
        ("created_at", "Created (Asc)"),
        ("-created_at", "Created (Desc)"),
        ("updated_at", "Updated (Asc)"),
        ("-updated_at", "Updated (Desc)"),
        ("status", "Status (Asc)"),
        ("-status", "Status (Desc)"),
    ];
    pub static ref USER_SORT_OPTIONS: Vec<(&'static str, &'static str)> = vec![
        ("username", "Username (A-z)"),
        ("-username", "Username (Z-a)"),
        ("name", "Name (A-z)"),
        ("-name", "Name (Z-a)"),
        ("email", "Email (A-z)"),
        ("-email", "Email (Z-a)"),
        ("created_at", "Created (Asc)"),
        ("-created_at", "Created (Desc)"),
        ("updated_at", "Updated (Asc)"),
        ("-updated_at", "Updated (Desc)"),
        ("last_login", "Last Login (Asc)"),
        ("-last_login", "Last Login (Desc)"),
    ];
    pub static ref USER_STAT_SORT_OPTIONS: Vec<(&'static str, &'static str)> = vec![
        ("username", "Username (A-z)"),
        ("-username", "Username (Z-a)"),
        ("-follower_count", "Follower Count (Desc)"),
        ("follower_count", "Follower Count (Asc)"),
        ("-following_count", "Following Count (Desc)"),
        ("following_count", "Following Count (Asc)"),
        ("-diet_count", "Diet Count (Desc)"),
        ("diet_count", "Diet Count (Asc)"),
        ("-diet_day_log_count", "Diet Day Log Count (Desc)"),
        ("diet_day_log_count", "Diet Day Log Count (Asc)"),
        ("-diet_target_count", "Diet Target Count (Desc)"),
        ("diet_target_count", "Diet Target Count (Asc)"),
        ("-progress_count", "Progress Count (Desc)"),
        ("progress_count", "Progress Count (Asc)"),
        ("-workout_count", "Workout Count (Desc)"),
        ("workout_count", "Workout Count (Asc)"),
        ("-workout_day_log_count", "Workout Day Log Count (Desc)"),
        ("workout_day_log_count", "Workout Day Log Count (Asc)"),
        ("-exercise_count", "Exercise Count (Desc)"),
        ("exercise_count", "Exercise Count (Asc)"),
        ("-set_count", "Set Count (Desc)"),
        ("set_count", "Set Count (Asc)"),
        ("-rep_count", "Rep Count (Desc)"),
        ("rep_count", "Rep Count (Asc)"),
        ("-food_created_count", "Food Created Count (Desc)"),
        ("food_created_count", "Food Created Count (Asc)"),
        ("-brand_created_count", "Brand Created Count (Desc)"),
        ("brand_created_count", "Brand Created Count (Asc)"),
        ("-meal_created_count", "Meal Created Count (Desc)"),
        ("meal_created_count", "Meal Created Count (Asc)"),
        ("-meal_food_created_count", "Meal Food Created Count (Desc)"),
        ("meal_food_created_count", "Meal Food Created Count (Asc)"),
        (
            "-meal_of_day_created_count",
            "Meal Of Day Created Count (Desc)"
        ),
        (
            "meal_of_day_created_count",
            "Meal Of Day Created Count (Asc)"
        ),
        ("-movement_created_count", "Movement Created Count (Desc)"),
        ("movement_created_count", "Movement Created Count (Asc)"),
        (
            "-muscle_group_created_count",
            "Muscle Group Created Count (Desc)"
        ),
        (
            "muscle_group_created_count",
            "Muscle Group Created Count (Asc)"
        ),
    ];
    pub static ref MOVEMENT_SORT_OPTIONS: Vec<(&'static str, &'static str)> = vec![
        ("name", "Name (A-z)"),
        ("-name", "Name (Z-a)"),
        ("created_at", "Created (Asc)"),
        ("-created_at", "Created (Desc)"),
        ("updated_at", "Updated (Asc)"),
        ("-updated_at", "Updated (Desc)"),
    ];
    pub static ref MUSCLE_GROUP_SORT_OPTIONS: Vec<(&'static str, &'static str)> = vec![
        ("name", "Name (A-z)"),
        ("-name", "Name (Z-a)"),
        ("created_at", "Created (Asc)"),
        ("-created_at", "Created (Desc)"),
        ("updated_at", "Updated (Asc)"),
        ("-updated_at", "Updated (Desc)"),
    ];
    pub static ref DATE_SORT_OPTIONS: Vec<(&'static str, &'static str)> = vec![
        ("-date", "Date (Desc)"),
        ("date", "Date (Asc)"),
        ("created_at", "Created (Asc)"),
        ("-created_at", "Created (Desc)"),
        ("updated_at", "Updated (Asc)"),
        ("-updated_at", "Updated (Desc)"),
    ];
    pub static ref GENERIC_SORT_OPTIONS: Vec<(&'static str, &'static str)> = vec![
        ("name", "Name (A-z)"),
        ("-name", "Name (Z-a)"),
        ("created_at", "Created (Asc)"),
        ("-created_at", "Created (Desc)"),
        ("updated_at", "Updated (Asc)"),
        ("-updated_at", "Updated (Desc)"),
    ];
}

#[component]
pub fn FilterSelect(
    name: &'static str,
    #[prop(default = name)] label: &'static str,
    options: &'static [(&'static str, &'static str)],
    value: Signal<String>,
) -> impl IntoView {
    let options_ref = options;

    let options = move || {
        options_ref
            .iter()
            .map(|(opt_value, opt_label)| {
                let is_selected = move || **opt_value == *value.get();
                view! {
                    <option value=*opt_value selected=is_selected()>
                        {*opt_label}
                    </option>
                }
            })
            .collect_view()
    };

    view! {
        <label class="block flex-1 min-w-40" aria-labelledby=format!("{}-label", name)>
            <span id=format!("{}-label", name) class="block mb-1 font-bold capitalize">
                {label}
            </span>
            <select
                name=name
                onchange="this.form.requestSubmit()"
                class="flex py-1.5 px-3 w-full bg-white rounded border focus:border-blue-500 focus:ring-2 focus:ring-blue-500 focus:outline-none disabled:bg-gray-500 disabled:opacity-50 h-[34px] placeholder:text-gray-400 disabled:placeholder:text-gray-500"
                aria-labelledby=format!("{}-label", name)
            >
                {options}
            </select>
        </label>
    }
}

#[component]
pub fn FieldSelect(
    name: &'static str,
    #[prop(default = name)] label: &'static str,
    options: &'static [(&'static str, &'static str)],
    #[prop(optional, into)] value: String,
) -> impl IntoView {
    let options_ref = options;
    let options = move || {
        options_ref
            .iter()
            .map(|(option_value, label)| {
                let selected_value = value.clone();
                let option_value = option_value.to_string();
                view! {
                    <option
                        prop:value=&option_value
                        prop:selected=move || option_value == selected_value
                    >
                        {*label}
                    </option>
                }
            })
            .collect_view()
    };

    view! {
        <label class="block flex-1 mb-4">
            <span class="block mb-1 font-bold">{capitalize_and_replace(label)}</span>
            <select
                name=name
                class="flex py-1.5 px-3 w-full bg-white rounded border focus:border-blue-500 focus:ring-2 focus:ring-blue-500 focus:outline-none disabled:bg-gray-500 disabled:opacity-50 h-[34px] placeholder:text-gray-400 disabled:placeholder:text-gray-500"
            >
                {options}
            </select>
        </label>
    }
}

#[component]
pub fn FieldSelectB(
    name: &'static str,
    #[prop(default = name)] label: &'static str,
    #[prop(optional, into)] value: MaybeSignal<String>,
    options: Vec<(&'static str, &'static str)>,
) -> impl IntoView {
    let options_view = move || {
        options
            .clone()
            .into_iter()
            .map(|(option_value, label)| {
                let selected_value = value.get();
                view! {
                    <option
                        value=option_value
                        prop:selected=move || *option_value == selected_value
                    >
                        {label}
                    </option>
                }
            })
            .collect_view()
    };
    view! {
        <label class="block mb-4">
            <span class="block mb-1 text-sm font-bold">{capitalize_and_replace(label)}</span>
            <select
                name=name
                class="block py-2.5 px-3 w-full bg-white rounded border focus:border-blue-500 focus:ring-2 focus:ring-blue-500 focus:outline-none"
            >
                {options_view}
            </select>
        </label>
    }
}

#[component]
pub fn SelectOption(
    value: &'static str,
    label: &'static str,
    selected: Signal<String>,
) -> impl IntoView {
    view! {
        <option value=value selected=move || selected() == value>
            {label}
        </option>
    }
}
