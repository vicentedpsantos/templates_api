use super::schema::templates;
use serde::{Deserialize, Serialize};

#[derive(Debug, Queryable, Serialize, Deserialize, AsChangeset)]
pub struct Template {
    pub id: i32,
    pub name: String,
    pub content: String,
    #[serde(skip_deserializing)]
    pub created_at: String,
}

#[derive(Insertable, Deserialize)]
#[table_name = "templates"]
pub struct NewTemplate {
    pub name: String,
    pub content: String,
}
