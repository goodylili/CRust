#![feature(decl_macro)]
use rocket::*;
use rocket::response::content::Json;


#[derive(FromForm, Debug)]
struct Book {
    first_name: String,
    last_name: String,
    phone: i32,
}





fn main() {
    rocket::ignite()
        .mount("/api", routes![hello])
        .launch();
}


// git add .
// git commit -m changes
// git push origin main