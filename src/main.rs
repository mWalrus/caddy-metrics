#[macro_use]
extern crate rocket;
mod matchers;
mod parser;
mod registry;
mod routes;

#[launch]
fn rocket() -> _ {
    let config = rocket::Config {
        port: 8001,
        ..Default::default()
    };

    let registry = registry::init();
    let matchers = matchers::init();
    let log_queue = parser::watch().unwrap();

    rocket::custom(config)
        .register("/", catchers![routes::default])
        .manage(registry)
        .manage(matchers)
        .manage(log_queue)
        .mount("/", routes![routes::index, routes::metrics])
}
