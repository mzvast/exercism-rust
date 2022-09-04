pub mod graph {

    pub mod graph_items {

        pub mod edge {
            use std::collections::HashMap;

            #[derive(Clone, Debug, PartialEq)]
            pub struct Edge {
                pub nodes: (String, String),
                pub attrs: HashMap<String, String>,
            }

            impl Edge {
                pub fn new(a: &str, b: &str) -> Self {
                    Edge {
                        nodes: (a.to_string(), b.to_string()),
                        attrs: HashMap::new(),
                    }
                }

                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    for (k, v) in attrs {
                        self.attrs.entry(k.to_string()).or_insert(v.to_string());
                    }
                    self
                }
            }
        }
        pub mod node {
            use std::{collections::HashMap, hash::Hash};

            #[derive(Clone, Debug, PartialEq)]
            pub struct Node {
                pub name: String,
                pub attrs: HashMap<String, String>,
            }

            impl Node {
                pub fn new(name: &str) -> Self {
                    Node {
                        name: name.to_owned(),
                        attrs: HashMap::new(),
                    }
                }

                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    for (k, v) in attrs {
                        self.attrs.entry(k.to_string()).or_insert(v.to_string());
                    }
                    self
                }

                pub fn get_attr(&self, k: &str) -> Option<&str> {
                    self.attrs.get(k).map(|s| s.as_str())
                }
            }
        }
        pub mod attr {
            #[derive(Clone, Debug, PartialEq)]
            pub struct Attr(Vec<(String, String)>);
        }
    }

    use std::collections::HashMap;

    use graph_items::edge::Edge;
    use graph_items::node::Node;

    // #[derive(Debug)]
    pub struct Graph {
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        pub attrs: HashMap<String, String>,
    }

    impl Graph {
        pub fn new() -> Self {
            Graph {
                nodes: Vec::new(),
                edges: Vec::new(),
                attrs: HashMap::new(),
            }
        }

        pub fn with_nodes(mut self, nodes: &Vec<Node>) -> Self {
            self.nodes = self
                .nodes
                .into_iter()
                .chain(nodes.iter().cloned())
                .collect();
            self
        }

        pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
            for (k, v) in attrs {
                self.attrs.entry(k.to_string()).or_insert(v.to_string());
            }
            self
        }

        pub fn with_edges(mut self, edges: &Vec<Edge>) -> Self {
            self.edges = self
                .edges
                .into_iter()
                .chain(edges.iter().cloned())
                .collect();
            self
        }

        pub fn get_node(self, name: &str) -> Option<Node> {
            self.nodes.iter().find(|&node| node.name == name).cloned()
        }
    }
}
