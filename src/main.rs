#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate ruscall;
extern crate actix_web;

use ruscall::compile::compile_from_str;
use std::env;
use actix_web::{Form, server, App, HttpRequest, Responder, HttpResponse,Path};
use actix_web::http::Method;
use std::fs;
use std::io::{BufReader, Read};
use std::io::prelude::*;

fn index(_req: &HttpRequest) -> impl Responder {
    let body = include_str!("../front/dist/index.html");
    HttpResponse::Ok()
        .content_type("text/html")
        .body(body)
}

fn text(info:Path<(String,String)>) -> impl Responder {
    let file_name=format!("front/dist/{}/{}",info.0,info.1);
    let mut f = BufReader::new(fs::File::open(file_name).unwrap());
    let mut body: String = "".to_string();
    f.read_to_string(&mut body).unwrap();
    HttpResponse::Ok()
        .content_type("text/plain")
        .body(body)
}

fn img(info:Path<(String,String)>) -> impl Responder {
    let file_name=format!("front/dist/{}/{}",info.0,info.1);
    let mut buffer = BufReader::new(fs::File::open(file_name).unwrap());
    let bytes=Vec::from(buffer.fill_buf().unwrap());
    HttpResponse::Ok()
        .content_type("text/plain")
        .body(bytes)
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
                .route("/{type:css|js}/{name}", Method::GET,text)
                .route("/{type:img}/{name}", Method::GET,img)
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