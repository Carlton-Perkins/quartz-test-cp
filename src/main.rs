use std::path::PathBuf;

use rocket::fs::NamedFile;
use rocket::get;
use rocket::routes;

#[shuttle_runtime::main]
async fn main() -> shuttle_rocket::ShuttleRocket {
    let rocket = rocket::build().mount(
        "/blog",
        routes![file_server], // FileServer::new(relative!("quartz/public"), Options::Index),
    );

    Ok(rocket.into())
}

#[get("/<file..>")]
pub async fn file_server(file: PathBuf) -> Option<NamedFile> {
    let mut path = PathBuf::from("quartz/public").join(file);
    if path.is_dir() {
        path.push("index.html");
    }

    println!("try path: {:?}", path);
    if let Some(file) = NamedFile::open(path.clone()).await.ok() {
        return Some(file);
    }

    path.set_extension("html");
    println!("try path: {:?}", path);
    if let Some(file) = NamedFile::open(path).await.ok() {
        return Some(file);
    }

    let path = PathBuf::from("quartz/public/index.html");
    if let Some(file) = NamedFile::open(path).await.ok() {
        return Some(file);
    }

    None
}
