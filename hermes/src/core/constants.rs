pub mod character {
    /// Hex codes for printable ASCII characters except alphanumeric characters
    pub const AT: u8 = 0x40;
    pub const BANG: u8 = 0x21;
    pub const OCTOTHORPE: u8 = 0x23;
    pub const DOLLAR: u8 = 0x24;
    pub const PERCENT: u8 = 0x25;
    pub const CARET: u8 = 0x5E;
    pub const AMPERSAND: u8 = 0x026;
    pub const ASTERICK: u8 = 0x2A;
    pub const OPEN_PARENTHESIS: u8 = 0x28;
    pub const CLOSE_PARENTHESIS: u8 = 0x29;
    pub const UNDERSCORE: u8 = 0x5F;
    pub const PLUS: u8 = 0x2B;

    pub const MINUS: u8 = 0x2D;
    pub const EQUALS: u8 = 0x3D;

    pub const OPEN_SQUARE_BRACKET: u8 = 0x5B;
    pub const CLOSE_SQUARE_BRACKET: u8 = 0x5D;
    pub const OPEN_CURLY_BRACKET: u8 = 0x7B;
    pub const CLOSE_CURLY_BRACKET: u8 = 0x7D;
    pub const BACKSLASH: u8 = 0x5C;
    pub const PIPE: u8 = 0x7C;
    pub const SEMICOLON: u8 = 0x3B;
    pub const SINGLE_QUOTE: u8 = 0x27;
    pub const COLON: u8 = 0x3a;
    pub const DOUBLE_QUOTE: u8 = 0x22;
    pub const COMMA: u8 = 0x2C;
    pub const PERIOD: u8 = 0x2E;
    pub const FORWARD_SLASH: u8 = 0x2F;
    pub const OPEN_ANGULAR_BRACKET: u8 = 0x3C;
    pub const CLOSE_ANGULA_BRACKET: u8 = 0x3E;
    pub const TERNARY: u8 = 0x3F;
    pub const BACK_TICK: u8 = 0x60;
    pub const TILDE: u8 = 0x7E;
    pub const SPACE: u8 = 0x20;

    pub const PRINTABLE_ASCII_CHARACTERS: [u8; 33] = [
        AT,
        BANG,
        OCTOTHORPE,
        DOLLAR,
        PERCENT,
        CARET,
        AMPERSAND,
        ASTERICK,
        OPEN_PARENTHESIS,
        CLOSE_PARENTHESIS,
        UNDERSCORE,
        PLUS,
        MINUS,
        EQUALS,
        OPEN_SQUARE_BRACKET,
        CLOSE_SQUARE_BRACKET,
        OPEN_CURLY_BRACKET,
        CLOSE_CURLY_BRACKET,
        BACKSLASH,
        PIPE,
        SEMICOLON,
        SINGLE_QUOTE,
        COLON,
        DOUBLE_QUOTE,
        COMMA,
        PERIOD,
        FORWARD_SLASH,
        OPEN_ANGULAR_BRACKET,
        CLOSE_ANGULA_BRACKET,
        TERNARY,
        BACK_TICK,
        TILDE,
        SPACE,
    ];

    pub const ENTER: u8 = 0x0D;
    pub const TAB: u8 = 0x09;
    pub const ESC: u8 = 0x1B;
    pub const CTRL_CLOSE_BRACE: u8 = 0x1D;
    pub const CTRL_PIPE: u8 = 0x1C;
    pub const CTRL_FW_SLASH: u8 = 0x1F;
    pub const CTRL_SPACE: u8 = 0x00;
}
