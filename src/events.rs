use crossterm::event::{Event, EventStream, KeyCode, KeyEventKind};
use futures_util::StreamExt;
use log::debug;
use std::time::Duration;
use tokio::sync::mpsc;

#[derive(Debug)]
pub enum AppEvent {
    Tick,
    Key { key: KeyCode, pressed: bool },
}

pub struct AppEventHandler {
    receiver: mpsc::Receiver<AppEvent>,
}

impl AppEventHandler {
    pub fn new(tick_rate: u64, mut shutdown_rx: tokio::sync::oneshot::Receiver<()>) -> Self {
        let (sender, receiver) = mpsc::channel(128);

        tokio::task::spawn_local(async move {
            let tick_duration = Duration::from_millis(tick_rate);
            let mut tick_interval = tokio::time::interval(tick_duration);
            let mut event_stream = EventStream::new();

            loop {
                tokio::select! {
                    _ = tick_interval.tick() => {
                        if sender.send(AppEvent::Tick).await.is_err() {
                            break;
                        }
                    },
                    maybe_event = event_stream.next() => {
                        match maybe_event {
                            Some(Ok(Event::Key(key_event))) => {
                                let pressed = match key_event.kind {
                                    KeyEventKind::Press => true,
                                    KeyEventKind::Release => false,
                                    KeyEventKind::Repeat => continue,
                                };
                                if sender.send(AppEvent::Key { key: key_event.code, pressed }).await.is_err() {
                                    break;
                                }
                            },
                            Some(Ok(_)) => {},
                            Some(Err(e)) => {
                                debug!("Event stream error: {:?}", e);
                                break;
                            },
                            None => break,
                        }
                    },
                    _ = &mut shutdown_rx => {
                        debug!("Shutdown signal received.");
                        break;
                    }
                }
            }
        });
        Self { receiver }
    }

    pub async fn next(&mut self) -> Option<AppEvent> {
        self.receiver.recv().await
    }
}
