use leptos::*;
use leptos_router::*;

use crate::app::FollowerCount;
use crate::component::icon::IconUsers;

#[derive(Debug, Default)]
pub enum LinkVariant {
    #[default]
    Primary,
    // Secondary,
    // Danger,
    Navigation,
    Page,
    UserNavLink,
    // Wide,
}

impl LinkVariant {
    pub fn into_css(&self) -> &'static str {
        match self {
            Self::Primary => "flex gap-2 p-2 whitespace-nowrap bg-gray-100 hover:bg-gray-200 hover:text-white",
            // Self::Secondary => "flex gap-2 p-2 whitespace-nowrap bg-gray-100 hover:bg-amber-200",
            // Self::Danger => {
                // "flex gap-2 p-2 whitespace-nowrap bg-gray-100 hover:bg-red-500 hover:text-white"
            // }
            Self::Navigation => "flex gap-2 px-4 py-2 hover:bg-amber-300 hover:text-white duration-300 aria-[current=page]:bg-zinc-600 aria-[current=page]:text-white",
            Self::Page => "flex gap-2 py-2 px-3 whitespace-nowrap duration-300 hover:bg-amber-300 bg-zinc-900 text-zinc-100",
            Self::UserNavLink => "flex gap-2 p-2 whitespace-nowrap hover:bg-amber-200 aria-[current=page]:bg-amber-200",
            // Self::Wide => "flex gap-2 px-4 py-2 hover:bg-amber-200 aria-[current=page]:bg-amber-200"
        }
    }
}

#[component]
pub fn Link<H>(
    href: H,
    #[prop(optional, into)] text: MaybeSignal<String>,
    #[prop(optional)] children: Option<Children>,
    #[prop(optional)] exact: bool,
    #[prop(optional)] variant: LinkVariant,
) -> impl IntoView
where
    H: ToHref + 'static,
{
    view! {
        <A href class=variant.into_css() exact=exact>
            {children.map(|children| children())}
            {text}
        </A>
    }
}

#[derive(Debug, Default)]
pub enum CircularIconLinkVariant {
    Large,
    #[default]
    Medium,
}

impl CircularIconLinkVariant {
    pub fn into_css(&self) -> &'static str {
        match self {
            Self::Large => "flex justify-center items-center w-10 h-10 text-xl font-bold text-white capitalize bg-red-500 rounded-full hover:bg-red-700 select-none shrink-0",
            Self::Medium => "flex justify-center items-center w-9 h-9 text-base font-bold text-white capitalize bg-blue-700 rounded-full hover:bg-blue-900 select-none shrink-0",  
        }
    }
}

#[component]
pub fn CircularIconLink<H>(
    href: H,
    initial: char,
    #[prop(optional)] variant: CircularIconLinkVariant,
) -> impl IntoView
where
    H: ToHref + 'static,
{
    view! {
        <A class=variant.into_css() href=href>
            {initial}
        </A>
    }
}

#[component]
pub fn NotificationLink<H>(href: H) -> impl IntoView
where
    H: ToHref + 'static,
{
    let follower_count = expect_context::<FollowerCount>().0;

    view! {
        <A class="block relative p-2 rounded bg-zinc-900 text-zinc-100 hover:bg-zinc-700" href=href>
            <IconUsers/>
            <span class="inline-flex absolute justify-center items-center w-5 h-5 text-xs font-bold leading-none text-red-100 bg-red-600 rounded-full top-[-6px] right-[-6px]">
                {follower_count}
            </span>
        </A>
    }
}

// pub enum HTMLLinkVariant {
//     Primary,
//     Secondary,
//     Danger,
// }

// impl HTMLLinkVariant {
//     pub fn to_css(&self) -> &'static str {
//         match self {
//             HTMLLinkVariant::Primary => "block px-3 py-1.5 rounded bg-gray-100 hover:bg-gray-200",
//             HTMLLinkVariant::Secondary => todo!(),
//             HTMLLinkVariant::Danger => todo!(),
//         }
//     }
// }

// #[component]
// pub fn HTMLLink(
//     #[prop(into)] href: String,
//     #[prop(into)] text: String,
//     #[prop(default = HTMLLinkVariant::Primary)] variant: HTMLLinkVariant,
//     #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
// ) -> impl IntoView {
//     view! {
//         <a {..attrs} href=href>
//             {text}
//         </a>
//     }
// }
