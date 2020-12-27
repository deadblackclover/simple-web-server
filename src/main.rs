use clap::{App, Arg};
use gotham::handler::assets::FileOptions;
use gotham::router::builder::*;

fn main() {
    let matches = App::new("wserver")
        .version("0.1.0")
        .author("DEADBLACKCLOVER <deadblackclover@protonmail.com>")
        .about("Simple web server powered by Rust")
        .arg(
            Arg::with_name("path")
                .long("path")
                .value_name("PATH")
                .help("File path")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("address")
                .short("a")
                .long("address")
                .value_name("IP")
                .help("Server IP address")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("port")
                .short("p")
                .long("port")
                .value_name("PORT")
                .help("Server port")
                .takes_value(true),
        )
        .get_matches();

    let path = matches.value_of("path").unwrap_or("./");
    let ip = matches.value_of("address").unwrap_or("127.0.0.1");
    let port = matches.value_of("port").unwrap_or("8080");

    let index = if path.ends_with("/") {
        format!("{}index.html", path)
    } else {
        format!("{}/index.html", path)
    };

    let router = build_simple_router(|route| {
        route.get("/").to_file(&index);

        route.get("/*").to_dir(
            FileOptions::new(&path)
                .with_cache_control("no-cache")
                .with_gzip(true)
                .build(),
        );
    });

    let addr = [ip, port].join(":");
    println!("Listening for requests at http://{}", addr);

    gotham::start(addr, router)
}
