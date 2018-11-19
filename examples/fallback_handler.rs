#![feature(async_await)]

fn fb_handler() -> tide::Response {
    let test = http::Response::builder()
        .status(http::status::StatusCode::NotFound)
        .body("Route not implemented!")
        .unwrap();
}

fn main() {
    let mut app = tide::App::new(());
    app.at("/").get(async || "Hello, world!");
    app.fallback_handler(fb_handler);
    app.serve("127.0.0.1:7878")
}
