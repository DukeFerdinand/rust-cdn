#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
use rocket::response::NamedFile;
use rocket::Request;
use std::path::{Path, PathBuf};

#[get("/static/<file..>")]
pub fn file(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

#[catch(404)]
pub fn not_found(req: &Request) -> String {
    let req_str = format!("File '{}' not found", req.uri().path());
    println!("{}", req.client_ip().unwrap());
    req_str
}

fn main() {
    rocket::ignite()
        .register(catchers![not_found])
        .mount("/", routes![file])
        .launch();
}
