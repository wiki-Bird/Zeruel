#[macro_use] extern crate rocket;

use rocket::fs::FileServer;
use rocket_dyn_templates::Template;
use std::collections::HashMap;

#[get("/")]
fn index() -> Template {
    Template::render("index", &HashMap::<String, String>::new())
}

#[get("/<short_url>")]
fn get_big_url(short_url: String) -> Template {
    let mut context = HashMap::new();
    context.insert("short_url", short_url);

    Template::render("short_url", &context)
}

#[post("/?<big_url>")]
fn post_url(big_url: String) -> Template {
    let mut context = HashMap::new();
    context.insert("big_url", big_url);

    Template::render("big_url", &context)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, get_big_url, post_url])
        .mount("/static", FileServer::from("static"))
        .attach(Template::fairing())
}
