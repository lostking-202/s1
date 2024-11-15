use http::httprequest;
use http::httpresponse;
use std::collections::HashMap;
use std::env;
use std::fmt::format;
use std::fs;
use std::path::Path;
use http::httprequest::HttpRequest;
use http::httpresponse::HttpResponse;
use serde::{Serialize,Deserialize};
use std::path::MAIN_SEPARATOR_STR;

pub trait Handler{
    fn handle(req:&HttpRequest)->HttpResponse;
    fn load_file(file_name:&str)->Option<String>{
        let default_path=format!("{}/public",env!("CARGO_MANIFEST_DIR"));
        let public_path=env::var("PUBLIC_PATH").unwrap_or(default_path);
        let full_path=format!("{}/{}",public_path,file_name);

        let contents=fs::read_to_string(full_path);
        contents.ok()
    }
}

pub struct StaticPageHandler;
pub struct PageNotFoundHandler;
pub struct WebServiceHandler;

#[derive(Serialize,Deserialize)]
pub struct OrderStatus{
    order_id:i32,
    order_date:String,
    order_status:String
}

impl Handler for PageNotFoundHandler{
    fn handle(req: &HttpRequest) -> HttpResponse {
        HttpResponse::new("404",None,Self::load_file("404.html"))
    }
}

impl Handler for StaticPageHandler{
    fn handle(req: &HttpRequest) -> HttpResponse {
        let http::httprequest::Resource::Path(s)=&req.resource;
        let route:Vec<&str>=s.split("/").collect();
        match route[1]{
            ""=>HttpResponse::new("200",None,Self::load_file("index.html")),
            "health"=>HttpResponse::new("200",None,Self::load_file("health.html")),
            path=>match Self::load_file(path){
                Some(contents)=>{
                    let mut map=HashMap::new();
                    if(path.ends_with(".css")){
                        map.insert("Content-Type","text/css");
                    }else if(path.ends_with(".js")){
                        map.insert("Content-Type","text/javascript");
                    }else{
                        map.insert("Content-Type","text/html");
                    }
                    HttpResponse::new("200",Some(map),Some(contents))
                }
                None=>HttpResponse::new("404",None,Self::load_file("404.html"))
            }
        }
    }
}

impl WebServiceHandler{
    fn load_json()->Vec<OrderStatus>{
        let default_path=format!("{}{}data",env!("CARGO_MANIFEST_DIR"),MAIN_SEPARATOR_STR);
        let data_path=env::var("DATA_PATH").unwrap_or(default_path);
        let full_path=format!(r"{}{}{}",data_path,MAIN_SEPARATOR_STR,"orders.json");
        let json_contents=fs::read_to_string(full_path);
        let orders=serde_json::from_str(json_contents.unwrap().as_str()).unwrap();
        orders
    }
}
impl Handler for WebServiceHandler{
    fn handle(req: &HttpRequest) -> HttpResponse {
        let http::httprequest::Resource::Path(s)=&req.resource;
        let route:Vec<&str>=s.split("/").collect();
        // localhost:3000/api/shipping/orders
        match route[2] {
            "shipping" if route.len()>2 && route[3]=="orders"=>{
                let body=Some(serde_json::to_string(&Self::load_json()).unwrap());
                let mut headers=HashMap::new();
                headers.insert("Content-Type","application/json");
                HttpResponse::new("200",Some(headers),body)
            }
            _=>HttpResponse::new("404",None,Self::load_file("404.html"))
        }
    }
}

#[cfg(test)]
mod tests{
    use std::fs;
    use std::path::Path;

    #[test]
    fn test(){
        // D:\Users\issuser\RustroverProjects\s1\httpserver\data\orders.json
        let path_string=r"D:\Users\issuser\RustroverProjects\s1\httpserver\data\orders.json";
        let json_contents=fs::read_to_string(path_string);
        println!("{}",json_contents.unwrap().as_str());
    }
}
