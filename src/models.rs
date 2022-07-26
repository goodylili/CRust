use diesel::prelude::*;


#[derive(Queryable)]
pub struct People {
    pub name: String,
    pub id: i32
}