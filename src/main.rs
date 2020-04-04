use std::error::Error;
use std::process::exit;
use std::thread;
use std::time::Duration;

use i3ipc::event::{
    inner::{ShutdownChange, WindowChange},
    Event,
};
use i3ipc::{I3Connection, I3EventListener, Subscription};

mod cli;
mod notifications;
mod util;

const RECONNECT_MAX_RETRIES: usize = 10;
const RECONNECT_TIMEOUT_MILLIS: u64 = 200;

fn connect_to_i3() -> (I3Connection, I3EventListener) {
    for i in 1..=RECONNECT_MAX_RETRIES {
        eprintln!(
            "Connecting to i3 (retry {} of {})",
            i, RECONNECT_MAX_RETRIES
        );
        thread::sleep(Duration::from_millis(RECONNECT_TIMEOUT_MILLIS));

        if let (Ok(c), Ok(l)) = (I3Connection::connect(), I3EventListener::connect()) {
            eprintln!("Connected!");
            return (c, l);
        }
    }

    eprintln!("Max retries reached, exiting...");
    exit(1);
}

fn listen_to_events(mut connection: I3Connection, mut listener: I3EventListener) -> ShutdownChange {
    let subs = [Subscription::Window, Subscription::Shutdown];
    listener
        .subscribe(&subs)
        .expect("failed to subscribe to i3 events");

    let mut iterator = listener.listen();
    loop {
        let event = iterator.next().expect("failed to wait for next event");
        match event {
            Ok(ev) => match ev {
                Event::ShutdownEvent(info) => {
                    return info.change;
                }
                Event::WindowEvent(info) => {
                    if (info.change == WindowChange::New || info.change == WindowChange::Close)
                        && info.container.window_type == Some("notification".to_string())
                    {
                        notifications::layout(&mut connection);
                    }
                }
                _ => unreachable!(),
            },
            Err(e) => {
                eprintln!("Error receiving event from i3: {:?}", e.source());
                exit(1);
            }
        }
    }
}

fn main() {
    let _ = cli::build_app().get_matches();

    loop {
        let (connection, listener) = connect_to_i3();
        match listen_to_events(connection, listener) {
            ShutdownChange::Restart => {
                eprintln!("Received restart event from i3");
                continue;
            }
            ShutdownChange::Exit => {
                eprintln!("Received exit event from i3, exiting...");
                exit(0);
            }
            _ => {
                eprintln!("Received unknown shutdown event from i3, exiting...");
                exit(2);
            }
        }
    }
}
