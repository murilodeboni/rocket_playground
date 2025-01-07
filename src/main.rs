#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/world")]              // <- route attribute
fn world() -> &'static str {  // <- request handler
    "hello, worldsadasd!"
}

//A Rocket route attribute can be any one of get, put, post, delete, head, patch, or options, each corresponding to the HTTP method to match against. For example, the following attribute will match against POST requests to the root path:
// #[post("/")]
// ...

#[get("/hello/<name>")]
fn hello(name: &str) -> String {
    format!("Hello, {}!", name)
}



//#[launch]
//fn rocket() -> _ {
//    rocket::build()
//        .mount("/", routes![index])
//        .mount("/test", routes![world]) // creates /test/world:
//}

// for handling async
#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .mount("/hello", routes![world])
        .mount("/", routes![hello])
        .launch()
        .await?;

    Ok(())
}
