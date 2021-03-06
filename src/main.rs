#![feature(plugin)]
#![plugin(rocket_codegen)]
#![feature(custom_derive)]

extern crate rocket;
extern crate hyper;

//use rocket::request::FromForm;
use rocket::config::{Config, Environment};
use rocket::response::Redirect;
use rocket::response::content;
//use rocket::http::RawStr;

use std::env;
use std::io::Read;
use std::path::PathBuf;

use hyper::Client;
use hyper::status::StatusCode;
//use hyper::client::response::Response;
use hyper::header::{Headers, ContentType, Accept, UserAgent, Location};

#[derive(FromForm)]
struct Query {
    q: String
}

fn do_lookup(term: String) -> Result<content::HTML<String>, Redirect> {
    let mut client = Client::new();
    client.set_redirect_policy(hyper::client::RedirectPolicy::FollowNone);

    let body = format!("s={}&submit.x=0&submit.y=0", term);
    //println!("body = {}", body);

    let mut headers = Headers::new();
     
    headers.set(UserAgent("curl/7.51.0 ".to_owned()));
    headers.set(Accept::star());
    headers.set(ContentType::form_url_encoded());
    let mut res = client.post("http://airnav.com/airports/get")
        .body(body.as_str()).headers(headers)
        .send().unwrap();

    //println!("Headers: \n{}", res.headers);
    //println!("Status: \n{}", res.status);

    let mut buffer = String::new();
    res.read_to_string(&mut buffer).unwrap();

    match res.status {
        StatusCode::Ok => {
            Ok(content::HTML(buffer))
        }
        StatusCode::MovedPermanently => {
            let header = res.headers.get::<Location>().unwrap();
            Err(Redirect::to(header))
        },
        _ => Err(Redirect::to("https://airnav.com/airports"))
    }
}

#[get("/<term..>")]
fn index(term: Option<PathBuf>) ->  Result<content::HTML<String>, Redirect> {
    match term {
        Some(q) => do_lookup(String::from(q.to_str().unwrap())),
        None => do_lookup(String::new()) 
    }
}

#[get("/?<q>")]
fn query_params(q: Query) ->  Result<content::HTML<String>, Redirect> {
    do_lookup(q.q) 
}

fn main() {
    let app = match env::var("PORT") {
        Ok(ports) => {
            let port: u16 = ports.parse().unwrap();
            let config = Config::build(Environment::Staging)
                .address("0.0.0.0")
                .port(port)
                .finalize().unwrap();

            rocket::custom(config, false)
        },
        Err(_) => rocket::ignite()
    };
    app.mount("/", routes![query_params]).mount("/", routes![index]).launch();
}
