use std::convert::TryInto;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Node {
    pub id: isize,
    pub name: &'static str,
    msg_count: usize,
    next_hop: Option<isize>,
    root_cost: usize,
    root_id: isize,
    pub is_discovered: bool,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Link {
    pub members: (isize, isize),
    pub cost: usize,
}

#[derive(Clone, Debug, Default)]
pub struct Tree {
    node_list: Vec<Node>,
    link_list: Vec<Link>,
}

#[derive(Default, Debug)]
pub struct SearchResult {
    pub links: Vec<Link>,
    pub cost: usize,
}

impl Node {
    pub fn new(name: &'static str) -> Self {
        Node {
            id: -1,
            name,
            msg_count: 0,
            next_hop: None,
            root_cost: 0,
            root_id: -1,
            is_discovered: false,
        }
    }
}

impl Link {
    pub fn new(members: (isize, isize), cost: usize) -> Self {
        Link { members, cost }
    }
}

impl Tree {
    pub fn new() -> Self {
        Tree {
            node_list: Vec::new(),
            link_list: Vec::new(),
        }
    }

    pub fn find_link(&mut self, a: isize, b: isize) -> Option<&Link> {
        let mut found_link: Option<&Link> = Option::default();
        for link in &self.link_list {
            if link.members.0 == a && link.members.1 == b
                || link.members.0 == b && link.members.1 == a
            {
                found_link = Some(link);
                break;
            }
        }
        found_link
    }

    pub fn find_links_from_node(&self, node_id: isize) -> Vec<&Link> {
        let link_list = &self.link_list;
        link_list
            .iter()
            .filter(|link| link.members.0 == node_id || link.members.1 == node_id)
            .collect()
    }

    pub fn add_link(&mut self, link: Link) {
        if self.find_link(link.members.0, link.members.1).is_none() {
            self.link_list.push(link);
        }
    }

    pub fn add_node(&mut self, mut node: Node) -> isize {
        for node1 in &self.node_list {
            if node1.name == node.name {
                return node1.id;
            }
        }
        let len = self.node_list.len();
        node.id = len.try_into().unwrap();
        self.node_list.push(node);
        node.id
    }

    pub fn get_node(&mut self, node_id: isize) -> Option<&mut Node> {
        let usizeindex: usize = node_id.try_into().unwrap();
        self.node_list.get_mut(usizeindex)
    }
}

impl SearchResult {
    pub fn new() -> Self {
        SearchResult {
            links: Vec::<Link>::new(),
            cost: 0,
        }
    }

    pub fn links(mut self, links: Vec<Link>) -> Self {
        self.links = links;
        self
    }

    pub fn cost(mut self, cost: usize) -> Self {
        self.cost = cost;
        self
    }
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

    #[test]
    fn find_links_from_node() {
        let mut tree = Tree::new();
        let node1 = Node::new("Node 1");
        let node2 = Node::new("Node 2");
        tree.add_node(node1);
        tree.add_node(node2);
        let link1 = Link::new((1, 1), 1);
        let link2 = Link::new((1, 2), 1);
        let link3 = Link::new((1, 3), 1);
        let link4 = Link::new((2, 2), 1);
        tree.add_link(link1);
        tree.add_link(link2);
        tree.add_link(link3);
        tree.add_link(link4);
        let links = tree.find_links_from_node(1);
        assert_eq!(links.len(), 3);
        assert!(links.contains(&&link1));
        assert!(links.contains(&&link2));
        assert!(links.contains(&&link3));
    }

    #[test]
    fn add_node() {
        let mut tree = Tree::new();
        let node = Node::new("Node1");
        tree.add_node(node);
        assert_eq!(tree.node_list[0].id, 0);
        assert_eq!(tree.node_list[0].name, node.name);
    }

    #[test]
    fn add_already_existing_node() {
        let mut tree = Tree::new();
        let node = Node::new("Node1");
        let id1 = tree.add_node(node);
        let id2 = tree.add_node(node);
        assert_eq!(id1, id2);
    }

    #[test]
    fn get_node() {
        let mut tree = Tree::new();
        let node = Node::new("Node1");
        tree.add_node(node);
        let node_retrieved = tree.get_node(0).unwrap();
        assert_eq!(node_retrieved.name, node.name);
        let node_retrieved2 = tree.get_node(2);
        assert!(node_retrieved2.is_none());
    }
}

#[cfg(test)]
mod link_test {
    use super::Link;
    #[test]
    pub fn new(){
        let link = Link::new((1,2),3);
        assert_eq!(link.members, (1,2));
        assert_eq!(link.cost, 3);
    }
}

#[cfg(test)]
mod node_test {
    use super::Node;

    #[test]
    fn test_node() {
        let node = Node::new("A");
        assert_eq!(node.root_id, -1);
        assert_eq!(node.name, "A");
    }
}