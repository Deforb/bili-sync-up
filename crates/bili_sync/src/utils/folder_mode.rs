pub const FOLDER_MODE_NORMAL: i32 = 0;
pub const FOLDER_MODE_FLAT: i32 = 1;
pub const FOLDER_MODE_WEAK_FLAT: i32 = 2;

pub fn normalize_folder_mode(mode: i32) -> i32 {
    match mode {
        FOLDER_MODE_FLAT | FOLDER_MODE_WEAK_FLAT => mode,
        _ => FOLDER_MODE_NORMAL,
    }
}

pub fn folder_mode_from_flat_folder(flat_folder: bool) -> i32 {
    if flat_folder {
        FOLDER_MODE_FLAT
    } else {
        FOLDER_MODE_NORMAL
    }
}

pub fn folder_mode_from_api(folder_mode: Option<&str>, flat_folder: Option<bool>, default_mode: i32) -> i32 {
    if let Some(mode) = folder_mode {
        return parse_folder_mode(mode).unwrap_or(default_mode);
    }
    if let Some(flat) = flat_folder {
        return folder_mode_from_flat_folder(flat);
    }
    normalize_folder_mode(default_mode)
}

pub fn parse_folder_mode(mode: &str) -> Option<i32> {
    match mode {
        "normal" => Some(FOLDER_MODE_NORMAL),
        "flat" => Some(FOLDER_MODE_FLAT),
        "weak_flat" => Some(FOLDER_MODE_WEAK_FLAT),
        _ => None,
    }
}

pub fn folder_mode_to_api(mode: i32) -> &'static str {
    match normalize_folder_mode(mode) {
        FOLDER_MODE_FLAT => "flat",
        FOLDER_MODE_WEAK_FLAT => "weak_flat",
        _ => "normal",
    }
}

pub fn is_flat_mode(mode: i32) -> bool {
    normalize_folder_mode(mode) == FOLDER_MODE_FLAT
}

pub fn is_weak_flat_mode(mode: i32) -> bool {
    normalize_folder_mode(mode) == FOLDER_MODE_WEAK_FLAT
}
