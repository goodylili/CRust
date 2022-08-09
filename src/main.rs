#![feature(decl_macro)]

use rocket::*;
use rocket_contrib::json::Json;
use serde::Deserialize;
use serde::Serialize;

//Configs to make struct serializable and debug able
#[derive(Deserialize, Serialize)]
struct BioData {
    age: i32,
    username: String,
}

fn vec_db() -> Vec<BioData> {
    let vec_store: Vec<BioData> = Vec::new();
    return vec_store;
}


#[post("/todo", format = "json", data = "<bio>")]
fn create_bio(bio: Json<BioData>) -> Json<BioData> {
    let person = BioData {
        age: bio.age,
        username: bio.username.to_string(),
    };
    vec_db().push(person);

    bio
}


#[get("/hello")]
fn hello() -> Json<&'static str> {
    Json("{'status' : 'successful',
'message`: 'Your first Rocket app is live'}")
}


fn main() {
    rocket::ignite()
        .mount("/api", routes![create_bio, hello])
        .launch();
}

