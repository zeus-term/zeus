use crate::platform::unix_signals::SIGNAL;

use super::{
	buffer::Buffer,
	err::InitializationError,
	io::IOHandler,
	key_mapper::{BindingPresentError, KeyMapper, KeypressAction},
};
use common::{
	catch_error,
	constants::character::{
		ctrl_chars, general_ascii_chars, printable_ascii_characters, triplet_char_actions,
	},
};
use std::{
	mem::MaybeUninit,
	rc::Rc,
	sync::{Arc, Mutex, Once},
};

type MutArc<T> = Arc<Mutex<T>>;

pub fn get_term_state() -> (
	&'static MutArc<IOHandler>,
	&'static MutArc<Buffer>,
	&'static Rc<KeyMapper>,
) {
	static mut HANDLER_STATE: MaybeUninit<MutArc<IOHandler>> = MaybeUninit::uninit();
	static mut BUFFER_STATE: MaybeUninit<MutArc<Buffer>> = MaybeUninit::uninit();
	static mut KEY_MAPPER_STATE: MaybeUninit<Rc<KeyMapper>> = MaybeUninit::uninit();
	static INIT_ONCE: Once = Once::new();

	unsafe {
		INIT_ONCE.call_once(|| {
			if let Ok(key_mapper) = init_keymapper() {
				KEY_MAPPER_STATE.write(Rc::new(key_mapper));
			} else {
				panic!();
			}
			HANDLER_STATE.write(Arc::new(Mutex::new(IOHandler::new())));
			BUFFER_STATE.write(Arc::new(Mutex::new(Buffer::new())));
		});

		(
			HANDLER_STATE.assume_init_ref(),
			BUFFER_STATE.assume_init_ref(),
			KEY_MAPPER_STATE.assume_init_ref(),
		)
	}
}

fn add_multi_char_action(
	key_mapper: &mut KeyMapper,
	keys: &[u8],
	action: KeypressAction,
) -> Result<(), BindingPresentError> {
	for (idx, val) in keys[0..keys.len() - 1].iter().enumerate() {
		let x = *val;

		// if this is already present then we don't care, its a good thing
		let _ = key_mapper
			.register_binding(&keys[0..=idx], Box::new(move || KeypressAction::Buffer(x)));
	}

	// if this is not present we are running into some problem will have to
	// take look at the initialization of keymapper
	key_mapper.register_binding(keys, Box::new(move || action))?;

	Ok(())
}

fn init_keymapper() -> Result<KeyMapper, InitializationError> {
	let mut key_mapper = KeyMapper::new();

	// Initialize keypress action for alphanumeric characters
	for character in ('a'..='z').chain('A'..='Z').chain('0'..='9') {
		catch_error!(
			key_mapper
				.register_binding(
					&[character as u8],
					Box::new(move || KeypressAction::Return(character as u8)),
				) => InitializationError{}
		);
	}

	// Initialize keypress action for non-alphanumeric characters
	for character in printable_ascii_characters::ALL_CHARS.iter() {
		catch_error!(
			key_mapper
				.register_binding(
					&[*character],
					Box::new(move || KeypressAction::Return(*character)),
				) => InitializationError {}
		);
	}

	for character in general_ascii_chars::ALL_GENERAL_CHARS.iter() {
		if *character == general_ascii_chars::ESC
			|| *character == general_ascii_chars::EOF
			|| *character == general_ascii_chars::NEWLINE
		{
			continue;
		}

		catch_error!(
			key_mapper
				.register_binding(
					&[*character],
					Box::new(move || KeypressAction::Return(*character)),
				) => InitializationError {}
		);
	}

	// Essential linux shell signals mapping
	catch_error!(
		key_mapper.register_binding(
			&[ctrl_chars::CTRL_C],
			Box::new(move || KeypressAction::Signal(SIGNAL::SIGINT)),
		) => InitializationError{}
	);

	catch_error!(
		key_mapper.register_binding(
			&[ctrl_chars::CTRL_Z],
			Box::new(move || KeypressAction::Signal(SIGNAL::SIGSTOP)),
		) => InitializationError{}
	);

	catch_error!(
		key_mapper.register_binding(
		&[ctrl_chars::CTRL_D],
		Box::new(move || KeypressAction::Signal(SIGNAL::SIGQUIT)),
		) => InitializationError{}
	);

	// multi-char actions
	catch_error!(
		add_multi_char_action(
			&mut key_mapper,
			&triplet_char_actions::UP,
			KeypressAction::Action(triplet_char_actions::Chars::Up),
		) => InitializationError{}
	);

	catch_error!(
		add_multi_char_action(
			&mut key_mapper,
			&triplet_char_actions::DOWN,
			KeypressAction::Action(triplet_char_actions::Chars::Down),
		) => InitializationError{}
	);

	catch_error!(
		add_multi_char_action(
			&mut key_mapper,
			&triplet_char_actions::LEFT,
			KeypressAction::Action(triplet_char_actions::Chars::Left),
		) => InitializationError{}
	);

	catch_error!(
		add_multi_char_action(
			&mut key_mapper,
			&triplet_char_actions::RIGHT,
			KeypressAction::Action(triplet_char_actions::Chars::Right),
		) => InitializationError{}
	);

	Ok(key_mapper)
}
