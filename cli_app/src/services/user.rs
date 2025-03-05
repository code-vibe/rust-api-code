use std::collections::HashMap;
use std::sync::Mutex;
use crate::model::User;

pub struct InMemoryUserStore {
    pub counter : i64,
    pub items : HashMap<i64, User>,
}

pub struct InMemoryUserService {
    data: Mutex<InMemoryUserStore>,
}

impl Default for InMemoryUserService {
    fn default() -> Self {
       Self {
           data: Mutex::new(InMemoryUserStore {
               counter : 0,
               items: Default::default(),
           }),
       }
    }
}

impl UserService for InMemoryUserService {

}
#[allow(async_fn_in_trait)]
pub trait UserService {

}