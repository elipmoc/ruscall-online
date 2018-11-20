#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate ruscall;
extern crate actix_web;

use ruscall::compile::compile_from_str;
use std::env;
use actix_web::{Form, server, App, HttpRequest, Responder, HttpResponse};
use actix_web::http::Method;

fn index(_req: &HttpRequest) -> impl Responder {
    let body = include_str!("../html/index.html");
    HttpResponse::Ok()
        .content_type("text/html")
        .body(body)
}

#[derive(Deserialize)]
struct Info {
    code: String,
}

fn compile(info: Form<Info>) -> impl Responder {
    let result = compile_from_str(&info.code, "foobar");
    let body =
        if let Err(err) = result {
            err
        } else {
            let current_dir = env::current_dir().unwrap().to_str().unwrap().to_string();

            if cfg!(target_os = "windows") {
                command_exec("cmd", &[
                    "/C",
                    &(current_dir + "\\compiled.exe")
                ])
            } else {
                command_exec("sh", &[
                    "-c",
                    &(current_dir + "/compiled.out")
                ])
            }
        };

    HttpResponse::Ok()
        .content_type("text/plain")
        .body(body)
}

fn main() {
    let port = env::var("PORT").unwrap_or("3000".to_string());
    println!("serving on http://localhost:{}...", port);
    server::new(
        ||
            App::new()
                .resource("/", |r| r.method(Method::GET).f(index))
                .resource("/compile", |r| r.method(Method::POST).with(compile))
    )
        .bind(format!("0.0.0.0:{}", port))
        .unwrap()
        .run();
}


use std::ffi::OsStr;

fn command_exec<I, S>(terminal: &str, args: I) -> String
    where
        I: IntoIterator<Item=S> + Clone,
        S: AsRef<OsStr>,
{
    use std::process::Command;

    let output = Command::new(terminal)
        .args(args.clone())
        .output()
        .expect("failed to execute process");
    format!("status: {} \nstdout: {}stderr: {}",
            output.status,
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr))
}