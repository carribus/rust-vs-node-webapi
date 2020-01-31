#[macro_use]
extern crate actix_web;

use actix_web::{web, App, HttpServer};

#[get("/")]
fn index() -> &'static str {
    r#"Rust-vs-node benchmark<br/>Rust-server benchmark server"#
}

fn fib(path: web::Path<(i64,)>) -> String {
    let mut a:i64 = 1;
    let mut b = 0;
    let mut length = path.0;
    
    while length >= 0 {
        let temp = a;
        a = a + b;
        b = temp;
        length -= 1;
    }

    format!("{}", b)
}

fn main() {
    HttpServer::new(move || {
        App::new()
            .service(index)
            .service(web::resource("/fib/{length}").route(web::get().to(fib)))
    })
    .bind("localhost:3001").unwrap()
    .run().unwrap()
}
