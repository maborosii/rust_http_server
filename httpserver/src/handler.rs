use http::httprequest::{self, Resource};
use http::{httprequest::HttpRequest, httpresponse::HttpResponse};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::fs;
use std::ops::Index;

pub trait Handler {
    fn handle(req: &HttpRequest) -> HttpResponse;
    fn load_file(file_name: &str) -> Option<String> {
        let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
        let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
        let full_path = format!("{}/{}", public_path, file_name);

        let contents = fs::read_to_string(full_path);
        // result -> option
        contents.ok()
    }
}

pub struct StaticPageHandler;
pub struct WebServiceHandler;
pub struct PageNotFoundHandler;

// cargo toml active serialize and deserialize derive
#[derive(Serialize, Deserialize)]
pub struct OrderStatus {
    order_id: i32,
    order_date: String,
    order_status: String,
}

impl Handler for PageNotFoundHandler {
    fn handle(req: &HttpRequest) -> HttpResponse {
        HttpResponse::new("404", None, Self::load_file("404.html"))
    }
}

impl Handler for StaticPageHandler {
    fn handle(req: &HttpRequest) -> HttpResponse {
        let Resource::Path(s) = &req.resource;
        let route: Vec<&str> = s.split('/').collect();
        match route[1] {
            "" => HttpResponse::new("200", None, Self::load_file("index.html")),
            "health" => HttpResponse::new("200", None, Self::load_file("health.html")),
            path => match Self::load_file(path) {
                Some(contents) => {
                    let mut m = HashMap::new();
                    if path.ends_with(".css") {
                        m.insert("Content-Type", "test/css");
                    } else if path.ends_with(".js") {
                        m.insert("Content-Type", "test/js");
                    } else {
                        m.insert("Content-Type", "test/html");
                    }
                    HttpResponse::new("200", Some(m), Some(contents))
                }
                None => HttpResponse::new("404", None, Self::load_file("404.html")),
            },
        }
    }
}

impl WebServiceHandler {
    fn load_json() -> Vec<OrderStatus> {
        let default_path = format!("{}/data", env!("CARGO_MANIFEST_DIR"));
        let public_path = env::var("DATA_PATH").unwrap_or(default_path);
        let full_path = format!("{}/{}", public_path, "orders.json");
        let json_contents = fs::read_to_string(full_path);
        let orders = serde_json::from_str(json_contents.unwrap().as_str()).unwrap();
        orders
    }
}

impl Handler for WebServiceHandler {
    fn handle(req: &HttpRequest) -> HttpResponse {
        let Resource::Path(s) = &req.resource;
        let route = s.split('/').collect::<Vec<&str>>();
        match route[2] {
            "air" if route.len() > 2 && route[3] == "orders" => {
                // TODO serde_json::to_string 待研究
                let body = Some(serde_json::to_string(&Self::load_json()).unwrap());
                let mut m = HashMap::new();
                m.insert("Content-Type", "application/json");
                HttpResponse::new("200", Some(m), body)
            }
            _ => HttpResponse::new("404", None, Self::load_file("404.html")),
        }
    }
}
