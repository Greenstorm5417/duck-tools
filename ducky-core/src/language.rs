use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyboardLayout {
    #[serde(flatten)]
    pub keys: HashMap<String, String>,
}

impl KeyboardLayout {
    pub fn default_us() -> Self {
        let mut keys = HashMap::new();

        keys.insert("CTRL".to_string(), "01,00,00".to_string());
        keys.insert("CONTROL".to_string(), "01,00,00".to_string());
        keys.insert("SHIFT".to_string(), "02,00,00".to_string());
        keys.insert("ALT".to_string(), "04,00,00".to_string());
        keys.insert("GUI".to_string(), "08,00,00".to_string());
        keys.insert("WINDOWS".to_string(), "08,00,00".to_string());
        keys.insert("COMMAND".to_string(), "08,00,00".to_string());

        keys.insert("a".to_string(), "00,00,04".to_string());
        keys.insert("A".to_string(), "02,00,04".to_string());
        keys.insert("b".to_string(), "00,00,05".to_string());
        keys.insert("B".to_string(), "02,00,05".to_string());
        keys.insert("c".to_string(), "00,00,06".to_string());
        keys.insert("C".to_string(), "02,00,06".to_string());
        keys.insert("d".to_string(), "00,00,07".to_string());
        keys.insert("D".to_string(), "02,00,07".to_string());
        keys.insert("e".to_string(), "00,00,08".to_string());
        keys.insert("E".to_string(), "02,00,08".to_string());
        keys.insert("f".to_string(), "00,00,09".to_string());
        keys.insert("F".to_string(), "02,00,09".to_string());
        keys.insert("g".to_string(), "00,00,0a".to_string());
        keys.insert("G".to_string(), "02,00,0a".to_string());
        keys.insert("h".to_string(), "00,00,0b".to_string());
        keys.insert("H".to_string(), "02,00,0b".to_string());
        keys.insert("i".to_string(), "00,00,0c".to_string());
        keys.insert("I".to_string(), "02,00,0c".to_string());
        keys.insert("j".to_string(), "00,00,0d".to_string());
        keys.insert("J".to_string(), "02,00,0d".to_string());
        keys.insert("k".to_string(), "00,00,0e".to_string());
        keys.insert("K".to_string(), "02,00,0e".to_string());
        keys.insert("l".to_string(), "00,00,0f".to_string());
        keys.insert("L".to_string(), "02,00,0f".to_string());
        keys.insert("m".to_string(), "00,00,10".to_string());
        keys.insert("M".to_string(), "02,00,10".to_string());
        keys.insert("n".to_string(), "00,00,11".to_string());
        keys.insert("N".to_string(), "02,00,11".to_string());
        keys.insert("o".to_string(), "00,00,12".to_string());
        keys.insert("O".to_string(), "02,00,12".to_string());
        keys.insert("p".to_string(), "00,00,13".to_string());
        keys.insert("P".to_string(), "02,00,13".to_string());
        keys.insert("q".to_string(), "00,00,14".to_string());
        keys.insert("Q".to_string(), "02,00,14".to_string());
        keys.insert("r".to_string(), "00,00,15".to_string());
        keys.insert("R".to_string(), "02,00,15".to_string());
        keys.insert("s".to_string(), "00,00,16".to_string());
        keys.insert("S".to_string(), "02,00,16".to_string());
        keys.insert("t".to_string(), "00,00,17".to_string());
        keys.insert("T".to_string(), "02,00,17".to_string());
        keys.insert("u".to_string(), "00,00,18".to_string());
        keys.insert("U".to_string(), "02,00,18".to_string());
        keys.insert("v".to_string(), "00,00,19".to_string());
        keys.insert("V".to_string(), "02,00,19".to_string());
        keys.insert("w".to_string(), "00,00,1a".to_string());
        keys.insert("W".to_string(), "02,00,1a".to_string());
        keys.insert("x".to_string(), "00,00,1b".to_string());
        keys.insert("X".to_string(), "02,00,1b".to_string());
        keys.insert("y".to_string(), "00,00,1c".to_string());
        keys.insert("Y".to_string(), "02,00,1c".to_string());
        keys.insert("z".to_string(), "00,00,1d".to_string());
        keys.insert("Z".to_string(), "02,00,1d".to_string());

        keys.insert("1".to_string(), "00,00,1e".to_string());
        keys.insert("!".to_string(), "02,00,1e".to_string());
        keys.insert("2".to_string(), "00,00,1f".to_string());
        keys.insert("@".to_string(), "02,00,1f".to_string());
        keys.insert("3".to_string(), "00,00,20".to_string());
        keys.insert("#".to_string(), "02,00,20".to_string());
        keys.insert("4".to_string(), "00,00,21".to_string());
        keys.insert("$".to_string(), "02,00,21".to_string());
        keys.insert("5".to_string(), "00,00,22".to_string());
        keys.insert("%".to_string(), "02,00,22".to_string());
        keys.insert("6".to_string(), "00,00,23".to_string());
        keys.insert("^".to_string(), "02,00,23".to_string());
        keys.insert("7".to_string(), "00,00,24".to_string());
        keys.insert("&".to_string(), "02,00,24".to_string());
        keys.insert("8".to_string(), "00,00,25".to_string());
        keys.insert("*".to_string(), "02,00,25".to_string());
        keys.insert("9".to_string(), "00,00,26".to_string());
        keys.insert("(".to_string(), "02,00,26".to_string());
        keys.insert("0".to_string(), "00,00,27".to_string());
        keys.insert(")".to_string(), "02,00,27".to_string());

        keys.insert("ENTER".to_string(), "00,00,28".to_string());
        keys.insert("ESC".to_string(), "00,00,29".to_string());
        keys.insert("ESCAPE".to_string(), "00,00,29".to_string());
        keys.insert("BACKSPACE".to_string(), "00,00,2a".to_string());
        keys.insert("TAB".to_string(), "00,00,2b".to_string());
        keys.insert("SPACE".to_string(), "00,00,2c".to_string());
        keys.insert(" ".to_string(), "00,00,2c".to_string());

        keys.insert("-".to_string(), "00,00,2d".to_string());
        keys.insert("_".to_string(), "02,00,2d".to_string());
        keys.insert("=".to_string(), "00,00,2e".to_string());
        keys.insert("+".to_string(), "02,00,2e".to_string());
        keys.insert("[".to_string(), "00,00,2f".to_string());
        keys.insert("{".to_string(), "02,00,2f".to_string());
        keys.insert("]".to_string(), "00,00,30".to_string());
        keys.insert("}".to_string(), "02,00,30".to_string());
        keys.insert("\\".to_string(), "00,00,31".to_string());
        keys.insert("|".to_string(), "02,00,31".to_string());
        keys.insert(";".to_string(), "00,00,33".to_string());
        keys.insert(":".to_string(), "02,00,33".to_string());
        keys.insert("'".to_string(), "00,00,34".to_string());
        keys.insert("\"".to_string(), "02,00,34".to_string());
        keys.insert("`".to_string(), "00,00,35".to_string());
        keys.insert("~".to_string(), "02,00,35".to_string());
        keys.insert(",".to_string(), "00,00,36".to_string());
        keys.insert("<".to_string(), "02,00,36".to_string());
        keys.insert(".".to_string(), "00,00,37".to_string());
        keys.insert(">".to_string(), "02,00,37".to_string());
        keys.insert("/".to_string(), "00,00,38".to_string());
        keys.insert("?".to_string(), "02,00,38".to_string());

        keys.insert("CAPSLOCK".to_string(), "00,00,39".to_string());
        keys.insert("F1".to_string(), "00,00,3a".to_string());
        keys.insert("F2".to_string(), "00,00,3b".to_string());
        keys.insert("F3".to_string(), "00,00,3c".to_string());
        keys.insert("F4".to_string(), "00,00,3d".to_string());
        keys.insert("F5".to_string(), "00,00,3e".to_string());
        keys.insert("F6".to_string(), "00,00,3f".to_string());
        keys.insert("F7".to_string(), "00,00,40".to_string());
        keys.insert("F8".to_string(), "00,00,41".to_string());
        keys.insert("F9".to_string(), "00,00,42".to_string());
        keys.insert("F10".to_string(), "00,00,43".to_string());
        keys.insert("F11".to_string(), "00,00,44".to_string());
        keys.insert("F12".to_string(), "00,00,45".to_string());

        keys.insert("PRINTSCREEN".to_string(), "00,00,46".to_string());
        keys.insert("SCROLLLOCK".to_string(), "00,00,47".to_string());
        keys.insert("PAUSE".to_string(), "00,00,48".to_string());
        keys.insert("BREAK".to_string(), "00,00,48".to_string());
        keys.insert("INSERT".to_string(), "00,00,49".to_string());
        keys.insert("HOME".to_string(), "00,00,4a".to_string());
        keys.insert("PAGEUP".to_string(), "00,00,4b".to_string());
        keys.insert("DELETE".to_string(), "00,00,4c".to_string());
        keys.insert("DEL".to_string(), "00,00,4c".to_string());
        keys.insert("END".to_string(), "00,00,4d".to_string());
        keys.insert("PAGEDOWN".to_string(), "00,00,4e".to_string());
        keys.insert("RIGHTARROW".to_string(), "00,00,4f".to_string());
        keys.insert("RIGHT".to_string(), "00,00,4f".to_string());
        keys.insert("LEFTARROW".to_string(), "00,00,50".to_string());
        keys.insert("LEFT".to_string(), "00,00,50".to_string());
        keys.insert("DOWNARROW".to_string(), "00,00,51".to_string());
        keys.insert("DOWN".to_string(), "00,00,51".to_string());
        keys.insert("UPARROW".to_string(), "00,00,52".to_string());
        keys.insert("UP".to_string(), "00,00,52".to_string());
        keys.insert("NUMLOCK".to_string(), "00,00,53".to_string());
        keys.insert("MENU".to_string(), "00,00,65".to_string());
        keys.insert("APP".to_string(), "00,00,65".to_string());

        KeyboardLayout { keys }
    }

    pub fn get_bytes_for_key(&self, key: &str) -> Option<Vec<String>> {
        self.keys
            .get(key)
            .map(|codes| codes.split(',').map(|s| s.to_string()).collect())
    }
}
