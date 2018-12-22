#![feature(proc_macro_hygiene, decl_macro)]



use std::collections::HashMap;
use rocket::Request;
use rocket::response::Redirect;
use rocket_contrib::templates::Template;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;


#[derive(Serialize)]
struct TemplateContext {
    name: String,
}

#[get("/")]
fn index() -> Template {
    let context = TemplateContext { name : "Hallo Welt".to_string() };
    Template::render("index",&context)
}

#[get("/users")]
fn index() -> Template {
    let context = TemplateContext { name : "alle Benutzer".to_string() };
    Template::render("index",&context)
}


#[catch(404)]
fn not_found(req: &Request) -> Template {
    let mut map = HashMap::new();
    map.insert("path", req.uri().path());
    Template::render("error/404", &map)
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![index])
        .attach(Template::fairing())
        .register(catchers![not_found])
}

fn main(){
    let _conn = establish_connection();
    rocket().launch();
}
