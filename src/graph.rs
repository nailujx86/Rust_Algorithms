use std::convert::TryInto;

/// A node which can be part of a graph.
/// Use a tree to work with nodes.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Node {
    pub id: isize,
    pub name: &'static str,
    pub is_discovered: bool,
}

/// This structure represents a link between two nodes.
/// It also contains the path cost of that link.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Link {
    pub members: (isize, isize),
    pub cost: usize,
}

/// A tree (a graph), consisting of nodes and links between them.
#[derive(Clone, Debug, Default)]
pub struct Tree {
    node_list: Vec<Node>,
    link_list: Vec<Link>,
}

/// A result of a search algorithm for a path between two nodes,
/// containing the path cost between them and a list of links connecting them.
#[derive(Default, Debug)]
pub struct SearchResult {
    pub links: Vec<Link>,
    pub cost: usize,
}

impl Node {
    /// Create a new node object, with some sensible default values.
    ///
    /// Watch out: This node is not part of a graph yet.
    /// # Example
    /// ```
    /// use rust_algorithms::graph::Node;
    ///
    /// let node = Node::new("A");
    /// assert_eq!(node.id, -1);
    /// assert_eq!(node.name, "A");
    /// ```
    pub fn new(name: &'static str) -> Self {
        Node {
            id: -1,
            name,
            is_discovered: false,
        }
    }
}

impl Link {
    /// Creates a link between two nodes.
    /// # Example
    /// ```
    /// use rust_algorithms::graph::Link;
    ///
    /// let link = Link::new((1,2),3);
    /// assert_eq!(link.members, (1,2));
    /// assert_eq!(link.cost, 3);
    /// ```
    pub fn new(members: (isize, isize), cost: usize) -> Self {
        Link { members, cost }
    }
}

impl Tree {
    /// Creates a new tree, with empty node- and link-list.
    ///
    /// # Example
    /// ```
    /// use rust_algorithms::graph::Tree;
    ///
    /// let tree = Tree::new();
    /// ```
    pub fn new() -> Self {
        Tree {
            node_list: Vec::new(),
            link_list: Vec::new(),
        }
    }

    /// Checks for existence of and finds a link between two specific nodes.
    ///
    /// # Example
    /// ```
    /// use rust_algorithms::graph::Link;
    /// use rust_algorithms::graph::Tree;
    ///
    /// let mut tree = Tree::new();
    ///
    /// tree.add_link(Link::new((1, 2), 5));
    /// tree.add_link(Link::new((2, 5), 8));
    ///
    /// let link = tree.find_link(2, 1).unwrap();
    ///
    /// assert_eq!(link.cost, 5);
    ///
    /// assert_eq!(tree.find_link(7, 9).is_none(), true);
    /// ```
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

    /// Finds all links in which the given node is part of.
    /// It does not check for/change the orientation of the link.
    ///
    /// # Example
    /// ```
    /// use rust_algorithms::graph::Tree;
    /// use rust_algorithms::graph::Link;
    /// use rust_algorithms::graph::Node;
    ///
    /// let mut tree = Tree::new();
    ///
    /// let node1 = Node::new("Node 1");
    /// let node2 = Node::new("Node 2");
    /// tree.add_node(node1);
    /// tree.add_node(node2);
    ///
    /// let link1 = Link::new((1, 1), 1);
    /// let link2 = Link::new((2, 2), 1);
    /// tree.add_link(link1);
    /// tree.add_link(link2);
    ///
    /// let links = tree.find_links_from_node(1);
    /// assert_eq!(links.len(), 1);
    /// assert!(links.contains(&&link1));
    /// ```
    pub fn find_links_from_node(&self, node_id: isize) -> Vec<&Link> {
        let link_list = &self.link_list;
        link_list
            .iter()
            .filter(|link| link.members.0 == node_id || link.members.1 == node_id)
            .collect()
    }

    /// Adds a link to the graph, if it is not a part of the tree yet.
    ///
    /// If the link is already a part of the tree, the tree remains unchanged.
    ///
    /// # Example
    /// ```
    /// use rust_algorithms::graph::Tree;
    /// use rust_algorithms::graph::Link;
    ///
    /// let mut tree = Tree::new();
    /// let link = Link::new((1, 2), 5);
    /// tree.add_link(link);
    ///
    /// assert_eq!(tree.find_link(link.members.0, link.members.1).unwrap(), &link);
    /// ```
    pub fn add_link(&mut self, link: Link) {
        if self.find_link(link.members.0, link.members.1).is_none() {
            self.link_list.push(link);
        }
    }

    /// Adds a node to the graph, if it is not a part of the tree yet.
    ///
    /// This operation returns the nodes ID inside of the tree, as it cannot be known before adding the node to the tree.
    /// This id can then be used to reference the node later. You can assign it back to the node.
    /// The nodes ID does NOT automatically change, as it gets cloned into the tree instead of moved.
    /// If the node is already a part of the tree, the tree remains unchanged.
    ///
    /// # Example
    /// ```
    /// use rust_algorithms::graph::Tree;
    /// use rust_algorithms::graph::Node;
    ///
    /// let mut tree = Tree::new();
    ///
    /// let mut node = Node::new("Node1");
    /// let node_id = tree.add_node(node);
    ///
    /// assert_eq!(tree.get_node(node_id).unwrap().name, node.name);
    /// ```
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

    /// Retrieves a node from the tree by its id.
    /// Returns None, if no node with that id is present inside the tree instead of panicing.
    ///
    /// # Example:
    /// ```
    /// use rust_algorithms::graph::Tree;
    /// use rust_algorithms::graph::Node;
    ///
    /// let mut tree = Tree::new();
    /// let node = Node::new("Node1");
    /// tree.add_node(node);
    ///
    /// let node_retrieved = tree.get_node(0).unwrap();
    /// assert_eq!(node_retrieved.name, node.name);
    ///
    /// let node_not_present = tree.get_node(2);
    /// assert!(node_not_present.is_none());
    /// ```
    pub fn get_node(&mut self, node_id: isize) -> Option<&mut Node> {
        let usizeindex: usize = node_id.try_into().unwrap();
        self.node_list.get_mut(usizeindex)
    }
}

impl SearchResult {
    /// Creates a new search result to return from a function.
    ///
    /// The result contains a list of links, which summarize the path from start to finish.
    /// Those links are ordered, but their individual orientation might be scrambled.
    /// An Example would be a link from node 1 to node 3, consisting of (1, 2) , (3, 2) instead of (1, 2) , (2 , 3)
    ///
    /// 
    /// This struct has a builder pattern, so it may be initialized as follows:
    /// # Example
    /// ```
    /// use rust_algorithms::graph::SearchResult;
    /// use rust_algorithms::graph::Link;
    /// 
    /// let result = SearchResult::new();

    /// let testlink = Link::new((1, 1), 2);
    /// let buildresult = SearchResult::new().cost(2).links(vec![testlink]);
    ///
    /// assert_eq!(buildresult.cost, 2);
    /// assert_eq!(buildresult.links, vec!(testlink));
    /// ```
    pub fn new() -> Self {
        SearchResult {
            links: Vec::<Link>::new(),
            cost: 0,
        }
    }

    /// Part of the builder pattern for a Link.
    /// 
    /// For more information see the SearchResult::new() documentation.
    pub fn links(mut self, links: Vec<Link>) -> Self {
        self.links = links;
        self
    }
    
    /// Part of the builder pattern for a Link.
    /// 
    /// For more information see the SearchResult::new() documentation.
    pub fn cost(mut self, cost: usize) -> Self {
        self.cost = cost;
        self
    }
}

#[cfg(test)]
mod tree_tests {
    use super::*;

    #[test]
    fn new() {
        let tree = Tree::new();
        assert_eq!(tree.link_list.len(), 0);
        assert_eq!(tree.node_list.len(), 0);
    }

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
mod link_tests {
    use super::Link;
    #[test]
    pub fn new() {
        let link = Link::new((1, 2), 3);
        assert_eq!(link.members, (1, 2));
        assert_eq!(link.cost, 3);
    }
}

#[cfg(test)]
mod node_tests {
    use super::Node;

    #[test]
    fn test_node() {
        let node = Node::new("A");
        assert_eq!(node.id, -1);
        assert_eq!(node.name, "A");
    }
}

#[cfg(test)]
mod search_result_tests {
    use super::Link;
    use super::SearchResult;

    #[test]
    fn new() {
        let result = SearchResult::new();
        assert_eq!(result.cost, 0);
        assert_eq!(result.links.len(), 0);

        let testlink = Link::new((1, 1), 2);
        let buildresult = SearchResult::new().cost(2).links(vec![testlink]);

        assert_eq!(buildresult.cost, 2);
        assert_eq!(buildresult.links, vec!(testlink));
    }
}
