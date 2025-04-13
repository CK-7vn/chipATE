// src/events.rs
use log::debug;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use tokio::sync::mpsc;

#[derive(Debug)]
pub enum AppEvent {
    Tick,
    Key { key: Keycode, pressed: bool },
}

pub struct AppEventHandler {
    receiver: mpsc::Receiver<AppEvent>,
}

impl AppEventHandler {
    pub fn new(tick_rate: u64, mut shutdown_rx: tokio::sync::oneshot::Receiver<()>) -> Self {
        let (sender, receiver) = mpsc::channel(128);
        let sdl_sender = sender.clone();

        tokio::task::spawn_local(async move {
            let sdl_context = sdl2::init().expect("failed to initialize SDL2");
            let video_subsystem = sdl_context.video().expect("failed to initialize SDL video");
            let _window = video_subsystem
                .window("hidden window", 0, 0)
                .position(0, 1)
                .build()
                .expect("failed to create SDL window");

            let mut event_pump = sdl_context
                .event_pump()
                .expect("Failed to get SDL event pump");
            let tick_duration = Duration::from_millis(tick_rate);
            let mut tick_interval = tokio::time::interval(tick_duration);

            loop {
                tokio::select! {
                    _ = tick_interval.tick() => {
                        if sdl_sender.send(AppEvent::Tick).await.is_err() {
                            break;
                        }
                    },
                    _ = async {
                        for event in event_pump.poll_iter() {
                            match event {
                                Event::KeyDown { keycode: Some(k), .. } => {
                                    if sdl_sender.send(AppEvent::Key { key: k, pressed: true }).await.is_err() {
                                        break;
                                    }
                                },
                                Event::KeyUp { keycode: Some(k), .. } => {
                                    if sdl_sender.send(AppEvent::Key { key: k, pressed: false }).await.is_err() {
                                        break;
                                    }
                                },
                                _ => {},
                            }
                        }
                    } => {},
                    _ = &mut shutdown_rx => {
                        debug!("Shutdown signal received in SDL event loop.");
                        break;
                    }
                }
                tokio::time::sleep(Duration::from_millis(5)).await;
            }
        });
        Self { receiver }
    }

    pub async fn next(&mut self) -> Option<AppEvent> {
        self.receiver.recv().await
    }
}
