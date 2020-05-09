use rand::Rng;

pub struct Node {
    pub id: isize,
    #[allow(dead_code)]
    pub name: &'static str,
    pub msg_count: usize,
    pub next_hop: Option<isize>,
    pub root_cost: usize,
    pub root_id: isize,
}

pub struct Link {
    pub members: (isize, isize),
    pub cost: usize
}

#[derive(Default)]
pub struct Tree {
    node_list: Vec<Node>,
    root_id: Option<isize>,
    link_list: Vec<Link>
}

impl Node {

    /// The Node is to be used within a Tree.
    /// In a Spanningtree it represents one participant, therefore a node.
    /// For identification purposes a payload can be supplied in the form of a String.
    /// The ID of the Node also represents the weight of it.
    /// A Spanningtree is balanced so that the node with the lowest weight is the root node.
    /// ```text
    /// *-*  * <- high weight (high id)
    ///   |  |
    ///   *--*
    ///      |
    ///      * <- low weight, therefore root
    /// ```
    /// 
    /// # Example
    /// ```
    /// use rust_algorithms::spanningtree::Node;
    /// let node: Node = Node::new(1, "Node Numero Uno");
    /// ```
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
    /// 
    /// Since a tree is to be balanced towards the node with the lowest weight,
    /// a node will only accept a path suggestion to the assumed root of the tree
    /// if either the weight of the assumed root is lower than the currently known,
    /// or the path size to the already known root is lower than the one currently saved.
    /// 
    /// Returns true if the node accepted the suggestion and either changed it's path to root
    /// or the cost it to the already known root is smaller than the current cost.
    /// 
    /// # Example
    /// ```
    /// use rust_algorithms::spanningtree::Node;
    /// let mut node: Node = Node::new(3, "Node Three");
    /// let accepted: bool = node.receive_suggestion(2, 8, 10);
    /// assert_eq!(accepted, true);
    /// ```
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

    /// Returns a link, if there is one, between node a and node b, identified by their ids.
    /// 
    /// # Example
    /// ```
    /// use rust_algorithms::spanningtree::*;
    /// let mut tree: Tree = Tree::new();
    /// tree.add_link(Link::new((8, 1), 6));
    /// let link_opt: Option<&Link> = tree.find_link(1,8);
    /// assert_eq!(link_opt.is_some(), true);
    /// ```
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

    /// Returns all links that have a connection to a node identified by their id.
    /// 
    /// # Example
    /// ```
    /// use rust_algorithms::spanningtree::*;
    /// let mut tree: Tree = Tree::new();
    /// tree.add_link(Link::new((8, 1), 6));
    /// tree.add_link(Link::new((4, 1), 3));
    /// let links: Vec<&Link> = tree.find_links(1);
    /// assert_eq!(links.len(), 2);
    /// ```
    pub fn find_links(&self, node_id: isize) -> Vec<&Link> {
        let link_list = &self.link_list;
        link_list.iter().filter(|link| link.members.0 == node_id || link.members.1 == node_id).collect()
    }

    /// Adds a link to the tree, if it doesnt exist yet.
    /// 
    /// # Example
    /// ```
    /// use rust_algorithms::spanningtree::*;
    /// let mut tree: Tree = Tree::new();
    /// tree.add_link(Link::new((8, 1), 6));
    /// assert_eq!(tree.find_links(1).len(), 1);
    /// ```
    pub fn add_link(&mut self, link: Link) {
        if self.find_link(link.members.0, link.members.1).is_none() {
            self.link_list.push(link);
        }
    }

    /// Adds a node to the tree if this doesnt exist already. 
    /// Also updates the root id of the tree if there is already one.
    /// 
    /// # Example
    /// ```
    /// use rust_algorithms::spanningtree::*;
    /// let mut tree: Tree = Tree::new();
    /// tree.add_node(Node::new(2, "Second Node"));
    /// assert_eq!(tree.get_node(2).is_some(), true);
    /// assert_eq!(tree.get_node(2).unwrap().name, "Second Node");
    /// ```
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

    /// Gets a specific node from the tree, specified by their id, wrapped in an Option.
    /// 
    /// # Example
    /// ```
    /// use rust_algorithms::spanningtree::*;
    /// let mut tree: Tree = Tree::new();
    /// tree.add_node(Node::new(2, "Second Node"));
    /// let node_opt: Option<&mut Node> = tree.get_node(2);
    /// assert_eq!(node_opt.is_some(), true);
    /// assert_eq!(node_opt.unwrap().name, "Second Node");
    /// ```
    pub fn get_node(&mut self, node_id: isize) -> Option<&mut Node> {
        let mut found_node: Option<&mut Node> = Option::default();
        if let Some(index) = self.node_list.iter().position(|node_item| node_item.id == node_id) {
            let node_item = self.node_list.get_mut(index).unwrap();
            found_node = Some(node_item); // Safe to unwrap due to the earlier if let
        }
        found_node
    }

    /// Runs a simulation run on the tree for the specified node.
    /// 
    /// The node it runs on passes its knowledge about the root node 
    /// and the cost to it to all its neigbouring nodes.
    /// If run_calc is called with the recursive option set to true it will run recursively =>
    /// if a node accepts a new root/a lower cost to the root run_calc will run on this node again
    /// and will spread the information to their neighbouring nodes and so on.
    /// 
    /// Returns false if no node with that id has been found.
    /// 
    /// # Recursive Run Example
    /// ```text
    ///       °-°      °-°3        
    ///         |        |   
    ///start->4*-°5 => 4*-*5
    ///         |        |
    ///        8°       8*
    /// ```
    ///
    /// # Example
    /// ```
    /// use rust_algorithms::spanningtree::*;
    /// let mut tree: Tree = Tree::new();
    /// tree.add_node(Node::new(4, "Second Node"));
    /// tree.add_node(Node::new(2, "Second Node"));
    /// tree.add_node(Node::new(3, "Second Node"));
    /// tree.add_link(Link::new((2,4), 5));
    /// tree.add_link(Link::new((3,2), 8));
    /// let run: bool = tree.run_calc(2, false);
    /// assert_eq!(run, true);
    /// assert_eq!(tree.get_node(4).unwrap().root_id, 2);
    /// assert_eq!(tree.get_node(3).unwrap().root_cost, 8);
    /// ```
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

    /// Simulates a tree until for x iterations or even longer if there was a min_hops specified and if there are nodes in the tree that haven't been touched by the simulation enough.
    /// 
    /// In a Spanningtree Algorithm implemented by Switches in a network a Switch will randomly send out a suggestion regarding its known root and weight.
    /// Therefore we'll try to simulate that by selecting a random node out of our nodes and running run_calc on it.
    /// In a "real" implementation a node would keep on sending until it hasn't found out about a new root node for a while.
    /// Here we can utilize a counter on each node to count how often it has been visited. (min_hop)
    /// We can use that solely or combine it with a iteration count. min_iterations defines how many nodes we should let the simulation run on at least.
    /// 
    /// # Example
    /// ```
    /// use rust_algorithms::spanningtree::*;
    /// let mut tree: Tree = Tree::new();
    /// tree.add_node(Node::new(4, "Second Node"));
    /// tree.add_node(Node::new(2, "Second Node"));
    /// tree.add_node(Node::new(3, "Second Node"));
    /// tree.add_link(Link::new((2,4), 5));
    /// tree.add_link(Link::new((3,2), 8));
    /// tree.simulate(10, 100, false);
    /// assert_eq!(tree.get_node(4).unwrap().msg_count > 100, true);
    /// assert_eq!(tree.get_node(4).unwrap().root_id, 2);
    /// ```
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
        assert_eq!(tree.node_list.len(), 1);
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
        assert_eq!(tree.run_calc(3, false), true);
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