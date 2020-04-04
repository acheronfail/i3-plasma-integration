use i3ipc::reply::Node;

pub fn walk_nodes(containers: &mut Vec<Node>, node: Node) {
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
