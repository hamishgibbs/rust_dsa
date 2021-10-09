/*

How it works:

*/

// HashMap stores a Hash Table of keys (hashes) & values
// HashSet stores a set of unique hashes
use std::collections::{HashMap, HashSet};
use std::fmt;

// Debug creates an automatic interface for printing any type
// Clone explicitly duplicates an object (instead of Copy which implicitly duplicates)
// i.e. Clone creates an entirely new memory location & pointer and copies data to that location
// While Copy (a bitwise copy) copies only a memory address
#[derive(Debug, Clone)]
pub struct NodeNotInGraph;

// Define a display mechanism for the NodeNotInGraph object to allow output of error message
impl fmt::Display for NodeNotInGraph {
    fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "accessing node that is not in the graph!")
    }
}

/*
Define object structure for a directed graph:

A hash map {hash: value} where value is a vector of adjacent node ID and weight

A directed graph is (slightly) simpler to implement because an edge between two nodes in only one edge

i.e.

{a: [(b, 1), (c, 2)]} identifies a 3 vertex graph with  V in [a, b, c]

*/
pub struct DirectedGraph {
    adjacency_table: HashMap<String, Vec<(String, i32)>>,
}

impl Graph for DirectedGraph {
    // Define new() method & signify that new() returns a DirectedGraph object
    fn new() -> DirectedGraph {

        // Create a DirectedGraph object by initialising a new HashMap (see defined type above)
        // i.e. DirectedGraph is implemented as a HashMap with a defined structure
        DirectedGraph {
            adjacency_table: HashMap::new(),
        }
    }

    // Define a method to retrive mutable adjacency table
    // Does this allow another method to update the adjacency_table of this graph object?
    // Yes
    fn adjacency_table_mutable(&mut self) -> &mut HashMap<String, Vec<(String, i32)>> {
        &mut self.adjacency_table
    }

    // Define a method to retrive an immutable adjacency table
    fn adjacency_table(&self) -> &HashMap<String, Vec<(String, i32)>> {
        &self.adjacency_table
    }

}


// Define structure of undirected graph
pub struct UndirectedGraph {
    adjacency_table: HashMap<String, Vec<(String, i32)>>
}

// Means that we are implementing the Graph interface for the UndirectedGraph type
impl Graph for UndirectedGraph {
    fn new() -> UndirectedGraph {
        UndirectedGraph {
            adjacency_table: HashMap::new(),
        }
    }

    fn adjacency_table_mutable(&mut self) -> &mut HashMap<String, Vec<(String, i32)>> {
        &mut self.adjacency_table
    }

    fn adjacency_table(&self) -> &HashMap<String, Vec<(String, i32)>> {
        &self.adjacency_table
    }

    // Does this add_edge method overlay the Graph add_edge method (below)?
    // Yes
    fn add_edge(&mut self, edge: (&str, &str, i32)) {
        self.add_node(edge.0);
        self.add_node(edge.1);

        // Insert both nodes in the adjacency table as connected to one another with weight edge.2
        // Note that because the graph is undirected, if a -> b, b -> a.
        // Whereas a directed graph can have, a -> b without b -> a
        self.adjacency_table
            .entry(edge.0.to_string())
            .and_modify(|e| {
                e.push((edge.1.to_string(), edge.2));
            });
        self.adjacency_table
            .entry(edge.1.to_string())
            .and_modify(|e| {
                e.push((edge.0.to_string(), edge.2));
            });
    }
}


// Implementation of methods that are shared between directed & undirected graphs
/*

`impl` is an implementation of a type where methods & constants are defined for that type

A type:

struct Example {
    whatever...
}

An implementation of the type Example (defines the `new` method on Example):

impl Example {
    fn new() {
        whatever...
    }
}

*/

/*

`trait` defines methods that are shared between implementations of a type (i.e. a shared interface)

Here, Graph defines the shared functionality between DirectedGraph and UndirectedGraph

*/
pub trait Graph {
    // define "slots" for implementations of directed or undirected graph
    fn new() -> Self;
    fn adjacency_table_mutable(&mut self) -> &mut HashMap<String, Vec<(String, i32)>>;
    fn adjacency_table(&self) -> &HashMap<String, Vec<(String, i32)>>;

    fn add_node(&mut self, node: &str) -> bool {
        // get node from adjacency table - .get() is defined for HashMap
        // Match return for get() call - if not in graph, insert & return true, else return false
        match self.adjacency_table().get(node) {
            None => {
                self.adjacency_table_mutable()
                    .insert((*node).to_string(), Vec::new());
                true
            }
            _ => false,
        }
    }

    fn add_edge(&mut self, edge: (&str, &str, i32)) {
        self.add_node(edge.0);
        self.add_node(edge.1);

        // adjacency_table_mutable() points to adjacency table of the Graph object so that it can be changed
        self.adjacency_table_mutable()
            .entry(edge.0.to_string())
            .and_modify(|e| {
                e.push((edge.1.to_string(), edge.2));
            });
    }

    // Simply return contents of the adjacency table (HashMap) for the given node
    // Else (if node doesn't exist), raise error
    fn neighbours(&self, node: &str) -> Result<&Vec<(String, i32)>, NodeNotInGraph> {
        match self.adjacency_table().get(node) {
            None => Err(NodeNotInGraph),
            Some(i) => Ok(i),
        }
    }

    // If graph contains a node, values in the adjacency_table must be something
    fn contains(&self, node: &str) -> bool {
        self.adjacency_table().get(node).is_some()
    }

    // Return HashSet of the keys of the adjacency_table
    fn nodes(&self) -> HashSet<&String> {
        self.adjacency_table().keys().collect()
    }

    // extract each vector in the adjacency_table for each key in the adjacency_table (and weight)
    fn edges(&self) -> Vec<(&String, &String, i32)> {
        let mut edges = Vec::new();
        for (from_node, from_node_neighbors) in self.adjacency_table() {
            for (to_node, weight) in from_node_neighbors {
                edges.push((from_node, to_node, *weight));
            }
        }
        edges
    }
}

#[cfg(test)]
mod test_undirected_graph {
    use super::Graph;
    use super::UndirectedGraph;

    #[test]
    fn test_add_edge() {
        let mut graph = UndirectedGraph::new();

        graph.add_edge(("a", "b", 5));
        graph.add_edge(("b", "c", 10));
        graph.add_edge(("c", "a", 7));

        let expected_edges = [
            (&String::from("a"), &String::from("b"), 5),
            (&String::from("b"), &String::from("a"), 5),
            (&String::from("b"), &String::from("c"), 10),
            (&String::from("c"), &String::from("b"), 10),
            (&String::from("c"), &String::from("a"), 7),
            (&String::from("a"), &String::from("c"), 7),
        ];

        for edge in expected_edges.iter() {
            assert_eq!(graph.edges().contains(edge), true);
        }
    }

    #[test]
    fn test_neighbors() {
        let mut graph = UndirectedGraph::new();

        graph.add_edge(("a", "b", 5));
        graph.add_edge(("b", "c", 10));
        graph.add_edge(("c", "a", 7));

        assert_eq!(
            graph.neighbours("a").unwrap(),
            &vec![(String::from("b"), 5), (String::from("c"), 7)]
        )

    }

}


#[cfg(test)]
mod test_directed_graph {
    use super::Graph;
    use super::DirectedGraph;

    #[test]
    fn test_add_node() {
        let mut graph = DirectedGraph::new();
        graph.add_node("a");
        graph.add_node("b");
        graph.add_node("c");
        assert_eq!(
            graph.nodes(),
            [&String::from("a"), &String::from("b"), &String::from("c")]
                .iter()
                .cloned()
                .collect()
        );
    }

    #[test]
    fn test_add_edge() {
        let mut graph = DirectedGraph::new();

        graph.add_edge(("a", "b", 5));
        graph.add_edge(("b", "c", 10));
        graph.add_edge(("c", "a", 7));

        let expected_edges = [
            (&String::from("a"), &String::from("b"), 5),
            (&String::from("b"), &String::from("c"), 10),
            (&String::from("c"), &String::from("a"), 7),
        ];

        for edge in expected_edges.iter() {
            assert_eq!(graph.edges().contains(edge), true);
        }

    }

    #[test]
    fn test_neighbors() {
        let mut graph = DirectedGraph::new();

        graph.add_edge(("a", "b", 5));
        graph.add_edge(("b", "c", 10));
        graph.add_edge(("c", "a", 7));

        assert_eq!(
            graph.neighbours("a").unwrap(),
            &vec![(String::from("b"), 5)]
        );
    }

    #[test]
    fn test_contains() {
        let mut graph = DirectedGraph::new();

        graph.add_node("a");
        graph.add_node("b");
        graph.add_node("c");

        assert_eq!(graph.contains("a"), true);
        assert_eq!(graph.contains("b"), true);
        assert_eq!(graph.contains("c"), true);
        assert_eq!(graph.contains("d"), false);

    }

}
