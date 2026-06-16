use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref OPERATOR_MAP: HashMap<&'static str, u16> = {
        let mut m = HashMap::new();
        m.insert("+", 0x02E8);
        m.insert("-", 0x03E8);
        m.insert("*", 0x04E8);
        m.insert("/", 0x05E8);
        m.insert("==", 0x06E8);
        m.insert("!=", 0x07E8);
        m.insert("<", 0x08E8);
        m.insert(">", 0x09E8);
        m.insert("<=", 0xA8E8);
        m.insert(">=", 0xA9E8);
        m.insert("&&", 0xAAE8);
        m.insert("AND", 0xAAE8);
        m.insert("||", 0xBBE8);
        m.insert("OR", 0xBBE8);
        m.insert("&", 0x0AE8);
        m.insert("|", 0x0BE8);
        m.insert(">>", 0x0CE8);
        m.insert("<<", 0x0DE8);
        m.insert("%", 0x0EE8);
        m.insert("^", 0xE80F);
        m
    };
    pub static ref RESERVED_VARIABLES: HashMap<&'static str, u16> = {
        let mut m = HashMap::new();
        m.insert("$_BUTTON_ENABLED", 0x4280);
        m.insert("$_BUTTON_USER_DEFINED", 0x4281);
        m.insert("$_BUTTON_PUSH_RECEIVED", 0x4284);
        m.insert("$_LED_SHOW_STORAGE_ACTIVITY", 0x4285);
        m.insert("$_SYSTEM_LEDS_ENABLED", 0x4286);
        m.insert("$_STORAGE_LEDS_ENABLED", 0x4287);
        m.insert("$_INJECTING_LEDS_ENABLED", 0x4288);
        m.insert("$_EXFIL_LEDS_ENABLED", 0x4289);
        m.insert("$_CAPSLOCK_ON", 0x4290);
        m.insert("$_NUMLOCK_ON", 0x4291);
        m.insert("$_SCROLLLOCK_ON", 0x4292);
        m.insert("$_SAVED_CAPSLOCK_ON", 0x4293);
        m.insert("$_SAVED_NUMLOCK_ON", 0x4294);
        m.insert("$_SAVED_SCROLLLOCK_ON", 0x4295);
        m.insert("$_RECEIVED_HOST_LOCK_LED_REPLY", 0x4296);
        m.insert("$_EXFIL_MODE_ENABLED", 0x4297);
        m.insert("$_STORAGE_ACTIVITY_TIMEOUT", 0x4298);
        m.insert("$_BUTTON_TIMEOUT", 0x4299);
        m.insert("$_PAYLOAD_PARSE_SPEED", 0x429A);
        m.insert("$_CURRENT_VID", 0x429B);
        m.insert("$_CURRENT_PID", 0x429C);
        m.insert("$_OS", 0x429D);
        m.insert("$_HOST_CONFIGURATION_REQUEST_COUNT", 0x429F);
        m.insert("$_CURRENT_ATTACKMODE", 0x42A0);
        m.insert("$_JITTER_ENABLED", 0x42A2);
        m.insert("$_JITTER_MAX", 0x42A3);
        m.insert("$_STORAGE_ACTIVE", 0x42A4);
        m.insert("$_RANDOM_INT", 0x42A8);
        m.insert("$_RANDOM_MIN", 0x42F0);
        m.insert("$_RANDOM_MAX", 0x42F1);
        m.insert("$_RANDOM_SEED", 0x42F2);
        m.insert("$_RANDOM_UINT16", 0x42F3);
        m.insert("$_RANDOM_ASCII_LOWER_LETTER", 0x42F4);
        m.insert("$_RANDOM_ASCII_UPPER_LETTER", 0x42F5);
        m.insert("$_RANDOM_ASCII_LETTER", 0x42F6);
        m.insert("$_RANDOM_ASCII_NUMBER", 0x42F7);
        m.insert("$_RANDOM_ASCII_SPECIAL", 0x42F8);
        m.insert("$_RANDOM_ASCII_CHAR", 0x42F9);
        m.insert("$_RANDOM_LOWER_LETTER_KEYCODE", 0x42FA);
        m.insert("$_RANDOM_UPPER_LETTER_KEYCODE", 0x42FB);
        m.insert("$_RANDOM_LETTER_KEYCODE", 0x42FF);
        m.insert("$_RANDOM_NUMBER_KEYCODE", 0x42FC);
        m.insert("$_RANDOM_SPECIAL_KEYCODE", 0x42FD);
        m.insert("$_RANDOM_CHAR_KEYCODE", 0x42FE);
        m
    };
    pub static ref RESERVED_CONSTANTS: HashMap<&'static str, u16> = {
        let mut m = HashMap::new();
        m.insert("FALSE", 0x4267);
        m.insert("TRUE", 0x4268);
        m.insert("WINDOWS", 0x4269);
        m.insert("MACOS", 0x4270);
        m.insert("LINUX", 0x4271);
        m.insert("ANDROID", 0x4272);
        m.insert("IOS", 0x4273);
        m.insert("CHROMEOS", 0x4274);
        m
    };
    pub static ref BUILTINS_MAP: HashMap<&'static str, u16> = {
        let mut m = HashMap::new();
        m.insert("RESET", 0xED04);
        m.insert("KEY_HOLD", 0xF8FF);
        m.insert("KEY_RELEASE", 0xE8EE);
        m.insert("DISABLE_BUTTON", 0xEEEB);
        m.insert("ENABLE_BUTTON", 0xEEEC);
        m.insert("RESTART_PAYLOAD", 0xF1EA);
        m.insert("STOP_PAYLOAD", 0xF1EB);
        m.insert("LED_OFF", 0xEDEA);
        m.insert("LED_G", 0xEDEB);
        m.insert("LED_GREEN", 0xEDEB);
        m.insert("LED_R", 0xEDEC);
        m.insert("LED_RED", 0xEDEC);
        m.insert("ENABLE_SYSTEM_LEDS", 0xED01);
        m.insert("DISABLE_SYSTEM_LEDS", 0xED02);
        m.insert("SAVE_HOST_KEYBOARD_LOCK_STATE", 0xEBEA);
        m.insert("RESTORE_HOST_KEYBOARD_LOCK_STATE", 0xEBEB);
        m.insert("WAIT_FOR_BUTTON_PRESS", 0xEAEA);
        m.insert("SAVE_ATTACKMODE", 0xE9EA);
        m.insert("RESTORE_ATTACKMODE", 0xE9EB);
        m.insert("WAIT_FOR_CAPS_ON", 0xEA01);
        m.insert("WAIT_FOR_CAPS_OFF", 0xEA02);
        m.insert("WAIT_FOR_CAPS_CHANGE", 0xEA03);
        m.insert("WAIT_FOR_NUM_ON", 0xEA04);
        m.insert("WAIT_FOR_NUM_OFF", 0xEA05);
        m.insert("WAIT_FOR_NUM_CHANGE", 0xEA06);
        m.insert("WAIT_FOR_SCROLL_ON", 0xEA07);
        m.insert("WAIT_FOR_SCROLL_OFF", 0xEA08);
        m.insert("WAIT_FOR_SCROLL_CHANGE", 0xEA09);
        m.insert("WAIT_FOR_STORAGE_ACTIVITY", 0xEAEE);
        m.insert("WAIT_FOR_STORAGE_INACTIVITY", 0xEAEF);
        m.insert("HIDE_PAYLOAD", 0xE9F8);
        m.insert("RESTORE_PAYLOAD", 0xE9F9);
        m
    };
    pub static ref DOUBLE_OPERATORS: Vec<&'static str> =
        vec!["&&", "||", "<<", ">>", "==", "!=", "()", "<=", ">="];
    pub static ref REQUIRES_LANG_PACK: Vec<&'static str> = {
        vec![
            "$_RANDOM_LOWER_LETTER_KEYCODE",
            "$_RANDOM_UPPER_LETTER_KEYCODE",
            "$_RANDOM_LETTER_KEYCODE",
            "$_RANDOM_NUMBER_KEYCODE",
            "$_RANDOM_SPECIAL_KEYCODE",
            "$_RANDOM_CHAR_KEYCODE",
        ]
    };
}

pub const MAX_PAYLOAD_SIZE: usize = 16384;
