use config::{Config, File};

use gotham::handler::assets::FileOptions;
use gotham::router::builder::*;

fn main() {
    let mut s = Config::new();
    s.merge(File::with_name("config")).unwrap();

    let path = s.get::<String>("path").unwrap();

    let router = build_simple_router(|route| {
        route.get("/*").to_dir(
            FileOptions::new(&path)
                .with_cache_control("no-cache")
                .with_gzip(true)
                .build(),
        );
    });

    let ip = s.get::<String>("ip").unwrap();
    let port = s.get::<String>("port").unwrap();

    let addr = [ip, port].join(":");
    println!("Listening for requests at http://{}", addr);

    gotham::start(addr, router)
}
