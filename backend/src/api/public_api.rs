use crate::{
    controller::common_controller::CommonController, error::response::RequestError,
    models::common_models::AppData,
};
use rocket::serde::json::Json;

#[get("/data")]
pub fn get_data() -> Result<Json<AppData>, Json<RequestError>> {
    let data = CommonController::get_data();

    if data.is_err() {
        return Err(Json(data.err().unwrap()));
    }

    Ok(Json(data.unwrap()))
}
