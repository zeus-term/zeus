use log::{info, warn};

pub fn cleanup_socket(socket_path: &str) {
	if let Err(err) = std::fs::remove_file(socket_path) {
		warn!(
			"Failed to remove existing socket {} file: {}",
			socket_path, err
		);
	} else {
		info!("Unix socket {} cleaned up", socket_path);
	}
}
