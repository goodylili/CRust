#![feature(decl_macro)]
use rocket::*;
use serde::Deserialize;
use serde::Serialize;

use rocket_contrib::json::Json;



//Configs to make struct serializable and debug able
#[derive(Deserialize, Serialize)]
struct BioData {
    age: i32,
    username: String,
}



#[post("/todo", format = "json", data = "<bio>")]
fn create_bio(bio: Json<BioData>) -> Json<BioData> {
    let person = BioData{
        age: bio.age,
        username: bio.username.to_string(),
    };

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

