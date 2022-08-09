#![feature(decl_macro)]
use rocket::*;
use serde::Deserialize;
use serde::Serialize;

use rocket_contrib::json::Json;



//Configs to make struct serializable and debug able
#[derive(Deserialize, Serialize)]
pub struct BioData {
    pub name: String,
    pub age: i32,
}



#[post("/todo", format = "json", data = "<bio>")]
fn create_bio(bio: Json<BioData>) -> Json<BioData> {
    let person = BioData{
        name: bio.name,
        age: bio.age
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

