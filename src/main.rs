#![feature(proc_macro_hygiene, decl_macro)]
mod eve_requests;

#[macro_use] extern crate rocket;
extern crate dotenv;

use std::fs::File;
use std::io::prelude::*;
use time::{ OffsetDateTime, Duration };

pub fn store_auth_data(new_auth: &json::JsonValue, tg_id: &str, character_name: &json::JsonValue) {
    let db_name = "auth.json";
    let mut db: json::JsonValue = match File::open(db_name) {
        Ok(mut file) => {
            let mut contents = String::new();
            file.read_to_string(&mut contents).expect("Auth file read");
            json::parse(&contents).expect("Auth file parse")
        },
        Err(_) => {
            json::JsonValue::new_array()
        }
    };
    let mut insertion: json::JsonValue = new_auth.clone();
    insertion["tg_id"] = (*tg_id).into();
    insertion["character_name"] = (*character_name).clone();
    let life_time = Duration::new(insertion["expires_in"].as_u32().expect("Login responce unexpected format").into(), 0);
    insertion["expires_in"] = (OffsetDateTime::now() + life_time).timestamp().into();
    insertion.remove("token_type");
    db.push(insertion).expect("Auth db format error");
    let mut output_file = File::create(db_name).expect(&format!("Can't open for writing file {}", db_name));
    output_file.write_all(db.pretty(2).as_bytes()).expect(&format!("Can't write db to file {}", db_name));
}

#[get("/?<code>&<state>")]
fn index(code: String, state: i64) -> std::result::Result<rocket_contrib::templates::Template, rocket::http::Status> {
    match eve_requests::login(&code) {
        Err(problem) => {
            problem.dump();
            Err(rocket::http::Status::NotAcceptable)
        }
        Ok(responce) => {
            let toketn: String = String::from(responce["access_token"].as_str().expect("Login access_token must be a string"));
            let character_info = eve_requests::verify(&toketn).expect("Login responce json parse");
            store_auth_data(&responce, &format!("{}", state), &character_info["CharacterName"]);
            let mut context = std::collections::HashMap::new();
            let character_name: String = character_info["CharacterName"].as_str().expect("wrong character format").to_string();
            context.insert("character_name".to_string(), &character_name);
            Ok(rocket_contrib::templates::Template::render("authorized", &context))
        }
    }
}

use rocket::response::NamedFile;

#[get("/favicon.png")]
fn favicon() -> Option<NamedFile> {
    let path = std::path::Path::new("favicon.png");
    NamedFile::open(&path).ok()
}

fn main() {
    dotenv::dotenv().ok();
    rocket::ignite()
        .attach(rocket_contrib::templates::Template::fairing())
        .mount("/", routes![index, favicon])
        .launch();
}
