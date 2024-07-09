use leptos::*;

use uuid::Uuid;

use super::model::RequestUser;

#[derive(Debug, Copy, Clone)]
pub struct RequestUserContext {
    pub user: RwSignal<RequestUser>,
}

impl RequestUserContext {
    pub fn new() -> Self {
        Self {
            user: RwSignal::new(RequestUser::default()),
        }
    }
    pub fn is_authenticated(&self) -> bool {
        self.user.with(|user| user.is_active)
    }
    pub fn is_staff(&self) -> bool {
        self.user.with(|user| user.is_staff)
    }
    pub fn is_superuser(&self) -> bool {
        self.user.with(|user| user.is_superuser)
    }
    pub fn is_superuser_or_object_owner(&self, user_id: Uuid) -> bool {
        self.user
            .with(|user| user.is_superuser || user.id == user_id)
    }
    pub fn is_object_owner(&self, username: String) -> bool {
        self.user.with(|user| user.username == username)
    }
    pub fn is_not_object_owner(&self, username: String) -> bool {
        self.user.with(|user| user.username != username)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct CanEditContext {
    pub can_edit: RwSignal<bool>,
}

impl CanEditContext {
    pub fn new() -> Self {
        Self {
            can_edit: RwSignal::new(false),
        }
    }
    pub fn cant_edit(&self) -> bool {
        self.can_edit.with(|v| *v == false)
    }
}
