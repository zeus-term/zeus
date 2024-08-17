use super::constants::character::triplet_char_actions;
use super::master_cmd::MasterCmd;
use crate::platform::unix_signals::SIGNAL;
use std::collections::HashMap;
use std::fmt;
use std::result::Result;

const PRIME: u32 = 199;

#[derive(Clone, Copy, Debug)]
pub enum KeypressAction {
    Return(u8),
    Buffer(u8),
    Signal(SIGNAL),
    Action(triplet_char_actions::Chars),
    MasterCommand(MasterCmd),
}

pub struct KeyMapper {
    key_fn_map: HashMap<u32, Box<dyn Fn() -> KeypressAction>>,
}

#[derive(Debug, Clone)]
pub struct BindingPresentError {
    hash: u32,
}

pub struct BindingNotPresentError {
    hash: u32,
}

fn hash(keys: &[u8]) -> u32 {
    let mut result: u32 = 0;

    for (idx, data) in keys.iter().enumerate() {
        result += PRIME.pow(idx as u32 + 1) * (*data as u32);
    }

    result
}

impl fmt::Display for BindingPresentError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Binding with hash value: {} already present", self.hash)
    }
}

impl fmt::Display for BindingNotPresentError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Binding with hash value: {} is absent", self.hash)
    }
}

impl Default for KeyMapper {
    fn default() -> Self {
        Self::new()
    }
}

impl KeyMapper {
    pub fn new() -> KeyMapper {
        KeyMapper {
            key_fn_map: HashMap::new(),
        }
    }

    pub fn register_binding(
        &mut self,
        keys: &[u8],
        callback: Box<dyn Fn() -> KeypressAction>,
    ) -> Result<(), BindingPresentError> {
        let key = hash(keys);

        if self.key_fn_map.get(&key).is_some() {
            return Err(BindingPresentError { hash: key });
        }

        self.key_fn_map.insert(key, callback);

        Ok(())
    }

    pub fn key_fn(
        &self,
        keys: &[u8],
    ) -> Result<&Box<dyn Fn() -> KeypressAction>, BindingNotPresentError> {
        let key = hash(keys);

        if self.key_fn_map.get(&key).is_none() {
            return Err(BindingNotPresentError { hash: key });
        }
        Ok(self.key_fn_map.get(&key).unwrap())
    }
}
