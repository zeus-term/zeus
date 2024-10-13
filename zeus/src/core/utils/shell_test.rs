#[cfg(test)]
mod tests {
	use crate::core::utils::shell;

	#[test]
	fn test_get_current_shell() {
		// We should always have a shell for root user
		assert_eq!(shell::get_user_shell(Some(0)), String::from("/bin/bash"));
	}
}
