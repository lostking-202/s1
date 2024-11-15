use http::httprequest::HttpRequest;
use http::httpresponse::HttpResponse;
use std::io::prelude::*;
use http::httprequest;
use super::handler::{WebServiceHandler, StaticPageHandler, PageNotFoundHandler, Handler};

pub struct Router;

impl Router{
    pub fn route(req:HttpRequest,stream :&mut impl Write){
        println!("{:?}",req.method);
        match req.method { 
            httprequest::Method::Get=>match &req.resource {
                httprequest::Resource::Path(s)=>{
                    let route:Vec<&str>=s.split("/").collect();
                    match route[1] {
                        "api"=>{
                            let resp:HttpResponse=WebServiceHandler::handle(&req);
                            let _=resp.send_response(stream);
                        }
                        _=>{
                            let resp:HttpResponse=StaticPageHandler::handle(&req);
                            let _=resp.send_response(stream);
                        }
                    }
                }

            }
            _=>{
                let resp:HttpResponse=PageNotFoundHandler::handle(&req);
                let _=resp.send_response(stream);
            }
        }
    }
}