pub mod socket {
	pub const HELIOS_COMM: &str = "/tmp/zeus-helios.sock";
	pub const HERMES_COMM: &str = "/tmp/zeus-hermes.sock";
}

pub mod msg_directives {
	use crate::str_vec;

	pub const CREATE_PTY: &[u8] = str_vec!("CREATE_PTY");
}

pub mod unix_paths {
	pub const ETC_PASSWD: &str = "/etc/passwd";
}

pub mod character {
	use crate::declare_all_consts;

	declare_all_consts!(printable_ascii_characters, ALL_CHARS, u8, {
		AT, At: 0x40,
		BANG, Bang: 0x21,
		OCTOTHORPE, Octothorpe: 0x23,
		DOLLAR, Dollar: 0x24,
		PERCENT, Percent: 0x25,
		CARET, Caret: 0x5E,
		AMPERSAND, Ampersand: 0x026,
		ASTERICK, Asterick: 0x2A,
		OPEN_PARENTHESIS, OpenParenthesis: 0x28,
		CLOSE_PARENTHESIS, CloseParenthesis: 0x29,
		UNDERSCORE, Underscore: 0x5F,
		PLUS, Plus: 0x2B,

		MINUS, Minus:   0x2D,
		EQUALS, Equals:   0x3D,

		OPEN_SQUARE_BRACKET, OpenSquareBracket:   0x5B,
		CLOSE_SQUARE_BRACKET, CloseSquareBracket:   0x5D,
		OPEN_CURLY_BRACKET, OpenCurlyBracket:   0x7B,
		CLOSE_CURLY_BRACKET, CloseCurlyBracket:   0x7D,
		BACKSLASH, Backslash:   0x5C,
		PIPE, Pipe:   0x7C,
		SEMICOLON, Semicolon:   0x3B,
		SINGLE_QUOTE, SingleQuote:   0x27,
		COLON, Colon:   0x3a,
		DOUBLE_QUOTE, DoubleQuote:   0x22,
		COMMA, Comma:   0x2C,
		PERIOD, Period:   0x2E,
		FORWARD_SLASH, ForwardSlash:   0x2F,
		OPEN_ANGULAR_BRACKET, OpenAngularBracket:   0x3C,
		CLOSE_ANGULA_BRACKET, CloseAngularBracket:   0x3E,
		TERNARY, Ternary:   0x3F,
		BACK_TICK, Backtick:   0x60,
		TILDE, Tilde:   0x7E,
		SPACE, Space:   0x20,
	});

	declare_all_consts!(
		general_ascii_chars,
		ALL_GENERAL_CHARS,
		u8,
		{
			ENTER, Enter:   0x0D,
			TAB, Tab:   0x09,
			ESC, Esc:   0x1B,
			CTRL_CLOSE_BRACE, Ctrl:   0x1D,
			CTRL_PIPE, CtrlPipe:   0x1C,
			CTRL_FW_SLASH, CtrlFwSlash:   0x1F,
			CTRL_SPACE, CtrlSpace:   0x00,
			BACKSPACE, Backspace: 0x7F,
			NEWLINE, Newline: 0x0A,
			EOF, EndOfFile: 0x04,
		}
	);

	declare_all_consts!(ctrl_chars, CTRL_ALL_CHARS, CTRL : Ctrl, u8, {
		A: 0x01,
		B: 0x02,
		C: 0x03,
		D: 0x04,
		E: 0x05,
		F: 0x06,
		G: 0x07,
		H: 0x08,
		I: 0x09,
		J: 0x0A,
		K: 0x0B,
		L: 0x0C,
		M: 0x0D,
		N: 0x0E,
		O: 0x0F,
		P: 0x10,
		Q: 0x11,
		R: 0x12,
		S: 0x13,
		T: 0x14,
		U: 0x15,
		V: 0x16,
		W: 0x17,
		X: 0x18,
		Y: 0x19,
		Z: 0x1A,
	});

	declare_all_consts!(triplet_char_actions, ALL_TRIPLET_CHAR_ACTIONS, [u8; 3], {
		UP, Up: [0x1B, 0x5B, 0x41],
		DOWN, Down: [0x1B, 0x5B, 0x42],
		RIGHT, Right: [0x1B, 0x5B, 0x43],
		LEFT, Left: [0x1B, 0x5B, 0x44],
		F1, F1: [0x1B, 0x4F, 0x50],
		F2, F2: [0x1b, 0x4f, 0x51],
		F3, F3: [0x1b, 0x4f, 0x52],
		F4, F4: [0x1b, 0x4f, 0x53],
	});

	declare_all_consts!(
		hex_char_actions, ALL_HEX_CHAR_ACTIONS, [u8; 6], {
			CTRL_UP, CtrlUp: [0x1b, 0x5b, 0x31, 0x3b, 0x35, 0x41],
			CTRL_DOWN, CtrlDown: [0x1b, 0x5b, 0x31, 0x3b, 0x35, 0x42],
			CTRL_RIGHT, CtrlRight: [0x1b, 0x5b, 0x31, 0x3b, 0x35, 0x43],
			CTRL_LEFT, CtrlLeft: [0x1b, 0x5b, 0x31, 0x3b, 0x35, 0x44],
		}
	);
}

pub const STDIN_FILENO: i32 = 0;
pub const STDOUT_FILENO: i32 = 1;
pub const STDERR_FILENO: i32 = 2;
