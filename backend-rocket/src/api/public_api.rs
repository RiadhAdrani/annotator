use crate::{
    controller::common_controller::CommonController, error::response::RequestError,
    models::common_models::AppData,
};
use rocket::{serde::json::Json, Response};

#[get("/data")]
pub fn get_data() -> Result<Json<AppData>, Json<RequestError>> {
    let data = CommonController::get_data();

    if data.is_err() {
        return Err(Json(data.err().unwrap()));
    }

    Ok(Json(data.unwrap()))
}

#[options("/data")]
fn options_handler<'r>() -> Response<'r> {
    Response::build()
        .raw_header("Access-Control-Allow-Origin", "http://localhost:8000/data")
        .raw_header("Access-Control-Allow-Methods", "OPTIONS, POST")
        .raw_header("Access-Control-Allow-Headers", "Content-Type")
        .finalize()
}
