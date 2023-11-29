use actix_web::{
    get,
    web::{self, Json},
    Result, Scope,
};

use crate::{
    controllers::public_controller::CommonController, models::common_models::AppData,
    object::error::ApiError,
};

#[get("/")]
pub async fn get_app_data() -> Result<Json<AppData>, ApiError> {
    let res = CommonController::get_data();

    if res.is_err() {
        return Err(res.err().unwrap());
    }

    Ok(Json(res.unwrap()))
}

pub fn data_routes() -> Scope {
    web::scope("/app-data").service(get_app_data)
}
