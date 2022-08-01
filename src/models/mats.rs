use crate::schema::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Material {
    pub id: i32,
    pub item: String,
    pub density: f32,
}

#[derive(Insertable, Debug)]
#[table_name = "materials"]
pub struct NewMaterial<'a> {
    pub item: &'a str,
    pub density: &'a f32,
}