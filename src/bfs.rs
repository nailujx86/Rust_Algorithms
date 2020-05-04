use std::cmp::Ordering;

#[derive(Eq)]
struct Node {
    id: usize,
    name: &'static str,
    msg_count: usize,
    next_hop: isize,
    root_cost: usize,
    root_id: usize,
    is_discovered: bool,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

struct Link {
    members: (usize, usize),
    cost: usize,
}

struct Tree {
    node_list: Vec<Node>,
    root_id: isize,
    link_list: Vec<Link>,
}

impl Node {
    fn new(id: usize, name: &'static str) -> Self {
        Node {
            id,
            name,
            msg_count: 0,
            next_hop: -1,
            root_cost: 0,
            root_id: id,
            is_discovered: false
        }
    }
}

impl Link {
    fn new(members: (usize, usize), cost: usize) -> Self {
        Link { members, cost }
    }
}

impl Tree {
    fn new() -> Self {
        Tree {
            node_list: Vec::new(),
            root_id: -1,
            link_list: Vec::new(),
        }
    }

    fn find_link(&mut self, a: usize, b: usize) -> Option<&Link> {
        let mut foundLink: Option<&Link> = Option::default();
        for link in &self.link_list {
            if link.members.0 == a && link.members.1 == b
                || link.members.0 == b && link.members.1 == a
            {
                foundLink = Some(link);
                break;
            }
        }
        foundLink
    }

    fn add_link(&mut self, link: Link) {
        if self.find_link(link.members.0, link.members.1).is_none() {
            self.link_list.push(link);
        }
    }

    fn get_connected_nodes(&mut self, node: Node) -> Vec<usize> {
        let mut links = Vec::new();
        for link in &self.link_list
        {
            if  link.members.1 == node.id {
                links.push(link.members.0);
            }
            if link.members.0 == node.id {
                links.push(link.members.1);
            }
        }
        links
    }

    fn find_node_by_id(&self, id: usize) -> Option<Node> {
        for node in self.node_list {
            if node.id == id {
                return Some(node)
            }
        }
        None
    }
}

fn discover_elements(mut tree: Tree, start_node_id: usize, search_node_id: usize) -> Option<Node>{

    use std::collections::VecDeque;

    // mark all nodes as not visited yet
    for node in tree.node_list {
        node.is_discovered = false;
    }

    //start with one node (the first one in this instance)
    let start_node = tree.find_node_by_id(start_node_id);
    let mut start_node = match start_node {
        Some(node) => node,
        None => return None
    };

    let mut queue = VecDeque::<Node>::new();

    start_node.is_discovered = true;

    queue.push_back(start_node);

    while queue.len() != 0 {
        let current_node = queue.pop_front();
        // the while loop guarantees that there is something to pop, so unwrapping is safe
        let current_node: Node = current_node.unwrap();

        if current_node.id == search_node_id {
            return Some(current_node)
        } else {
            for node in tree.get_connected_nodes(current_node) {
                // thats not good, and shall be replaced by a better search algorithm or even a key/Value list or a pointer based solution 
                let foundNode = tree.find_node_by_id(node);
                let foundNode = match foundNode {
                    Some(node) => node,
                    None => panic!("Error: Node not found which was supposed to be found.")
                };
                

                if foundNode.is_discovered == false {
                    //queue.append(node);
                }
            }
        }
    }

    None
}

#[cfg(test)]
mod tree_tests {
    use super::*;

    #[test]
    fn add_link() {
        let mut tree = Tree::new();
        tree.add_link(Link::new((1, 2), 5));
        tree.add_link(Link::new((2, 5), 8));
        assert_eq!(tree.link_list.len(), 2);
        assert_eq!(tree.link_list[0].members.1, 2);
    }

    #[test]
    fn find_link() {
        let mut tree = Tree::new();
        tree.add_link(Link::new((1, 2), 5));
        tree.add_link(Link::new((2, 5), 8));
        let link = tree.find_link(2, 1);
        assert_eq!(link.is_some(), true);
        let unwrapped_link = link.unwrap();
        assert_eq!(unwrapped_link.cost, 5);
        assert_eq!(tree.find_link(7, 9).is_none(), true);
    }
}

#[cfg(test)]
mod link_test {}

#[cfg(test)]
mod node_test {
    use super::*;

    #[test]
    fn test_node() {
        let mut node = Node::new(1, "A");
        assert_eq!(node.root_id, 1);
        assert_eq!(node.name, "A");
    }
}
