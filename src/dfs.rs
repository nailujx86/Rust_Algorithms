use crate::graph::*;

/// A function to search for the path to a node using the [Depth-first search](https://en.wikipedia.org/wiki/Depth-first_search) method.
/// 
/// This function takes two node ids, for the start and target node, and computes a path between them.
/// This path consists of a Vec of Links.
/// The first link is always from the first element to itself.
/// # Example:
/// ```rust
/// use rust_algorithms::graph::*;
/// use rust_algorithms::dfs::*;
/// 
/// let mut graph = Graph::new();
/// 
/// let mut node1 = Node::new("Node 1");
/// let mut node2 = Node::new("Node 2");
/// node1.id = graph.add_node(node1);
/// node2.id = graph.add_node(node2);
/// 
/// let link1 = Link::new((node1.id, node2.id), 5);
/// graph.add_link(link1);
/// 
/// let result = dfs_search_node(graph, node1.id, node2.id).unwrap();
/// let link0 = Link::new((node1.id, node1.id), 0);
/// 
/// assert_eq!(result.links[0], link0);
/// assert_eq!(result.links[1], link1);
/// assert_eq!(result.cost, 5);
/// ```
pub fn dfs_search_node(
    mut graph: Graph,
    start_node_id: isize,
    search_node_id: isize,
) -> Option<SearchResult> {

    // if start node is the node searched for, return a link to itself
    if start_node_id == search_node_id {
        return Some(
            SearchResult::new()
                .links(vec![Link::new((start_node_id, search_node_id), 0)])
                .cost(0),
        );
    }

    // call the recursive function with the link from the start element to itself as first link
    let result = search_node_recursive(&mut graph, start_node_id, search_node_id, vec!(Link::new((0,0),0)));
    
    // compute the total link cost and return the result
    match result {
        Some(mut res) => {
            let mut cost: usize = 0;
            for link in &res.links {
                cost += link.cost;
            }
            res.cost = cost;
            Some(res)
        },
        None => {
            None
        }
    }
}

fn search_node_recursive(
    graph: &mut Graph,
    start_node_id: isize,
    search_node_id: isize,
    link_chain: Vec<Link>,
) -> Option<SearchResult> {

    // Abort condition: check if the current node is the one searched for.
    if start_node_id == search_node_id {
        return Some(SearchResult::new().links(link_chain).cost(0));
    }

    // make a new stack
    let mut stack: Vec<(&Link, isize)> = Vec::new();
    let mygraph = graph.clone();

    // find all links going out from the current start_node
    for link in mygraph.find_links_from_node(start_node_id) {
        let other_node = if link.members.0 == start_node_id {
            link.members.1
        } else {
            link.members.0
        };
        // if a node was found on the other end of the link, push it onto the stack
        if let Some(node) = graph.get_node(other_node) {
            if !node.is_discovered {
                stack.push((link, other_node));
                node.is_discovered = true;
            }
        }
    }

    // visit every element on the stack
    while !stack.is_empty() {
        // we can safely unwrap(), as we checked for is_empty() in the while loop
        let stack_element = stack.pop().unwrap();
        
        // clone the link chain and add the link to the new element to it
        let mut new_vector = link_chain.clone();
        new_vector.push(*stack_element.0);

        // recursively call the function for the new element
        if let Some(result) =
            search_node_recursive(graph, stack_element.1, search_node_id, new_vector)
        {
            return Some(result);
        }
    }

    // if the stack is empty and all recursive functions have been processed, all visitable nodes have been visited, and no result has been found.
    None
}

#[cfg(test)]
mod discover_test {
    use super::*;

    #[test]
    fn test_discover_no_start_element() {
        let mut graph = Graph::new();
        let node1 = Node::new("Node 1");
        graph.add_node(node1);
        assert_eq!(dfs_search_node(graph, 2, 1).is_none(), true);
    }

    #[test]
    fn test_discover_no_target_element() {
        let mut graph = Graph::new();
        let node1 = Node::new("Node 1");
        graph.add_node(node1);
        assert_eq!(dfs_search_node(graph, 1, 2).is_none(), true);
    }

    #[test]
    fn test_discover_no_start_and_target_element() {
        let graph = Graph::new();
        assert_eq!(dfs_search_node(graph, 1, 2).is_none(), true);
    }

    #[test]
    fn test_discover_no_link() {
        let mut graph = Graph::new();
        let node1 = Node::new("Node 1");
        let node2 = Node::new("Node 1");
        graph.add_node(node1);
        graph.add_node(node2);
        graph.add_link(Link::new((1, 3), 1));
        assert_eq!(dfs_search_node(graph, 1, 2).is_none(), true);
    }

    #[test]
    fn test_discover_element_to_self() {
        let graph = Graph::new();
        let result = dfs_search_node(graph, 1, 1).unwrap();
        assert_eq!(result.links[0], Link::new((1, 1), 0));
        assert_eq!(result.cost, 0);
    }

    #[test]
    fn my_test_discover_two_elements() {
        let mut graph = Graph::new();
        let mut node1 = Node::new("Node 1");
        let mut node2 = Node::new("Node 2");
        node1.id = graph.add_node(node1);
        node2.id = graph.add_node(node2);
        let link1 = Link::new((node1.id, node2.id), 5);
        graph.add_link(link1);
        let result = dfs_search_node(graph, node1.id, node2.id).unwrap();
        let link0 = Link::new((node1.id, node1.id), 0);
        assert_eq!(result.links[0], link0);
        assert_eq!(result.links[1], link1);
        assert_eq!(result.cost, 5);
    }

    #[test]
    fn test_discover_multiple_elements() {
        let mut graph = Graph::new();
        let mut node1 = Node::new("Node 1");
        let mut node2 = Node::new("Node 2");
        let mut node3 = Node::new("Node 3");
        let mut node4 = Node::new("Node 4");
        let mut node5 = Node::new("Node 5");
        let mut node6 = Node::new("Node 6");
        let mut node7 = Node::new("Node 7");
        node1.id = graph.add_node(node1);
        node2.id = graph.add_node(node2);
        node3.id = graph.add_node(node3);
        node4.id = graph.add_node(node4);
        node5.id = graph.add_node(node5);
        node6.id = graph.add_node(node6);
        node7.id = graph.add_node(node7);
        let link1 = Link::new((node1.id, node3.id), 1);
        let link2 = Link::new((node1.id, node2.id), 1);
        let link3 = Link::new((node2.id, node4.id), 2);
        let link4 = Link::new((node3.id, node5.id), 1);
        let link5 = Link::new((node3.id, node6.id), 1);
        let link6 = Link::new((node4.id, node7.id), 1);
        graph.add_link(link1);
        graph.add_link(link2);
        graph.add_link(link3);
        graph.add_link(link4);
        graph.add_link(link5);
        graph.add_link(link6);
        let result = dfs_search_node(graph, node1.id, node7.id).unwrap();
        assert_eq!(result.cost, 4);
        assert_eq!(result.links[1], Link::new((node1.id, node2.id), 1));
        assert_eq!(result.links[2], Link::new((node2.id, node4.id), 2));
        assert_eq!(result.links[3], Link::new((node4.id, node7.id), 1));
    }

    #[test]
    fn discover_elements_with_loose_end_links() {
        let mut graph = Graph::new();
        let mut node1 = Node::new("Node 1");
        let mut node2 = Node::new("Node 2");
        node1.id = graph.add_node(node1);
        node2.id = graph.add_node(node2);
        let link1 = Link::new((node1.id, node2.id), 1);
        let link2 = Link::new((1, 65999), 1);
        graph.add_link(link1);
        graph.add_link(link2);
        let result = dfs_search_node(graph, node1.id, node2.id).unwrap();
        assert_eq!(result.cost, 1);
        assert_eq!(result.links[1], Link::new((node1.id, node2.id), 1));
    }

    #[test]
    fn discover_elements_with_objects_linked_to_themselves() {
        let mut graph = Graph::new();
        let mut node1 = Node::new("Node 1");
        let mut node2 = Node::new("Node 2");
        node1.id = graph.add_node(node1);
        node2.id = graph.add_node(node2);
        let link1 = Link::new((node1.id, node2.id), 5);
        let link2 = Link::new((node1.id, node1.id), 5);
        graph.add_link(link1);
        graph.add_link(link2);
        let result = dfs_search_node(graph, node1.id, node2.id).unwrap();
        assert_eq!(result.links[1], link1);
        assert_eq!(result.cost, 5);
    }

    #[test]
    fn discover_elements_with_circular_graphs() {
        let mut graph = Graph::new();
        let mut node1 = Node::new("Node 1");
        let mut node2 = Node::new("Node 2");
        let mut node3 = Node::new("Node 3");
        let mut node4 = Node::new("Node 4");
        let mut node5 = Node::new("Node 5");
        let mut node6 = Node::new("Node 6");
        let mut node7 = Node::new("Node 7");
        node1.id = graph.add_node(node1);
        node2.id = graph.add_node(node2);
        node3.id = graph.add_node(node3);
        node4.id = graph.add_node(node4);
        node5.id = graph.add_node(node5);
        node6.id = graph.add_node(node6);
        node7.id = graph.add_node(node7);
        let link1 = Link::new((node1.id, node3.id), 1);
        let link2 = Link::new((node1.id, node2.id), 1);
        let link3 = Link::new((node2.id, node4.id), 2);
        let link4 = Link::new((node3.id, node5.id), 1);
        let link5 = Link::new((node5.id, node1.id), 1);
        let link6 = Link::new((node4.id, node7.id), 1);
        graph.add_link(link1);
        graph.add_link(link2);
        graph.add_link(link3);
        graph.add_link(link4);
        graph.add_link(link5);
        graph.add_link(link6);
        let result = dfs_search_node(graph, node1.id, node7.id).unwrap();
        assert_eq!(result.cost, 4);
        assert_eq!(result.links[1], Link::new((node1.id, node2.id), 1));
        assert_eq!(result.links[2], Link::new((node2.id, node4.id), 2));
        assert_eq!(result.links[3], Link::new((node4.id, node7.id), 1));
    }
}
