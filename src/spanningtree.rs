use rand::Rng;

pub struct Node {
    id: isize,
    #[allow(dead_code)]
    name: &'static str,
    msg_count: usize,
    next_hop: Option<isize>,
    root_cost: usize,
    root_id: isize,
}

pub struct Link {
    members: (isize, isize),
    cost: usize
}

#[derive(Default)]
pub struct Tree {
    node_list: Vec<Node>,
    root_id: Option<isize>,
    link_list: Vec<Link>
}

impl Node {
    /// Constructs a new node with an id and a specific payload.
    pub fn new(id: isize, name: &'static str) -> Self {
        Node {
            id,
            name,
            msg_count: 0,
            next_hop: None,
            root_cost: 0,
            root_id: id
        }
    }

    /// Receives a suggestion for a path to a node. If the path seems to go to the root node or is smaller than the already known path it gets accepted by the node.
    pub fn receive_suggestion(&mut self, suggested_id: isize, source_id: isize, root_cost: usize) -> bool {
        self.msg_count += 1;
        if suggested_id < self.root_id {
            self.root_cost = root_cost;
            self.next_hop = Some(source_id);
            self.root_id = suggested_id;
            return true;
        } else if suggested_id == self.root_id && root_cost < self.root_cost {
            self.root_cost = root_cost;
            self.next_hop = Some(source_id);
            return true;
        }
        false
    }
}

impl Link {
    pub fn new(members: (isize, isize), cost: usize) -> Self {
        Link {
            members,
            cost
        }
    }
}

impl Tree {
    pub fn new() -> Self {
        Tree {
            node_list: Vec::new(),
            root_id: None,
            link_list : Vec::new()
        }
    }

    /// Finds a link between two nodes. Or no link if there is none.
    pub fn find_link(&mut self, a: isize, b: isize) -> Option<&Link> {
        let mut found_link: Option<&Link> = Option::default();
        for link in &self.link_list {
            if link.members.0 == a && link.members.1 == b || link.members.0 == b && link.members.1 == a {
                found_link = Some(link);
                break;
            }
        }
        found_link
    }

    /// Returns all links that have a connection to a node
    pub fn find_links(&self, node_id: isize) -> Vec<&Link> {
        let link_list = &self.link_list;
        link_list.iter().filter(|link| link.members.0 == node_id || link.members.1 == node_id).collect()
    }

    /// Adds a link to the tree
    pub fn add_link(&mut self, link: Link) {
        if self.find_link(link.members.0, link.members.1).is_none() {
            self.link_list.push(link);
        }
    }

    /// Adds a node to the tree if this doesnt exist already. Also updates the root id of the tree if there is already one.
    pub fn add_node(&mut self, node: Node) {
        for node1 in &self.node_list {
            if node1.id == node.id {
                return;
            }
        }
        if self.root_id.is_none() || (self.root_id.is_some() && node.id < self.root_id.unwrap()) {
            self.root_id = Some(node.id);
        }
        self.node_list.push(node);
    }

    /// Gets a specific node from the tree
    pub fn get_node(&mut self, node_id: isize) -> Option<&mut Node> {
        let mut found_node: Option<&mut Node> = Option::default();
        if let Some(index) = self.node_list.iter().position(|node_item| node_item.id == node_id) {
            let node_item = self.node_list.get_mut(index).unwrap();
            found_node = Some(node_item); // Safe to unwrap due to the earlier if let
        }
        found_node
    }

    /// Runs a simulation run on the tree for the specified node.
    pub fn run_calc(&mut self, node_id: isize, recursive: bool) -> bool {
        let root_cost: usize;
        let root_id: isize;
            {   // Scoped, so the borrow of self is released after this scope ends.
                let node: Option<&Node> = self.node_list.iter().find(|n| n.id == node_id);
                match node {
                    Some(node_result) => {
                        root_cost = node_result.root_cost;
                        root_id = node_result.root_id;
                    },
                    None => return false
                }
            }
        let mut recursive_vec: Vec<isize> = Vec::new();
        for link in &self.link_list {
            if let Some(index) = self.node_list.iter().position(|node_item| node_item.id == (if node_id == link.members.0 {link.members.1} else if node_id == link.members.1 {link.members.0} else {-1})) {
                let other_node = self.node_list.get_mut(index).unwrap(); // Safe to unwrap due to the if let Some in the line before
                let accept = other_node.receive_suggestion(root_id, node_id, root_cost + link.cost);
                if accept && recursive {
                    recursive_vec.push(other_node.id);
                }
            }
        }
        for id in recursive_vec {
            self.run_calc(id, recursive);
        }
        true
    }

    /// Simulates a tree until for x iterations or even longer if there was a min_hops specified and if there are nodes in the tree that haven't been touched by the simulation enough
    pub fn simulate(&mut self, min_iterations: usize, min_hops: usize, recursive: bool) {
        while {
            for _i in 0..min_iterations {
                let randi = rand::thread_rng().gen_range(0, self.node_list.len());
                let nodeid: isize = self.node_list[randi].id;
                self.run_calc(nodeid, recursive);
            }
            self.node_list.iter().any(|node| node.msg_count <= min_hops) && min_hops != 0
        } {}
    }
}

#[cfg(test)]
mod tree_tests {
    use super::*;

    #[test]
    fn add_link() {
        let mut tree = Tree::new();
        tree.add_link(Link::new((1,2), 5));
        tree.add_link(Link::new((2,5), 8));
        assert_eq!(tree.link_list.len(), 2);
        assert_eq!(tree.link_list[0].members.1, 2);
    }

    #[test]
    fn find_link() {
        let mut tree = Tree::new();
        tree.add_link(Link::new((1,2), 5));
        tree.add_link(Link::new((2,5), 8));
        let link = tree.find_link(2, 1);
        assert_eq!(link.is_some(), true);
        let unwrapped_link = link.unwrap();
        assert_eq!(unwrapped_link.cost, 5);
        assert_eq!(tree.find_link(7, 9).is_none(), true);
    }

    #[test]
    fn find_links() {
        let mut tree = Tree::new();
        tree.add_link(Link::new((1,2), 5));
        tree.add_link(Link::new((2,5), 8));
        tree.add_link(Link::new((7,9), 2));
        let links = tree.find_links(2);
        assert_eq!(links.len(), 2);
    }

    #[test]
    fn multiple_nodes_with_same_id() {
        let mut tree = Tree::new();
        tree.add_node(Node::new(4, "E"));
        tree.add_node(Node::new(4, "E"));
        assert_eq!(tree.node_list.len, 1);
    }

    #[test]
    fn test_run_calc() {
        let mut tree = Tree::new();
        tree.add_node(Node::new(5, "A"));
        tree.add_node(Node::new(1, "B"));
        tree.add_node(Node::new(1, "B"));
        tree.add_node(Node::new(3, "C"));
        tree.add_node(Node::new(7, "D"));
        let node2 = Node::new(6, "E");
        tree.add_node(node2);
        tree.add_node(Node::new(4, "F"));
        tree.add_link(Link::new((5, 1), 10));
        tree.add_link(Link::new((5, 3), 10));
        tree.add_link(Link::new((1, 7), 15));
        tree.add_link(Link::new((1, 6), 10));
        tree.add_link(Link::new((3, 7), 3));
        tree.add_link(Link::new((3, 6), 10));
        tree.add_link(Link::new((7, 6), 2));
        tree.add_link(Link::new((7, 4), 10));
        tree.add_link(Link::new((6, 4), 2));
        assert_eq!(tree.run_calc(999, false), false);
        tree.simulate(10, 10, true);
        assert_eq!(tree.node_list.iter().all(|node| node.msg_count > 10), true);
        assert_eq!(tree.node_list.iter().all(|node| node.root_id == 1), true);
        assert_eq!(tree.get_node(3).unwrap().next_hop.unwrap(), 7);
        assert_eq!(tree.node_list[1].root_id, 1);
        for node in tree.node_list {
            println!("ID: {}, Name: {}, Messages: {}, Next Hop: {}, Root Cost: {}, Root ID: {}", node.id, node.name, node.msg_count, node.next_hop.unwrap_or(0), node.root_cost, node.root_id);
        }
    }
    
}

#[cfg(test)]
mod link_test {
    use super::*;

    #[test]
    fn test_link() {
        let link = Link::new((3,9), 4);
        assert_eq!(link.cost, 4);
        assert_eq!(link.members.0, 3);
    }
}

#[cfg(test)]
mod node_test {
    use super::*;

    #[test]
    fn test_node() {
        let node = Node::new(1, "A");
        assert_eq!(node.root_id, 1);
        assert_eq!(node.name, "A");
    }
}