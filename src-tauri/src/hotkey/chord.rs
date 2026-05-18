use std::collections::HashSet;
use std::time::{Duration, Instant};

use rdev::{EventType, Key};

use super::platform;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Modifier {
    Ctrl,
    Shift,
    Alt,
    Super,
}

impl Modifier {
    pub fn matches(self, k: Key) -> bool {
        matches!(
            (self, k),
            (Modifier::Ctrl, Key::ControlLeft | Key::ControlRight)
                | (Modifier::Shift, Key::ShiftLeft | Key::ShiftRight)
                | (Modifier::Alt, Key::Alt | Key::AltGr)
                | (Modifier::Super, Key::MetaLeft | Key::MetaRight)
        )
    }
}

#[derive(Debug, Clone)]
pub enum ParsedTrigger {
    Combo {
        modifiers: Vec<Modifier>,
        key: Key,
        presses: u8,
    },
    ModifierTap {
        modifier: Modifier,
        presses: u8,
    },
}

fn all_other_modifiers_idle(required: &[Modifier]) -> bool {
    [Modifier::Ctrl, Modifier::Shift, Modifier::Alt, Modifier::Super]
        .into_iter()
        .filter(|m| !required.contains(m))
        .all(|m| !platform::modifier_held(m))
}

pub fn is_copy_combo(trigger: &ParsedTrigger) -> bool {
    matches!(
        trigger,
        ParsedTrigger::Combo { modifiers, key, .. }
            if modifiers.len() == 1
                && modifiers[0] == Modifier::Ctrl
                && *key == Key::KeyC
    )
}

pub struct ChordEngine {
    pub trigger: ParsedTrigger,
    pub window: Duration,
    pressed: HashSet<Key>,
    counter: u8,
    last_event_at: Option<Instant>,
    mod_held: bool,
    mod_clean: bool,
}

impl ChordEngine {
    pub fn new(trigger: ParsedTrigger, window: Duration) -> Self {
        Self {
            trigger,
            window,
            pressed: HashSet::new(),
            counter: 0,
            last_event_at: None,
            mod_held: false,
            mod_clean: false,
        }
    }

    pub fn replace(&mut self, trigger: ParsedTrigger, window: Duration) {
        self.trigger = trigger;
        self.window = window;
        self.counter = 0;
        self.last_event_at = None;
        self.mod_held = false;
        self.mod_clean = false;
    }

    pub fn process(&mut self, event_type: &EventType) -> bool {
        match event_type {
            EventType::KeyPress(k) => self.on_key_down(*k),
            EventType::KeyRelease(k) => {
                self.on_key_up(*k);
                false
            }
            _ => false,
        }
    }

    fn on_key_down(&mut self, k: Key) -> bool {
        let first_press = self.pressed.insert(k);

        if let ParsedTrigger::ModifierTap { modifier, .. } = &self.trigger {
            if modifier.matches(k) {
                if first_press {
                    self.mod_held = true;
                    self.mod_clean = true;
                }
            } else if self.mod_held {
                self.mod_clean = false;
            }
            return false;
        }

        if !first_press {
            return false;
        }

        if let ParsedTrigger::Combo {
            modifiers, key, presses,
        } = &self.trigger
        {
            if k != *key {
                return false;
            }
            let mod_ok = modifiers.iter().all(|m| {
                if platform::HAS_OS_MODIFIER_QUERY {
                    platform::modifier_held(*m)
                } else {
                    self.pressed.iter().any(|p| m.matches(*p))
                }
            });
            let none_extra = !platform::HAS_OS_MODIFIER_QUERY
                || all_other_modifiers_idle(modifiers);
            if !mod_ok || !none_extra {
                return false;
            }
            return self.register_step(*presses);
        }
        false
    }

    fn on_key_up(&mut self, k: Key) {
        self.pressed.remove(&k);

        if let ParsedTrigger::ModifierTap { modifier, presses } = &self.trigger {
            if modifier.matches(k) && self.mod_held {
                if self.mod_clean {
                    self.register_step(*presses);
                }
                self.mod_held = false;
                self.mod_clean = false;
            }
        }
    }

    fn register_step(&mut self, required: u8) -> bool {
        let now = Instant::now();
        if let Some(last) = self.last_event_at {
            if now.duration_since(last) > self.window {
                self.counter = 0;
            }
        }
        self.counter = self.counter.saturating_add(1);
        self.last_event_at = Some(now);
        if self.counter >= required {
            self.counter = 0;
            self.last_event_at = None;
            true
        } else {
            false
        }
    }
}
