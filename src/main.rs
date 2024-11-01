#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Welcome to my portfolio!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}