use std::convert::TryInto;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Node {
    id: isize,
    name: &'static str,
    msg_count: usize,
    next_hop: Option<isize>,
    root_cost: usize,
    root_id: isize,
    is_discovered: bool,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Link {
    members: (isize, isize),
    cost: usize,
}

#[derive(Clone, Debug, Default)]
pub struct Tree {
    node_list: Vec<Node>,
    link_list: Vec<Link>,
}

#[derive(Default, Debug)]
pub struct SearchResult {
    links: Vec<Link>,
    cost: usize,
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

    pub fn add_node(&mut self, mut node: Node)-> isize{
        for node1 in &self.node_list {
            if node1.name == node.name {
                return node1.id
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
}

pub fn find_path_to_element(
    mut tree: Tree,
    start_node_id: isize,
    search_node_id: isize,
) -> Option<SearchResult> {
    use std::collections::VecDeque;

    if start_node_id == search_node_id {
        let mut s = SearchResult::new();
        s.cost = 0;
        let link = Link::new((start_node_id, search_node_id), 0);
        let mut vector = Vec::new();
        vector.push(link);
        s.links = vector;
        return Some(s);
    }

    // the first int is the nodes id, the second the depth in the tree, the third the link that lead to this node
    let mut queue = VecDeque::<(isize, usize, Vec<Link>)>::new();

    // the first link leads from the first element to itself with no cost. It is just there to provide any link
    let mut vector = Vec::new();
    vector.push(Link::new((start_node_id, start_node_id), 0));
    queue.push_front((start_node_id, 1, vector));

    let mut start_node = match tree.get_node(start_node_id) {
        Some(node) => node,
        None => {
            return None;
        }
    };
    start_node.is_discovered = true;

    while !queue.is_empty() {
        // The while loop guarantees that there is something to pop, so unwrapping is safe
        let current_queue_element = queue.pop_front().unwrap();
        let current_node = current_queue_element.0;

        if current_node == search_node_id {
            let mut cost = 0;
            for link in current_queue_element.2.iter() {
                cost += link.cost;
            }
            let mut s = SearchResult::new();
            s.links = current_queue_element.2;
            s.cost = cost;
            return Some(s);
        } else {
            let mytree = tree.clone();
            let links = mytree.find_links_from_node(current_node);
            for link in links {
                //ignore circular links (from object to itself)
                if link.members.0 != link.members.1 {
                    let found_node: isize = if link.members.0 == current_node {
                        link.members.1
                    } else {
                        link.members.0
                    };
                    if let Some(node) = tree.get_node(found_node) {
                        if !node.is_discovered {
                            let mut new_vector = current_queue_element.2.clone();
                            new_vector.push(*link);
                            queue.push_back((found_node, current_queue_element.1 + 1, new_vector));
                            node.is_discovered = true;
                        }
                    }
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

    #[test]
    fn find_links_from_node(){
        let mut tree = Tree::new();
        let node1 = Node::new("Node 1");
        let node2 = Node::new("Node 2");
        tree.add_node(node1);
        tree.add_node(node2);
        let link1 = Link::new((1,1),1);
        let link2 = Link::new((1,2),1);
        let link3 = Link::new((1,3),1);
        let link4 = Link::new((2,2),1);
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
    fn add_node(){
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
    fn get_node(){
        let mut tree = Tree::new();
        let node = Node::new("Node1");
        tree.add_node(node);
        println!("{}",node.id);
        let node_retrieved = tree.get_node(0).unwrap();
        assert_eq!(node_retrieved.name, node.name);
        let node_retrieved2 = tree.get_node(2);
        assert!(node_retrieved2.is_none());
    }
}

#[cfg(test)]
mod link_test {}

#[cfg(test)]
mod node_test {
    use super::*;

    #[test]
    fn test_node() {
        let node = Node::new("A");
        assert_eq!(node.root_id, -1);
        assert_eq!(node.name, "A");
    }
}

#[cfg(test)]
mod discover_test {
    use super::*;

    #[test]
    fn test_discover_no_start_element() {
        let mut tree = Tree::new();
        let node1 = Node::new("Node 1");
        tree.add_node(node1);
        assert_eq!(find_path_to_element(tree, 2, 1).is_none(), true);
    }

    #[test]
    fn test_discover_no_target_element() {
        let mut tree = Tree::new();
        let node1 = Node::new("Node 1");
        tree.add_node(node1);
        assert_eq!(find_path_to_element(tree, 1, 2).is_none(), true);
    }

    #[test]
    fn test_discover_no_start_and_target_element() {
        let tree = Tree::new();
        assert_eq!(find_path_to_element(tree, 1, 2).is_none(), true);
    }

    #[test]
    fn test_discover_no_link() {
        let mut tree = Tree::new();
        let node1 = Node::new("Node 1");
        let node2 = Node::new("Node 1");
        tree.add_node(node1);
        tree.add_node(node2);
        tree.add_link(Link::new((1, 3), 1));
        assert_eq!(find_path_to_element(tree, 1, 2).is_none(), true);
    }

    #[test]
    fn test_discover_element_to_self() {
        let tree = Tree::new();
        let result = find_path_to_element(tree, 1, 1).unwrap();
        assert_eq!(result.links[0], Link::new((1, 1), 0));
        assert_eq!(result.cost, 0);
    }

    #[test]
    fn test_discover_two_elements() {
        let mut tree = Tree::new();
        let mut node1 = Node::new("Node 1");
        let mut node2 = Node::new("Node 2");
        node1.id = tree.add_node(node1);
        node2.id = tree.add_node(node2);
        let link1 = Link::new((node1.id, node2.id), 5);
        tree.add_link(link1);
        let result = find_path_to_element(tree, node1.id, node2.id).unwrap();
        let link0 = Link::new((node1.id, node1.id), 0);
        assert_eq!(result.links[0], link0);
        assert_eq!(result.links[1], link1);
        assert_eq!(result.cost, 5);
    }

    #[test]
    fn test_discover_multiple_elements() {
        let mut tree = Tree::new();
        let mut node1 = Node::new("Node 1");
        let mut node2 = Node::new("Node 2");
        let mut node3 = Node::new("Node 3");
        let mut node4 = Node::new("Node 4");
        let mut node5 = Node::new("Node 5");
        let mut node6 = Node::new("Node 6");
        let mut node7 = Node::new("Node 7");
        node1.id = tree.add_node(node1);
        node2.id =tree.add_node(node2);
        node3.id =tree.add_node(node3);
        node4.id =tree.add_node(node4);
        node5.id =tree.add_node(node5);
        node6.id =tree.add_node(node6);
        node7.id =tree.add_node(node7);
        let link1 = Link::new((node1.id, node3.id), 1);
        let link2 = Link::new((node1.id, node2.id), 1);
        let link3 = Link::new((node2.id, node4.id), 2);
        let link4 = Link::new((node3.id, node5.id), 1);
        let link5 = Link::new((node3.id, node6.id), 1);
        let link6 = Link::new((node4.id, node7.id), 1);
        tree.add_link(link1);
        tree.add_link(link2);
        tree.add_link(link3);
        tree.add_link(link4);
        tree.add_link(link5);
        tree.add_link(link6);
        let result = find_path_to_element(tree, node1.id, node7.id).unwrap();
        assert_eq!(result.cost, 4);
        assert_eq!(result.links[1], Link::new((node1.id, node2.id), 1));
        assert_eq!(result.links[2], Link::new((node2.id, node4.id), 2));
        assert_eq!(result.links[3], Link::new((node4.id, node7.id), 1));
    }

    #[test]
    fn discover_elements_with_loose_end_links() {
        let mut tree = Tree::new();
        let mut node1 = Node::new("Node 1");
        let mut node2 = Node::new("Node 2");
        node1.id = tree.add_node(node1);
        node2.id = tree.add_node(node2);
        let link1 = Link::new((node1.id, node2.id), 1);
        let link2 = Link::new((1, 65999), 1);
        tree.add_link(link1);
        tree.add_link(link2);
        let result = find_path_to_element(tree, node1.id, node2.id).unwrap();
        assert_eq!(result.cost, 1);
        assert_eq!(result.links[1], Link::new((node1.id, node2.id), 1));
    }

    #[test]
    fn discover_elements_with_objects_linked_to_themselves() {
        let mut tree = Tree::new();
        let mut node1 = Node::new("Node 1");
        let mut node2 = Node::new("Node 2");
        node1.id = tree.add_node(node1);
        node2.id = tree.add_node(node2);
        let link1 = Link::new((node1.id, node2.id), 5);
        let link2 = Link::new((node1.id, node1.id), 5);
        tree.add_link(link1);
        tree.add_link(link2);
        let result = find_path_to_element(tree, node1.id, node2.id).unwrap();
        assert_eq!(result.links[1], link1);
        assert_eq!(result.cost, 5);
    }

    #[test]
    fn discover_elements_with_circular_trees() {
        let mut tree = Tree::new();
        let mut node1 = Node::new("Node 1");
        let mut node2 = Node::new("Node 2");
        let mut node3 = Node::new("Node 3");
        let mut node4 = Node::new("Node 4");
        let mut node5 = Node::new("Node 5");
        let mut node6 = Node::new("Node 6");
        let mut node7 = Node::new("Node 7");
        node1.id = tree.add_node(node1);
        node2.id = tree.add_node(node2);
        node3.id = tree.add_node(node3);
        node4.id = tree.add_node(node4);
        node5.id = tree.add_node(node5);
        node6.id = tree.add_node(node6);
        node7.id = tree.add_node(node7);
        let link1 = Link::new((node1.id, node3.id), 1);
        let link2 = Link::new((node1.id, node2.id), 1);
        let link3 = Link::new((node2.id, node4.id), 2);
        let link4 = Link::new((node3.id, node5.id), 1);
        let link5 = Link::new((node5.id, node1.id), 1);
        let link6 = Link::new((node4.id, node7.id), 1);
        tree.add_link(link1);
        tree.add_link(link2);
        tree.add_link(link3);
        tree.add_link(link4);
        tree.add_link(link5);
        tree.add_link(link6);
        let result = find_path_to_element(tree, node1.id, node7.id).unwrap();
        assert_eq!(result.cost, 4);
        assert_eq!(result.links[1], Link::new((node1.id, node2.id), 1));
        assert_eq!(result.links[2], Link::new((node2.id, node4.id), 2));
        assert_eq!(result.links[3], Link::new((node4.id, node7.id), 1));
    }
}
