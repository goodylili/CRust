#![feature(decl_macro)]
use rocket::*;
use serde::Deserialize;
use rocket_contrib::json::Json;

//Configs to make struct serializable and debug able
#[derive(Deserialize, Debug)]
pub struct BioData {
    pub name: String,
    pub age: i32,
}



#[post("/todo", data = "<task>")]
fn create_bio(task: Json<BioData>) {



}


fn main() {
    rocket::ignite()
        .mount("/api", routes![create_bio])
        .launch();
}

