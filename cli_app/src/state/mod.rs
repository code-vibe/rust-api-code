use crate::settings::Settings;
use std::sync::Arc;
use arc_swap::{ArcSwap};
use crate::services::post::InMemoryPostService;
use crate::services::user::InMemoryUserService;

pub struct ApplicationState {
    pub settings: ArcSwap<Settings>,
    pub user_service: Arc<InMemoryUserService>,
    pub post_service : Arc<InMemoryPostService>,
}

impl ApplicationState {
    pub fn new(settings: &Settings) -> anyhow::Result<Self> {
        Ok(Self {
            settings: ArcSwap::new(Arc::new((*settings).clone())),
            user_service: Arc::new(InMemoryUserService::default()),
            post_service: Arc::new(InMemoryPostService::default()),
        })
    }
}