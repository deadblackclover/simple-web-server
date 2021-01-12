use clap::{App, Arg};
use futures::prelude::*;
use gotham::handler::assets::FileOptions;
use gotham::handler::HandlerFuture;
use gotham::helpers::http::response::create_empty_response;
use gotham::hyper::{body, Body, HeaderMap, Method, StatusCode, Uri, Version};
use gotham::router::builder::*;
use gotham::state::{FromState, State};
use std::pin::Pin;

fn log_request(state: &State) {
    let method = Method::borrow_from(state);
    let uri = Uri::borrow_from(state);
    let http_version = Version::borrow_from(state);
    let headers = HeaderMap::borrow_from(state);

    println!("\n[{:?}] {:?}", method, uri);
    println!("{:?}", http_version);
    println!("Headers: {:?}", headers);
}

fn request_handler(mut state: State) -> Pin<Box<HandlerFuture>> {
    log_request(&state);

    let f = body::to_bytes(Body::take_from(&mut state)).then(|full_body| match full_body {
        Ok(valid_body) => {
            let body_content = String::from_utf8(valid_body.to_vec()).unwrap();
            println!("Body: {}", body_content);
            let res = create_empty_response(&state, StatusCode::OK);
            future::ok((state, res))
        }
        Err(e) => future::err((state, e.into())),
    });

    Box::pin(f.boxed())
}

fn main() {
    let matches = App::new("wserver")
        .version("0.1.3")
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
        .arg(
            Arg::with_name("threads")
                .short("t")
                .long("threads")
                .value_name("NUMBER")
                .help("Number of threads")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("logs")
                .short("l")
                .long("logs")
                .help("Logging requests"),
        )
        .get_matches();

    let path = matches.value_of("path").unwrap_or("./");
    let ip = matches.value_of("address").unwrap_or("127.0.0.1");
    let port = matches.value_of("port").unwrap_or("8080");
    let threads = matches.value_of("threads").unwrap_or("1");

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

        if matches.occurrences_of("logs") > 0 {
            route.delete("/").to(request_handler);
            route.delete("/*").to(request_handler);

            route.head("/").to(request_handler);
            route.head("/*").to(request_handler);

            route.options("/").to(request_handler);
            route.options("/*").to(request_handler);

            route.patch("/").to(request_handler);
            route.patch("/*").to(request_handler);

            route.post("/").to(request_handler);
            route.post("/*").to(request_handler);

            route.put("/").to(request_handler);
            route.put("/*").to(request_handler);
        }
    });

    let addr = [ip, port].join(":");
    println!("Listening for requests at http://{}", addr);

    gotham::start_with_num_threads(
        addr,
        router,
        threads.parse::<usize>().expect("Parsing threads error!"),
    )
}
