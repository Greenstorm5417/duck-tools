use tower_lsp::lsp_types::*;

/// Get all DuckyScript keyword completions
pub fn get_keyword_completions() -> Vec<CompletionItem> {
    let mut items = Vec::new();

    // String commands
    for (label, detail) in &[
        ("STRING", "Type a string of characters"),
        ("STRINGLN", "Type a string followed by ENTER"),
        ("STRING_BLOCK", "Multi-line string block"),
        ("STRINGLN_BLOCK", "Multi-line string block with ENTER"),
        ("END_STRING", "End string block"),
    ] {
        items.push(create_keyword_item(label, detail));
    }

    // Timing commands
    for (label, detail) in &[
        ("DELAY", "Wait for specified milliseconds"),
        ("DEFAULT_DELAY", "Set default delay between keystrokes"),
        ("DEFAULTDELAY", "Set default delay (alternative)"),
        ("STRING_DELAY", "Set delay between characters in STRING"),
        ("STRINGDELAY", "Set string delay (alternative)"),
    ] {
        items.push(create_keyword_item(label, detail));
    }

    // Key commands
    for (label, detail) in &[
        ("ENTER", "Press ENTER key"),
        ("HOLD", "Hold a key down"),
        ("RELEASE", "Release a held key"),
        ("REPEAT", "Repeat previous command N times"),
    ] {
        items.push(create_keyword_item(label, detail));
    }

    // Control flow
    for (label, detail) in &[
        ("IF", "Conditional statement"),
        ("ELSE_IF", "Alternative condition"),
        ("ELSE", "Fallback condition"),
        ("END_IF", "End conditional block"),
        ("WHILE", "Loop while condition is true"),
        ("END_WHILE", "End while loop"),
    ] {
        items.push(create_keyword_item(label, detail));
    }

    // Functions
    for (label, detail) in &[
        ("FUNCTION", "Define a function"),
        ("END_FUNCTION", "End function definition"),
        ("RETURN", "Return from function"),
    ] {
        items.push(create_keyword_item(label, detail));
    }

    // Variables
    for (label, detail) in &[
        ("VAR", "Declare a variable"),
        ("INJECT_VAR", "Inject variable value as keystrokes"),
        ("EXFIL", "Exfiltrate data to USB storage"),
    ] {
        items.push(create_keyword_item(label, detail));
    }

    // Preprocessor
    for (label, detail) in &[
        ("DEFINE", "Define a preprocessor constant"),
        ("IF_DEFINED_TRUE", "Conditional compilation if defined"),
        (
            "IF_NOT_DEFINED_TRUE",
            "Conditional compilation if not defined",
        ),
        ("ELSE_DEFINED", "Preprocessor else"),
        ("END_IF_DEFINED", "End preprocessor conditional"),
    ] {
        items.push(create_keyword_item(label, detail));
    }

    // System commands
    for (label, detail) in &[
        ("ATTACKMODE", "Configure USB device mode"),
        ("INJECT", "Inject keystrokes"),
        ("INJECT_MOD", "Inject with modifiers"),
        ("KEYCODE", "Send raw keycode"),
    ] {
        items.push(create_keyword_item(label, detail));
    }

    // Modifier commands
    for (label, detail) in &[
        ("MOD_KEY_DOWN", "Press modifier and key"),
        ("MOD_KEY_UP", "Release modifier and key"),
        ("KEY_DOWN", "Press key down"),
        ("KEY_UP", "Release key"),
        ("MOD_DOWN", "Press modifier down"),
        ("MOD_UP", "Release modifier"),
    ] {
        items.push(create_keyword_item(label, detail));
    }

    // Random generators
    for (label, detail) in &[
        (
            "RANDOM_LOWERCASE_LETTER",
            "Generate random lowercase letter",
        ),
        (
            "RANDOM_UPPERCASE_LETTER",
            "Generate random uppercase letter",
        ),
        ("RANDOM_NUMBER", "Generate random digit (0-9)"),
        ("RANDOM_LETTER", "Generate random letter"),
        ("RANDOM_SPECIAL", "Generate random special character"),
        ("RANDOM_CHAR", "Generate random character"),
    ] {
        items.push(create_keyword_item(label, detail));
    }

    // Blocks
    for (label, detail) in &[
        ("EXTENSION", "Define payload extension"),
        ("END_EXTENSION", "End extension definition"),
        ("STAGE", "Define staged payload"),
        ("END_STAGE", "End stage definition"),
        ("BUTTON_DEF", "Define button action"),
        ("END_BUTTON", "End button definition"),
    ] {
        items.push(create_keyword_item(label, detail));
    }

    // Comments
    for (label, detail) in &[
        ("REM", "Comment line"),
        ("REM_BLOCK", "Start comment block"),
        ("END_REM", "End comment block"),
    ] {
        items.push(create_keyword_item(label, detail));
    }

    // Debug
    for (label, detail) in &[
        ("DEBUGGER_BREAKPOINT", "Set debugger breakpoint"),
        (
            "INJECT_BREAKPOINT_LINE_NUMBER",
            "Inject breakpoint line number",
        ),
    ] {
        items.push(create_keyword_item(label, detail));
    }

    items
}

/// Get modifier key completions
pub fn get_modifier_completions() -> Vec<CompletionItem> {
    let mut items = Vec::new();

    for (label, detail) in &[
        ("CTRL", "Control modifier key"),
        ("CONTROL", "Control modifier key (alternative)"),
        ("SHIFT", "Shift modifier key"),
        ("ALT", "Alt modifier key"),
        ("GUI", "GUI/Windows/Command key"),
        ("WINDOWS", "Windows key"),
        ("COMMAND", "Command key (Mac)"),
        ("CTRL-ALT", "Ctrl+Alt combination"),
    ] {
        items.push(CompletionItem {
            label: label.to_string(),
            kind: Some(CompletionItemKind::CONSTANT),
            detail: Some(detail.to_string()),
            ..Default::default()
        });
    }

    items
}

/// Get special key completions
pub fn get_special_key_completions() -> Vec<CompletionItem> {
    let mut items = Vec::new();

    // Function keys
    for i in 1..=12 {
        items.push(CompletionItem {
            label: format!("F{}", i),
            kind: Some(CompletionItemKind::CONSTANT),
            detail: Some(format!("Function key F{}", i)),
            ..Default::default()
        });
    }

    // Special keys
    for (label, detail) in &[
        ("ESCAPE", "Escape key"),
        ("ESC", "Escape key (short)"),
        ("TAB", "Tab key"),
        ("CAPSLOCK", "Caps Lock key"),
        ("PRINTSCREEN", "Print Screen key"),
        ("SCROLLLOCK", "Scroll Lock key"),
        ("PAUSE", "Pause key"),
        ("BREAK", "Break key"),
        ("INSERT", "Insert key"),
        ("HOME", "Home key"),
        ("PAGEUP", "Page Up key"),
        ("PAGEDOWN", "Page Down key"),
        ("DELETE", "Delete key"),
        ("END", "End key"),
        ("UP", "Up arrow"),
        ("DOWN", "Down arrow"),
        ("LEFT", "Left arrow"),
        ("RIGHT", "Right arrow"),
        ("UPARROW", "Up arrow (alternative)"),
        ("DOWNARROW", "Down arrow (alternative)"),
        ("LEFTARROW", "Left arrow (alternative)"),
        ("RIGHTARROW", "Right arrow (alternative)"),
        ("SPACE", "Space bar"),
        ("BACKSPACE", "Backspace key"),
        ("APP", "Application/Menu key"),
        ("MENU", "Menu key"),
    ] {
        items.push(CompletionItem {
            label: label.to_string(),
            kind: Some(CompletionItemKind::CONSTANT),
            detail: Some(detail.to_string()),
            ..Default::default()
        });
    }

    // Numpad keys
    for (label, detail) in &[
        ("NUM_LOCK", "Num Lock key"),
        ("NUMLOCK", "Num Lock (alternative)"),
        ("KP_0", "Numpad 0"),
        ("KP_1", "Numpad 1"),
        ("KP_2", "Numpad 2"),
        ("KP_3", "Numpad 3"),
        ("KP_4", "Numpad 4"),
        ("KP_5", "Numpad 5"),
        ("KP_6", "Numpad 6"),
        ("KP_7", "Numpad 7"),
        ("KP_8", "Numpad 8"),
        ("KP_9", "Numpad 9"),
        ("KP_PLUS", "Numpad +"),
        ("KP_MINUS", "Numpad -"),
        ("KP_MULTIPLY", "Numpad *"),
        ("KP_DIVIDE", "Numpad /"),
        ("KP_ENTER", "Numpad Enter"),
        ("KP_DOT", "Numpad ."),
    ] {
        items.push(CompletionItem {
            label: label.to_string(),
            kind: Some(CompletionItemKind::CONSTANT),
            detail: Some(detail.to_string()),
            ..Default::default()
        });
    }

    items
}

/// Get reserved variable completions
pub fn get_reserved_variable_completions() -> Vec<CompletionItem> {
    let mut items = Vec::new();

    for (label, detail) in &[
        ("$_HOST_OS", "Detected host operating system"),
        ("$_CAPSLOCK_ON", "CAPS LOCK state (TRUE/FALSE)"),
        ("$_NUMLOCK_ON", "NUM LOCK state (TRUE/FALSE)"),
        ("$_SCROLLLOCK_ON", "SCROLL LOCK state (TRUE/FALSE)"),
        ("$_GUI_LABEL", "System-specific GUI key name"),
        ("$_CURRENT_PATH", "Current working directory"),
        ("$_EXFIL_LABEL", "USB storage drive label"),
        ("$_f_ret", "Function return value"),
        ("$_RANDOM_INT", "Random integer"),
        ("$_RANDOM_MIN", "Minimum value for random int"),
        ("$_RANDOM_MAX", "Maximum value for random int"),
    ] {
        items.push(CompletionItem {
            label: label.to_string(),
            kind: Some(CompletionItemKind::VARIABLE),
            detail: Some(detail.to_string()),
            ..Default::default()
        });
    }

    items
}

/// Create a keyword completion item
fn create_keyword_item(label: &str, detail: &str) -> CompletionItem {
    CompletionItem {
        label: label.to_string(),
        kind: Some(CompletionItemKind::KEYWORD),
        detail: Some(detail.to_string()),
        ..Default::default()
    }
}
