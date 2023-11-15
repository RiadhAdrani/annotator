use rocket::{
    fairing::{Fairing, Info, Kind},
    Data, Request, Response,
};

use super::env::APP_URL;

pub struct Cors;

#[rocket::async_trait]
impl Fairing for Cors {
    fn info(&self) -> Info {
        Info {
            name: "Append CORS headers in responses",
            kind: Kind::Response,
        }
    }

    async fn on_request(&self, req: &mut Request<'_>, _res: &mut Data<'_>) {
        println!("[CORS] Request from : {:?}", req.uri().path().to_string());

        // req.add_header(Header::new(
        //     header::ACCESS_CONTROL_ALLOW_ORIGIN.to_string(),
        //     APP_URL.to_string(),
        // ));
        // req.add_header(Header::new(
        //     header::ACCESS_CONTROL_ALLOW_METHODS.to_string(),
        //     "GET, POST, PUT, DELETE, PATCH",
        // ));
        // req.add_header(Header::new(
        //     header::ACCESS_CONTROL_ALLOW_HEADERS.to_string(),
        //     "*",
        // ));
        // req.add_header(Header::new(
        //     header::ACCESS_CONTROL_ALLOW_CREDENTIALS.to_string(),
        //     "true",
        // ));
    }

    async fn on_response<'r>(&self, req: &'r Request<'_>, res: &mut Response<'r>) {
        println!("[CORS] Response for : {:?}", req.uri().path().to_string());

        res.set_raw_header("Access-Control-Allow-Origin", APP_URL.to_string());
        res.set_raw_header(
            "Access-Control-Allow-Methods",
            "GET, POST, PUT, DELETE, PATCH",
        );
        res.set_raw_header("Access-Control-Allow-Headers", "*");
        res.set_raw_header("Access-Control-Allow-Credentials", "true");
    }
}
