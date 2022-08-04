#![feature(decl_macro)]
use rocket::*;
use rocket::response::content::Json;


#[derive(FromForm, Debug)]
struct Person {
    first_name: String,
    last_name: String,
    phone: i32,
}

#[get("api/get", "data = <name>")]
fn get_person() {
    let a_person = Person{
        first_name: String::from("Jeffery"),
        last_name: String::from("PenSteak"),
        phone: 193349180
    };
    Template::render("")

}

#[get("api/get", "data = <name>")]
fn post_person() {
    let a_person = Person{
        first_name: String::from("Jeffery"),
        last_name: String::from("PenSteak"),
        phone: 193349180
    };
    Template::render("")

}




fn main() {
    rocket::ignite()
        .mount("/api", routes![hello])
        .launch();
}

#[catch(404)]
fn not_found(req: &Request) -> String {
    print!("{}", req);
    format!("Oh no! We couldn't find the requested path '{}'", req.uri())
}

// git add .
// git commit -m changes
// git push origin main