use crate::{
    helpers::colors_helpers::LABEL_COLORS, models::common_models::AppData, object::error::ApiError,
};

pub struct CommonController;

impl CommonController {
    pub fn get_data() -> Result<AppData, ApiError> {
        let colors = LABEL_COLORS.clone();

        let response = AppData { colors };

        Ok(response)
    }
}
