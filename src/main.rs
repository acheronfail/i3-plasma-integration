use std::error::Error;
use std::thread;
use std::time::Duration;
use std::process::exit;

use i3ipc::{I3EventListener,Subscription,I3Connection};
use i3ipc::event::{Event, inner::{WindowChange, ShutdownChange}};
use i3ipc::reply::Node;

mod cli;

const RECONNECT_MAX_RETRIES: usize = 10;
const RECONNECT_TIMEOUT_MILLIS: u64 = 200;
const PADDING: i32 = 10;

fn connect_to_i3() -> (I3Connection, I3EventListener) {
    for i in 0..RECONNECT_MAX_RETRIES {
        eprintln!("Connecting to i3 (retry {} of {})", i + 1, RECONNECT_MAX_RETRIES);
        thread::sleep(Duration::from_millis(RECONNECT_TIMEOUT_MILLIS));

        if let (Ok(connection), Ok(listener)) = (I3Connection::connect(), I3EventListener::connect()) {
            eprintln!("Connected!");
            return (connection, listener);
        }
    }

    eprintln!("Max retries reached, exiting...");
    exit(1);
}

fn find_notification_containers(root: Node) -> Vec<Node> {
    let mut containers = vec![];

    fn walk_nodes(containers: &mut Vec<Node>, node: Node) {
        if node.window_type == Some("notification".to_string()) {
            containers.push(node.clone());
        }

        for n in node.nodes {
            walk_nodes(containers, n);
        }
        for n in node.floating_nodes {
            walk_nodes(containers, n);
        }
    }

    walk_nodes(&mut containers, root);

    containers
}

fn layout_notifications(connection: &mut I3Connection) {
    let mut next_top = PADDING;

    let root = connection.get_tree().expect("failed to get i3 tree");
    let (_, _, _, root_height) = root.rect;

    let notifications = find_notification_containers(root);
    for notification in notifications {
        move_notification(connection, &notification, next_top);

        // Ensure notifications don't get placed outside the window.
        let (_, _, _, notification_height) = notification.rect;
        let maybe_next_top = next_top + notification_height + PADDING;
        if maybe_next_top < root_height - notification_height {
            next_top = maybe_next_top;
        } else {
            println!("{}, {}", next_top, root_height);
        }
    }
}

fn move_notification(connection: &mut I3Connection, node: &Node, next_top: i32) {
    let cmd = format!("[con_id=\"{}\"] move position {} {}", node.id, PADDING, next_top);
    let res = connection.run_command(&cmd).expect("failed to move node");
    for outcome in res.outcomes {
        if !outcome.success || outcome.error.is_some() {
            eprintln!("Error running command: {:?}", outcome.error);
        }
    }
}

fn listen_to_events(mut connection: I3Connection, mut listener: I3EventListener) -> ShutdownChange {
    let subs = [Subscription::Window, Subscription::Shutdown];
    listener.subscribe(&subs).expect("failed to subscribe to i3 events");

    let mut iterator = listener.listen();
    loop {
        let event = iterator.next().expect("failed to wait for next event");
        match event {
            Ok(ev) => match ev {
                Event::ShutdownEvent(info) => { return info.change; },
                Event::WindowEvent(info) => {
                    if (info.change == WindowChange::New || info.change == WindowChange::Close) && info.container.window_type == Some("notification".to_string()) {
                        layout_notifications(&mut connection);
                    }
                }
                _ => unreachable!()
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
            },
            ShutdownChange::Exit => {
                eprintln!("Received exit event from i3, exiting...");
                exit(0);
            },
            _ => {
                eprintln!("Received unknown shutdown event from i3, exiting...");
                exit(2);
            },
        }
    }
}
