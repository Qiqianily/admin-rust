use sqlx::{MySql, Pool};
use std::ops::Deref;
use std::sync::Arc;

#[allow(unused)]
#[derive(Debug, Clone)]
pub struct AppState {
    pub inner: Arc<AppStateInner>,
    pub pool: Pool<MySql>,
}

// AppStateInner is a struct that holds any application-specific state that needs to be shared
// across multiple requests.
impl AppState {
    pub fn new(pool: Pool<MySql>) -> Self {
        Self {
            inner: Arc::new(AppStateInner {}),
            pool,
        }
    }
}
// Deref allows us to access the inner AppStateInner struct from the AppState struct.
impl Deref for AppState {
    type Target = AppStateInner;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[allow(unused)]
#[derive(Debug, Clone)]
pub struct AppStateInner {}
