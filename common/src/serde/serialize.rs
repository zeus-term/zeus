pub(crate) trait Serialize {
	fn validate(self);

	fn serialize(self) -> Vec<u8>;
}
