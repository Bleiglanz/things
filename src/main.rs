#![feature(proc_macro_hygiene, decl_macro, custom_attribute)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate rocket_contrib;


extern crate serde_json;
extern crate dotenv;

use diesel::prelude::*;
use diesel::pg::PgConnection;

use rocket::Request;
use rocket_contrib::templates::Template;
use rocket_contrib::databases::diesel;
use std::collections::HashMap;


#[derive(Serialize)]
struct TemplateContext {
    name: String,
}



#[get("/users/<sid>")]
fn users(conn:DbConn, sid:i32) -> Template {
    //let res = schema::user.filter(id.eq(sid)).load::<User>(&conn);
    let context = TemplateContext { name : "alle Benutzer".to_string() + &sid.to_string() };
    Template::render("index",&context)
}


#[get("/")]
fn index() -> Template {
    let context = TemplateContext { name : "Hallo Welt".to_string() };
    Template::render("index",&context)
}


#[catch(404)]
fn not_found(req: &Request) -> Template {
    let mut map = HashMap::new();
    map.insert("path", req.uri().path());
    Template::render("error/404", &map)
}

#[database("thingsdb")]
struct DbConn(diesel::PgConnection);

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![index,users])
        .attach(DbConn::fairing())
        .attach(Template::fairing())
        .register(catchers![not_found])
}

fn main(){
    rocket().launch();
}
