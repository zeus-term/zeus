use crate::enum_with_value;

pub enum KeywordType {
	Placeholder,
	Command,
}

enum_with_value!(ProtocolKeyword, usize, KeywordType, {
	Unknown => 0 => KeywordType::Placeholder,
	Nack => 1 => KeywordType::Command,
	Ack => 2 => KeywordType::Command, // Placeholder
	Close => 3 => KeywordType::Command, // Command
	ForwardToShell => 4 => KeywordType::Command, // Command
	GetSuggestion => 5 => KeywordType::Command, // Command
	StartMsg => 200 => KeywordType::Placeholder, // Placeholder
	EndMsg => 201 => KeywordType::Placeholder, // Placeholer
});
