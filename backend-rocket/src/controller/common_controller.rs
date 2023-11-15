use crate::{
    error::response::RequestError, helpers::colors_helpers::LABEL_COLORS,
    models::common_models::AppData,
};

pub struct CommonController;

impl CommonController {
    pub fn get_data() -> Result<AppData, RequestError> {
        let colors = LABEL_COLORS.clone();

        let response = AppData { colors };

        Ok(response)
    }

    pub fn options
}
