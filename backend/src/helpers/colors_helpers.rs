use std::collections::HashMap;

use crate::{error::common::CommonError, models::text_annotation_model::Label};

lazy_static! {
    pub static ref LABEL_COLORS: HashMap<String, String> = HashMap::<String, String>::from([
        ("dark_blue".to_string(), "#00008B".to_string()),
        ("blue".to_string(), "#4169E1".to_string()),
        ("dodger_blue".to_string(), "#1E90FF".to_string()),
        ("steel_blue".to_string(), "#4682B4".to_string()),
        ("purple".to_string(), "#663399".to_string()),
        ("pink".to_string(), "#FF69B4".to_string()),
        ("red".to_string(), "#8B0000".to_string()),
        ("brown".to_string(), "#A0522D".to_string()),
        ("coral".to_string(), "#FF7F50".to_string()),
        ("burly_wood".to_string(), "#DEB887".to_string()),
        ("orange".to_string(), "#FFA500".to_string()),
        ("yellow".to_string(), "#FFD700".to_string()),
        ("green_yellow".to_string(), "#ADFF2F".to_string()),
        ("olive".to_string(), "#808000".to_string()),
        ("green".to_string(), "#228B22".to_string()),
        ("aquamarine".to_string(), "#7FFFD4".to_string()),
        ("cyan".to_string(), "#00FFFF".to_string()),
        ("dark_cyan".to_string(), "#008B8B".to_string()),
        ("gray".to_string(), "#A9A9A9".to_string()),
        ("light_gray".to_string(), "#D3D3D3".to_string()),
    ]);
}

pub fn is_color_used(color: String, labels: &Vec<Label>) -> bool {
    let is_used = labels.iter().any(|label| label.color == color);

    is_used
}

pub fn is_valid_color(color: String) -> bool {
    let is_valid = LABEL_COLORS.get(&color);

    is_valid.is_some()
}

pub fn get_next_valid_color(labels: &Vec<Label>) -> Result<String, CommonError> {
    let color = LABEL_COLORS
        .iter()
        .find(|it| !is_color_used(it.0.clone(), labels));

    if color.is_none() {
        return Err(CommonError {
            description: "All colors are used".to_string(),
        });
    }

    Ok(color.unwrap().0.clone())
}
