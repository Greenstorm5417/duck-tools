use ducky_core::lexer::{tokenize_line, TokenType};
use tower_lsp::lsp_types::*;

/// Generate hover documentation for a command
pub fn get_hover_info(line: &str, _position: Position) -> Option<Hover> {
    let token_type = tokenize_line(line.trim());
    
    let documentation = match token_type {
        TokenType::String => "**STRING** - Type a string of characters\n\nExample: `STRING Hello World`",
        TokenType::StringLn => "**STRINGLN** - Type a string followed by ENTER\n\nExample: `STRINGLN Hello World`",
        TokenType::Delay => "**DELAY** - Wait for specified milliseconds\n\nExample: `DELAY 1000`",
        TokenType::DefaultDelay => "**DEFAULT_DELAY** - Set default delay between keystrokes\n\nExample: `DEFAULT_DELAY 50`",
        TokenType::Enter => "**ENTER** - Press the Enter/Return key\n\nExample: `ENTER`",
        TokenType::Hold => "**HOLD** - Hold a key down\n\nExample: `HOLD SHIFT`",
        TokenType::Release => "**RELEASE** - Release a held key\n\nExample: `RELEASE SHIFT`",
        TokenType::Repeat => "**REPEAT** - Repeat the previous command N times\n\nExample: `REPEAT 5`",
        
        TokenType::If => "**IF** - Conditional statement\n\nExample:\n```duckyscript\nIF $var == 1 THEN\n    STRING true\nEND_IF\n```",
        TokenType::While => "**WHILE** - Loop statement\n\nExample:\n```duckyscript\nWHILE $count < 10\n    STRING loop\nEND_WHILE\n```",
        TokenType::Function => "**FUNCTION** - Define a reusable function\n\nExample:\n```duckyscript\nFUNCTION myFunc()\n    STRING called\nEND_FUNCTION\n```",
        
        TokenType::Declaration => "**VAR** - Declare and assign a variable\n\nExample: `VAR $myVar = 123`",
        TokenType::InjectVar => "**INJECT_VAR** - Inject variable value as keystrokes\n\nExample: `INJECT_VAR $myVar`",
        TokenType::ExfilVar => "**EXFIL** - Exfiltrate data to USB storage\n\nExample: `EXFIL $data`",
        
        TokenType::Attackmode => "**ATTACKMODE** - Configure USB device mode\n\nExample: `ATTACKMODE HID STORAGE`\n\nModes:\n- HID (keyboard)\n- STORAGE (USB drive)\n- SERIAL (CDC ACM)\n- VID/PID configuration",
        
        TokenType::RandomLowercaseLetter => "**RANDOM_LOWERCASE_LETTER** - Generate random lowercase letter\n\nExample: `VAR $letter = RANDOM_LOWERCASE_LETTER`",
        TokenType::RandomUppercaseLetter => "**RANDOM_UPPERCASE_LETTER** - Generate random uppercase letter\n\nExample: `VAR $letter = RANDOM_UPPERCASE_LETTER`",
        TokenType::RandomNumber => "**RANDOM_NUMBER** - Generate random digit (0-9)\n\nExample: `VAR $num = RANDOM_NUMBER`",
        
        TokenType::ModKeyDown => "**MOD_KEY_DOWN** - Press modifier and key simultaneously\n\nExample: `MOD_KEY_DOWN CTRL c`",
        TokenType::KeyDown => "**KEY_DOWN** - Press key down (hold)\n\nExample: `KEY_DOWN SHIFT`",
        TokenType::KeyUp => "**KEY_UP** - Release key\n\nExample: `KEY_UP SHIFT`",
        
        TokenType::Define => "**DEFINE** - Define a preprocessor constant\n\nExample: `DEFINE DEBUG_MODE`",
        TokenType::IfDefined => "**IF_DEFINED** - Conditional compilation\n\nExample:\n```duckyscript\nIF_DEFINED DEBUG_MODE\n    STRING Debug enabled\nEND_IF_DEFINED\n```",
        
        TokenType::Extension => "**EXTENSION** - Define payload extension\n\nExample:\n```duckyscript\nEXTENSION myExt\n    STRING extension code\nEND_EXTENSION\n```",
        
        TokenType::Stage => "**STAGE** - Define staged payload\n\nExample:\n```duckyscript\nSTAGE 1\n    STRING first stage\nEND_STAGE\n```",
        
        TokenType::Rem => "**REM** - Comment line (not executed)\n\nExample: `REM This is a comment`",
        TokenType::RemBlock => "**REM_BLOCK** - Multi-line comment block\n\nExample:\n```duckyscript\nREM_BLOCK\nMultiple lines\nof comments\nEND_REM\n```",
        
        _ => return None,
    };
    
    Some(Hover {
        contents: HoverContents::Markup(MarkupContent {
            kind: MarkupKind::Markdown,
            value: documentation.to_string(),
        }),
        range: None,
    })
}

/// Get documentation for reserved variables
pub fn get_variable_hover(var_name: &str) -> Option<Hover> {
    let doc = match var_name {
        "$_HOST_OS" => "**$_HOST_OS** - Detected host operating system\n\nValues: `WINDOWS`, `MACOS`, `LINUX`, `UNKNOWN`",
        "$_CAPSLOCK_ON" => "**$_CAPSLOCK_ON** - CAPS LOCK state\n\nValues: `TRUE`, `FALSE`",
        "$_NUMLOCK_ON" => "**$_NUMLOCK_ON** - NUM LOCK state\n\nValues: `TRUE`, `FALSE`",
        "$_SCROLLLOCK_ON" => "**$_SCROLLLOCK_ON** - SCROLL LOCK state\n\nValues: `TRUE`, `FALSE`",
        "$_GUI_LABEL" => "**$_GUI_LABEL** - System-specific GUI key name\n\nExamples: `WINDOWS`, `COMMAND`, `GUI`",
        "$_CURRENT_PATH" => "**$_CURRENT_PATH** - Current working directory path",
        "$_EXFIL_LABEL" => "**$_EXFIL_LABEL** - USB storage drive label for exfiltration",
        "$_f_ret" => "**$_f_ret** - Function return value\n\nUsed internally to return values from functions",
        _ => return None,
    };
    
    Some(Hover {
        contents: HoverContents::Markup(MarkupContent {
            kind: MarkupKind::Markdown,
            value: doc.to_string(),
        }),
        range: None,
    })
}
