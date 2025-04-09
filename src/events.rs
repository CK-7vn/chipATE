use crossterm::event::{self, Event};
use std::time::{Duration, Instant};
use tokio::sync::{mpsc, oneshot};

pub struct AppEventHandler {
    receiver: mpsc::Receiver<AppEvent>,
}

pub enum AppEvent {
    Tick,
    Key(crossterm::event::KeyEvent),
}

impl AppEventHandler {
    pub fn new(tick_rate: u64, mut shutdown_rx: oneshot::Receiver<()>) -> Self {
        let tick_rate = Duration::from_millis(tick_rate);
        let (sender, receiver) = mpsc::channel(10);
        //let handler = {
        let sender = sender.clone();
        tokio::spawn(async move {
            let mut last_tick = Instant::now();
            loop {
                tokio::select! {
                    _ = tokio::time::sleep(tick_rate) => {
                        if last_tick.elapsed() >= tick_rate {
                            if sender.send(AppEvent::Tick).await.is_err() {
                                break;
                            }
                            last_tick = Instant::now();
                        }
                    }
                    _ = &mut shutdown_rx => {
                        println!("Event handler shutting down");
                        break;
                    }
                }

                if event::poll(Duration::from_millis(0)).expect("Poll failed") {
                    match event::read().expect("Read failed") {
                        Event::Key(e) => {
                            let _ = sender.send(AppEvent::Key(e)).await;
                        }
                        _ => {
                            let _ = sender.send(AppEvent::Tick).await;
                        }
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
