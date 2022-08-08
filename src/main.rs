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

#[get("/hello")]
fn hello() -> Json<&'static str> {
    Json("{'status' : 'successful',
'message`: 'Your first Rocket app is live'}")
}



#[post("/todo", format = "json", data = "<bio>")]
fn create_bio(bio: Json<BioData>) -> String {
    format!("{:?}",bio)

}


fn main() {
    rocket::ignite()
        .mount("/api", routes![create_bio, hello])
        .launch();
}

//

