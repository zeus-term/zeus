use std::{
    mem::MaybeUninit,
    rc::Rc,
    sync::{Arc, Mutex, Once},
};

use super::{
    buffer::Buffer,
    constants::character::PRINTABLE_ASCII_CHARACTERS,
    err::InitializationError,
    io::IOHandler,
    key_mapper::{KeyMapper, KeypressAction},
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

pub fn init_keymapper() -> Result<KeyMapper, InitializationError> {
    let mut key_mapper = KeyMapper::new();

    // Initialize keypress action for alphanumeric characters
    for character in ('a'..='z').chain('A'..='Z').chain('0'..='9') {
        if key_mapper
            .register_binding(
                &[character as u8],
                Box::new(move || KeypressAction::Return(character as u8)),
            )
            .is_err()
        {
            return Err(InitializationError {});
        }
    }

    // Initialize keypress action for non-alphanumeric characters
    for character in PRINTABLE_ASCII_CHARACTERS {
        if key_mapper
            .register_binding(
                &[character],
                Box::new(move || KeypressAction::Return(character)),
            )
            .is_err()
        {
            return Err(InitializationError {});
        }
    }

    // These are all CTRL-<Alphabhet> inputs
    // TODO: for now we are just echoing the bytes but we would want it do actual implementation
    // what these keys actually do for like C-c to send SIGINT and C-z to send SIGSTP
    // Exception C-i and TAB have same ascii value so we are ignoring C-i here
    for character in (1..=8).chain(9..=26) {
        if key_mapper
            .register_binding(
                &[character as u8],
                Box::new(move || KeypressAction::Return(character as u8)),
            )
            .is_err()
        {
            return Err(InitializationError {});
        }
    }

    Ok(key_mapper)
}
