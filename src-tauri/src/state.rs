use std::sync::Arc;

use parking_lot::Mutex;

use crate::config::ConfigStore;
use crate::hotkey::HotkeyService;
use crate::secret::SecretStore;
use crate::translator::Translator;

pub struct AppState {
    pub config: Arc<ConfigStore>,
    pub secrets: Arc<SecretStore>,
    pub hotkey: Arc<HotkeyService>,
    pub translator: Arc<Translator>,
    pub previous_window: Arc<Mutex<Option<isize>>>,
}
