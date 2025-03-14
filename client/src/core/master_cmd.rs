#[derive(Debug, Clone, Copy)]
pub enum MasterCmd {
	TriggerAutocomplete,
	SubmitCommand,
	TermBell,
	Signal(u8),
}
