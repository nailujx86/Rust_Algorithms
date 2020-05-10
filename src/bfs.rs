use crate::graph::*;

/// A function to search for the path to a node using the [Breadth-first search](https://en.wikipedia.org/wiki/Breadth-first_search) method.
/// 
/// This function takes two node ids, for the start and target node, and computes a path between them.
/// This path consists of a Vec of Links.
/// The first link is always from the first element to itself.
/// # Example:
/// ```rust
/// use rust_algorithms::graph::*;
/// use rust_algorithms::bfs::*;
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
/// let result = bfs_search_node(graph, node1.id, node2.id).unwrap();
/// let link0 = Link::new((node1.id, node1.id), 0);
/// 
/// assert_eq!(result.links[0], link0);
/// assert_eq!(result.links[1], link1);
/// assert_eq!(result.cost, 5);
/// ```

pub fn bfs_search_node(
    mut graph: Graph,
    start_node_id: isize,
    search_node_id: isize,
) -> Option<SearchResult> {
    use std::collections::VecDeque;

    if start_node_id == search_node_id {
        return Some(
            SearchResult::new()
                .cost(0)
                .links(vec![Link::new((start_node_id, search_node_id), 0)]),
        );
    }

    // the first int is the nodes id, the second the depth in the graph, the third the link that lead to this node
    let mut queue = VecDeque::<(isize, Vec<Link>)>::new();

    // the first link leads from the first element to itself with no cost. It is just there to provide any link
    let mut vector = Vec::new();
    vector.push(Link::new((start_node_id, start_node_id), 0));
    queue.push_front((start_node_id, vector));

    // retrieve the start node from the graph and mark it as visited.
    // if it does not exist, there cannot be a path, return None.
    let mut start_node = match graph.get_node(start_node_id) {
        Some(node) => node,
        None => {
            return None;
        }
    };
    start_node.is_discovered = true;

    // iterate through the queue
    while !queue.is_empty() {
        // The while loop guarantees that there is something to pop, so unwrapping is safe
        let current_queue_element = queue.pop_front().unwrap();
        let current_node = current_queue_element.0;

        // abort case: node searched for is found. Add up link cost and return result.
        if current_node == search_node_id {
            let mut cost = 0;
            for link in current_queue_element.1.iter() {
                cost += link.cost;
            }
            return Some(
                SearchResult::new()
                    .cost(cost)
                    .links(current_queue_element.1),
            );
        }
        // this node was not the one searched for. 
        else {

            // find all links from this node
            let mygraph = graph.clone();
            let links = mygraph.find_links_from_node(current_node);
            for link in links {
                //ignore circular links (from object to itself)
                if link.members.0 != link.members.1 {
                    // get the node_id of the node on the other end of the link
                    let found_node: isize = if link.members.0 == current_node {
                        link.members.1
                    } else {
                        link.members.0
                    };

                    // if the node can be found inside the graph
                    if let Some(node) = graph.get_node(found_node) {
                        // and it has not been discovered yet
                        if !node.is_discovered {
                            // push the link to it to a new linklist
                            let mut new_vector = current_queue_element.1.clone();
                            new_vector.push(*link);
                            // and add that and the node to the queue
                            queue.push_back((found_node, new_vector));
                            // mark the node as visited, as it will be processed
                            node.is_discovered = true;
                        }
                    }
                }
            }
        }
    }

    // if the queue is empty and no element was found, return None.
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
        assert_eq!(bfs_search_node(graph, 2, 1).is_none(), true);
    }

    #[test]
    fn test_discover_no_target_element() {
        let mut graph = Graph::new();
        let node1 = Node::new("Node 1");
        graph.add_node(node1);
        assert_eq!(bfs_search_node(graph, 1, 2).is_none(), true);
    }

    #[test]
    fn test_discover_no_start_and_target_element() {
        let graph = Graph::new();
        assert_eq!(bfs_search_node(graph, 1, 2).is_none(), true);
    }

    #[test]
    fn test_discover_no_link() {
        let mut graph = Graph::new();
        let node1 = Node::new("Node 1");
        let node2 = Node::new("Node 1");
        graph.add_node(node1);
        graph.add_node(node2);
        graph.add_link(Link::new((1, 3), 1));
        assert_eq!(bfs_search_node(graph, 1, 2).is_none(), true);
    }

    #[test]
    fn test_discover_element_to_self() {
        let graph = Graph::new();
        let result = bfs_search_node(graph, 1, 1).unwrap();
        assert_eq!(result.links[0], Link::new((1, 1), 0));
        assert_eq!(result.cost, 0);
    }

    #[test]
    fn test_discover_two_elements() {
        let mut graph = Graph::new();
        let mut node1 = Node::new("Node 1");
        let mut node2 = Node::new("Node 2");
        node1.id = graph.add_node(node1);
        node2.id = graph.add_node(node2);
        let link1 = Link::new((node1.id, node2.id), 5);
        graph.add_link(link1);
        let result = bfs_search_node(graph, node1.id, node2.id).unwrap();
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
        let result = bfs_search_node(graph, node1.id, node7.id).unwrap();
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
        let result = bfs_search_node(graph, node1.id, node2.id).unwrap();
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
        let result = bfs_search_node(graph, node1.id, node2.id).unwrap();
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
        let result = bfs_search_node(graph, node1.id, node7.id).unwrap();
        assert_eq!(result.cost, 4);
        assert_eq!(result.links[1], Link::new((node1.id, node2.id), 1));
        assert_eq!(result.links[2], Link::new((node2.id, node4.id), 2));
        assert_eq!(result.links[3], Link::new((node4.id, node7.id), 1));
    }
}
