use leptos::*;

#[component]
fn SvgWrapper(
    size: usize,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    view! {
        <svg
            {..attrs}
            class="flex-shrink-0 stroke-current stroke-2 fill-none"
            xmlns="http://www.w3.org/2000/svg"
            width=size
            height=size
            viewBox="0 0 24 24"
            stroke-linecap="round"
            stroke-linejoin="round"
        >
            {children()}
        </svg>
    }
}

#[component]
pub fn Chevron(
    direction: &'static str,
    #[prop(default = 20)] size: usize,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
) -> impl IntoView {
    let points = if direction == "up" {
        "18 15 12 9 6 15"
    } else {
        "6 9 12 15 18 9"
    };
    view! {
        <SvgWrapper size attrs>
            <polyline points=points></polyline>
        </SvgWrapper>
    }
}

#[component]
pub fn ChevronUp(
    #[prop(default = 20)] size: usize,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
) -> impl IntoView {
    view! {
        <SvgWrapper size attrs>
            <polyline points="18 15 12 9 6 15"></polyline>
        </SvgWrapper>
    }
}

#[component]
pub fn ChevronDown(
    #[prop(default = 20)] size: usize,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
) -> impl IntoView {
    view! {
        <SvgWrapper size attrs>
            <polyline points="6 9 12 15 18 9"></polyline>
        </SvgWrapper>
    }
}

#[component]
pub fn ChevronLeft(
    #[prop(default = 20)] size: usize,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
) -> impl IntoView {
    view! {
        <SvgWrapper size attrs>
            <path d="M15 18l-6-6 6-6"></path>
        </SvgWrapper>
    }
}

#[component]
pub fn ChevronRight(
    #[prop(default = 20)] size: usize,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
) -> impl IntoView {
    view! {
        <SvgWrapper size attrs>
            <path d="M9 18l6-6-6-6"></path>
        </SvgWrapper>
    }
}

#[component]
pub fn ChevronsLeft(
    #[prop(default = 20)] size: usize,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
) -> impl IntoView {
    view! {
        <SvgWrapper size attrs>
            <polyline points="11 17 6 12 11 7"></polyline>
            <polyline points="18 17 13 12 18 7"></polyline>
        </SvgWrapper>
    }
}

#[component]
pub fn ChevronsRight(
    #[prop(default = 20)] size: usize,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
) -> impl IntoView {
    view! {
        <SvgWrapper size attrs>
            <polyline points="13 17 18 12 13 7"></polyline>
            <polyline points="6 17 11 12 6 7"></polyline>
        </SvgWrapper>
    }
}

#[component]
pub fn IconCalendar(
    #[prop(default = 20)] size: usize,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
) -> impl IntoView {
    view! {
        <SvgWrapper size attrs>
            <rect x="3" y="4" width="18" height="18" rx="2" ry="2"></rect>
            <line x1="16" y1="2" x2="16" y2="6"></line>
            <line x1="8" y1="2" x2="8" y2="6"></line>
            <line x1="3" y1="10" x2="21" y2="10"></line>
        </SvgWrapper>
    }
}

#[component]
pub fn IconCheck(
    #[prop(default = 20)] size: usize,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
) -> impl IntoView {
    view! {
        <SvgWrapper size attrs>
            <polyline points="20 6 9 17 4 12"></polyline>
        </SvgWrapper>
    }
}

#[component]
pub fn IconClose(
    #[prop(default = 20)] size: usize,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
) -> impl IntoView {
    view! {
        <SvgWrapper size attrs>
            <line x1="18" x2="6" y1="6" y2="18"></line>
            <line x1="6" x2="18" y1="6" y2="18"></line>
        </SvgWrapper>
    }
}

#[component]
pub fn IconCopy(
    #[prop(default = 20)] size: usize,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
) -> impl IntoView {
    view! {
        <SvgWrapper size attrs>
            <rect x="9" y="9" width="13" height="13" rx="2" ry="2"></rect>
            <path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"></path>
        </SvgWrapper>
    }
}

#[component]
pub fn IconEditA(
    #[prop(default = 20)] size: usize,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
) -> impl IntoView {
    view! {
        <SvgWrapper size attrs>
            <path d="M20 14.66V20a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V6a2 2 0 0 1 2-2h5.34"></path>
            <polygon points="18 2 22 6 12 16 8 16 8 12 18 2"></polygon>
        </SvgWrapper>
    }
}

#[component]
pub fn IconEditB(
    #[prop(default = 20)] size: usize,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
) -> impl IntoView {
    view! {
        <SvgWrapper size attrs>
            <polygon points="16 3 21 8 8 21 3 21 3 16 16 3"></polygon>
        </SvgWrapper>
    }
}

#[component]
pub fn IconEditC(
    #[prop(default = 20)] size: usize,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
) -> impl IntoView {
    view! {
        <SvgWrapper size attrs>
            <polygon points="14 2 18 6 7 17 3 17 3 13 14 2"></polygon>
            <line x1="3" y1="22" x2="21" y2="22"></line>
        </SvgWrapper>
    }
}

#[component]
pub fn IconFile(
    #[prop(default = 20)] size: usize,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
) -> impl IntoView {
    view! {
        <SvgWrapper size attrs>
            <path d="M13 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V9z"></path>
            <polyline points="13 2 13 9 20 9"></polyline>
        </SvgWrapper>
    }
}

#[component]
pub fn IconFilePlus(
    #[prop(default = 20)] size: usize,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
) -> impl IntoView {
    view! {
        <SvgWrapper size attrs>
            <path d="M20 11.08V8l-6-6H6a2 2 0 0 0-2 2v16c0 1.1.9 2 2 2h6"></path>
            <path d="M14 3v5h5M18 21v-6M15 18h6"></path>
        </SvgWrapper>
    }
}

#[component]
pub fn IconHelp(
    #[prop(default = 20)] size: usize,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
) -> impl IntoView {
    view! {
        <SvgWrapper size attrs>
            <circle cx="12" cy="12" r="10"></circle>
            <path d="M9.09 9a3 3 0 0 1 5.83 1c0 2-3 3-3 3"></path>
            <line x1="12" x2="12.01" y1="17" y2="17"></line>
        </SvgWrapper>
    }
}

#[component]
pub fn IconHome(
    #[prop(default = 20)] size: usize,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
) -> impl IntoView {
    view! {
        <SvgWrapper size attrs>
            <path d="M3 9l9-7 9 7v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z"></path>
            <polyline points="9 22 9 12 15 12 15 22"></polyline>
        </SvgWrapper>
    }
}

#[component]
pub fn IconLogin(
    #[prop(default = 20)] size: usize,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
) -> impl IntoView {
    view! {
        <SvgWrapper size attrs>
            <path d="M15 3h4a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2h-4"></path>
            <polyline points="10 17 15 12 10 7"></polyline>
            <line x1="15" x2="3" y1="12" y2="12"></line>
        </SvgWrapper>
    }
}

#[component]
pub fn IconLogout(
    #[prop(default = 20)] size: usize,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
) -> impl IntoView {
    view! {
        <SvgWrapper size attrs>
            <path d="M9 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h4"></path>
            <polyline points="16 17 21 12 16 7"></polyline>
            <line x1="21" x2="9" y1="12" y2="12"></line>
        </SvgWrapper>
    }
}

#[component]
pub fn IconMenu(
    #[prop(default = 20)] size: usize,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
) -> impl IntoView {
    view! {
        <SvgWrapper size attrs>
            <line x1="3" y1="12" x2="21" y2="12"></line>
            <line x1="3" y1="6" x2="21" y2="6"></line>
            <line x1="3" y1="18" x2="21" y2="18"></line>
        </SvgWrapper>
    }
}

#[component]
pub fn IconMinus(
    #[prop(default = 20)] size: usize,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
) -> impl IntoView {
    view! {
        <SvgWrapper size attrs>
            <line x1="5" x2="19" y1="12" y2="12"></line>
        </SvgWrapper>
    }
}

#[component]
pub fn IconPlus(
    #[prop(default = 20)] size: usize,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
) -> impl IntoView {
    view! {
        <SvgWrapper size attrs>
            <line x1="12" y1="5" x2="12" y2="19"></line>
            <line x1="5" y1="12" x2="19" y2="12"></line>
        </SvgWrapper>
    }
}

#[component]
pub fn IconSearch(
    #[prop(default = 20)] size: usize,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
) -> impl IntoView {
    view! {
        <SvgWrapper size attrs>
            <circle cx="11" cy="11" r="8"></circle>
            <line x1="21" y1="21" x2="16.65" y2="16.65"></line>
        </SvgWrapper>
    }
}

#[component]
pub fn IconSettings(
    #[prop(default = 20)] size: usize,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
) -> impl IntoView {
    view! {
        <SvgWrapper size attrs>
            <circle cx="12" cy="12" r="3"></circle>
            <path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z"></path>
        </SvgWrapper>
    }
}

#[component]
pub fn IconTrash(
    #[prop(default = 20)] size: usize,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
) -> impl IntoView {
    view! {
        <SvgWrapper size attrs>
            <polyline points="3 6 5 6 21 6"></polyline>
            <path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"></path>
            <line x1="10" y1="11" x2="10" y2="17"></line>
            <line x1="14" y1="11" x2="14" y2="17"></line>
        </SvgWrapper>
    }
}

#[component]
pub fn IconUser(
    #[prop(default = 20)] size: usize,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
) -> impl IntoView {
    view! {
        <SvgWrapper size attrs>
            <path d="M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2"></path>
            <circle cx="12" cy="7" r="4"></circle>
        </SvgWrapper>
    }
}

#[component]
pub fn IconUserMinus(
    #[prop(default = 20)] size: usize,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
) -> impl IntoView {
    view! {
        <SvgWrapper size attrs>
            <path d="M16 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2"></path>
            <circle cx="8.5" cy="7" r="4"></circle>
            <line x1="23" y1="11" x2="17" y2="11"></line>
        </SvgWrapper>
    }
}

#[component]
pub fn IconUserPlus(
    #[prop(default = 20)] size: usize,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
) -> impl IntoView {
    view! {
        <SvgWrapper size attrs>
            <path d="M16 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2"></path>
            <circle cx="8.5" cy="7" r="4"></circle>
            <line x1="20" y1="8" x2="20" y2="14"></line>
            <line x1="23" y1="11" x2="17" y2="11"></line>
        </SvgWrapper>
    }
}

#[component]
pub fn IconUsers(
    #[prop(default = 20)] size: usize,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
) -> impl IntoView {
    view! {
        <SvgWrapper size attrs>
            <path d="M17 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2"></path>
            <circle cx="9" cy="7" r="4"></circle>
            <path d="M23 21v-2a4 4 0 0 0-3-3.87"></path>
            <path d="M16 3.13a4 4 0 0 1 0 7.75"></path>
        </SvgWrapper>
    }
}

#[component]
pub fn MoreHorizontal(
    #[prop(default = 20)] size: usize,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
) -> impl IntoView {
    view! {
        <SvgWrapper size attrs>
            <circle cx="12" cy="12" r="1"></circle>
            <circle cx="19" cy="12" r="1"></circle>
            <circle cx="5" cy="12" r="1"></circle>
        </SvgWrapper>
    }
}
