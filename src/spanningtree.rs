
struct Node {
    id: usize,
    name: char,
    msg_count: usize,
    next_hop: isize,
    root_cost: usize,
    root_id: usize,
}

#[derive(Debug)]
struct Link {
    members: (usize, usize),
    cost: usize
}

struct Tree {
    node_list: Vec<Node>,
    root_id: isize,
    link_list: Vec<Link>
}

impl Node {
    fn new(id: usize, name: char) -> Self {
        Node {
            id,
            name,
            msg_count: 0,
            next_hop: -1,
            root_cost: 0,
            root_id: id
        }
    }
}

impl Link {
    fn new(members: (usize, usize), cost: usize) -> Self {
        Link {
            members,
            cost
        }
    }
}

impl Tree {
    fn new() -> Self {
        Tree {
            node_list: Vec::new(),
            root_id: -1,
            link_list : Vec::new()
        }
    }

    fn find_link(&mut self, a: usize, b: usize) -> Option<&Link> {
        let mut foundLink: Option<&Link> = Option::default();
        for link in &self.link_list {
            if link.members.0 == a && link.members.1 == b || link.members.0 == b && link.members.1 == a {
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
}

#[cfg(test)]
mod link_test {
    
}