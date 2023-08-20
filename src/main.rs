#[macro_use]
extern crate rocket;
mod matchers;
mod parser;
mod registry;
mod routes;

use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about)]
struct Args {
    #[clap(long, short, help = "Absolute path to log file", required = true)]
    file: PathBuf,
}

#[launch]
fn rocket() -> _ {
    let config = rocket::Config {
        port: 8001,
        ..Default::default()
    };

    let args = Args::parse();

    let registry = registry::init();
    let matchers = matchers::init();
    let log_queue = parser::watch(args.file).unwrap();

    rocket::custom(config)
        .register("/", catchers![routes::default])
        .manage(registry)
        .manage(matchers)
        .manage(log_queue)
        .mount("/", routes![routes::index, routes::metrics])
}
