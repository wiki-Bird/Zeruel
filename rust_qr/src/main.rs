#[macro_use] extern crate rocket;

use rocket::fs::FileServer;
use rocket_dyn_templates::Template;
use std::collections::HashMap;
use rocket::response::Redirect;
use rocket::form::Form;

#[derive(FromForm)]
pub struct UrlForm {
    big_url: String,
}


#[get("/")]
fn index() -> Template {
    Template::render("index", &HashMap::<String, String>::new())
}

#[get("/<short_url>")]
fn redirect_url(short_url: String) -> Template {
    let mut context = HashMap::new();
    context.insert("short_url", short_url);

    Template::render("short_url", &context)
}

#[post("/", data = "<url_form>")]
fn post_url(url_form: Form<UrlForm>) -> Redirect {
    Redirect::to(format!("/submit/{}", url_form.big_url))
}

#[get("/submit/<url>")]
fn submit_url(url: String) -> Template {
    let mut context = HashMap::new();
    context.insert("big_url", url);
    Template::render("big_url", &context)
}


#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, redirect_url, post_url, submit_url])
        .mount("/static", FileServer::from("static"))
        .attach(Template::fairing())
}
