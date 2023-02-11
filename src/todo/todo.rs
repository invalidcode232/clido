use serde::{Deserialize, Serialize};
use tabled::Tabled;

#[derive(Tabled, Deserialize, Serialize, Clone, Debug)]
pub struct Todo {
    pub index: i32,
    pub todo: String,
    pub date_added: String,
    pub done: bool,
}
