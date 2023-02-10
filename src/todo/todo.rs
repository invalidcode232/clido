use serde::{Deserialize, Serialize};
use tabled::Tabled;

#[derive(Tabled, Deserialize, Serialize, Clone)]
pub struct Todo {
    pub todo: String,
    pub date_added: String,
}
