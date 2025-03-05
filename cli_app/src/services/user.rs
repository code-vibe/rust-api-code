use std::sync::Mutex;

pub struct InMemoryUserService {
    pub counter : i64,
    pub items : HashMap<i64, User>,
}

pub struct InMemoryUserService {
    data: Mutex<InMemoryUserService>,
}

impl default for InMemoryUserService {
    fn default() -> Self {
       Self {
           data: Mutex::new(InMemoryUserService {
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