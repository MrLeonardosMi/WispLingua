use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

use parking_lot::Mutex;
use tokio::sync::mpsc;

use crate::config::HotkeyConfig;
use crate::error::AppResult;

use super::chord::{ChordEngine, ParsedTrigger};
use super::parser::parse_trigger;

#[derive(Clone, Copy, Debug)]
pub struct CursorPos {
    pub x: i32,
    pub y: i32,
}

pub struct HotkeyService {
    engine: Arc<Mutex<ChordEngine>>,
    cursor: Arc<Mutex<CursorPos>>,
    fire_tx: mpsc::UnboundedSender<()>,
    fire_rx: Arc<Mutex<Option<mpsc::UnboundedReceiver<()>>>>,
}

impl HotkeyService {
    pub fn new(initial: &HotkeyConfig) -> AppResult<Self> {
        let parsed = parse_trigger(&initial.trigger)?;
        let engine = Arc::new(Mutex::new(ChordEngine::new(
            parsed,
            Duration::from_millis(initial.chord_window_ms),
        )));
        let cursor = Arc::new(Mutex::new(CursorPos { x: 0, y: 0 }));
        let (tx, rx) = mpsc::unbounded_channel();
        Ok(Self {
            engine,
            cursor,
            fire_tx: tx,
            fire_rx: Arc::new(Mutex::new(Some(rx))),
        })
    }

    pub fn cursor(&self) -> CursorPos {
        *self.cursor.lock()
    }

    pub fn take_receiver(&self) -> Option<mpsc::UnboundedReceiver<()>> {
        self.fire_rx.lock().take()
    }

    pub fn apply(&self, cfg: &HotkeyConfig) -> AppResult<()> {
        let parsed = parse_trigger(&cfg.trigger)?;
        let mut e = self.engine.lock();
        e.replace(parsed, Duration::from_millis(cfg.chord_window_ms));
        Ok(())
    }

    pub fn parsed_trigger(&self) -> ParsedTrigger {
        self.engine.lock().trigger.clone()
    }

    pub fn start(&self) {
        let engine = self.engine.clone();
        let cursor = self.cursor.clone();
        let tx = self.fire_tx.clone();

        thread::Builder::new()
            .name("rdev-listener".to_string())
            .spawn(move || {
                let mut last_fire = Instant::now() - Duration::from_secs(10);
                let cooldown = Duration::from_millis(200);
                let cb = move |ev: rdev::Event| {
                    if let rdev::EventType::MouseMove { x, y } = ev.event_type {
                        *cursor.lock() = CursorPos { x: x as i32, y: y as i32 };
                        return;
                    }
                    let fired = engine.lock().process(&ev.event_type);
                    if fired && last_fire.elapsed() > cooldown {
                        last_fire = Instant::now();
                        let _ = tx.send(());
                    }
                };
                if let Err(e) = rdev::listen(cb) {
                    log::error!("rdev listener stopped: {e:?}");
                }
            })
            .expect("spawn rdev listener");
    }
}
