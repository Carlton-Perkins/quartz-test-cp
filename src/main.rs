use rocket::fs::relative;
use rocket::fs::FileServer;
use rocket::fs::Options;

#[shuttle_runtime::main]
async fn main() -> shuttle_rocket::ShuttleRocket {
    let rocket = rocket::build().mount(
        "/blog",
        FileServer::new(relative!("quartz/public"), Options::Index),
    );

    Ok(rocket.into())
}
