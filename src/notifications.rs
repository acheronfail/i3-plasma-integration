use i3ipc::reply::Node;
use i3ipc::I3Connection;

use crate::util::walk_nodes;

const PADDING: i32 = 10;

fn find_notification_containers(root: Node) -> Vec<Node> {
  let mut containers = vec![];
  walk_nodes(&mut containers, root);

  containers
}

fn move_notification(connection: &mut I3Connection, node: &Node, next_top: i32) {
  let cmd = format!(
    "[con_id=\"{}\"] move position {} {}",
    node.id, PADDING, next_top
  );
  let res = connection.run_command(&cmd).expect("failed to move node");
  for outcome in res.outcomes {
    if !outcome.success || outcome.error.is_some() {
      eprintln!("Error running command: {:?}", outcome.error);
    }
  }
}

pub fn layout(connection: &mut I3Connection) {
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
