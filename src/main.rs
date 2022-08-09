#![feature(decl_macro)]

use rocket::*;
use rocket::http::RawStr;
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
    let mut vec_store: Vec<BioData> = Vec::new();
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

#[get("/person/<name>")]
fn get_bio(name: &RawStr) {
    for structs in &vec_db() {
        if structs.username == name.to_string() {

        }
    }

}

#[put("/person/<age>")]
fn update_bio(age: i32) ->  {


}


#[delete("/person/<age>")]
fn delete_bio(age: i32 ) ->  {


}




fn main() {
    let mut vec_store: Vec<BioData> = Vec::new();

    rocket::ignite()
        .mount("/api", routes![create_bio, get_bio])
        .launch();
}

