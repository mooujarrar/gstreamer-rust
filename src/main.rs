#[macro_use] extern crate rocket;

// import tutorial_main from gstreamer_test
use crate::gstreamer_test::tutorial_main;
// Declare the module gstreamer_test in the crate root
pub mod gstreamer_test;

#[get("/")]
fn index() -> &'static str {
    "Hello, Rust!!"
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    tutorial_main();
    let _rocket = rocket::build()
        .mount("/", routes![index])
        .launch()
        .await?;
    Ok(())
}
