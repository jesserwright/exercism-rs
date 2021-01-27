pub mod graph {
    use graph_items::{edge::Edge, node::Node};
    use std::collections::HashMap;
    #[derive(Default)]
    pub struct Graph {
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        pub attrs: HashMap<String, String>,
    }

    impl Graph {
        pub fn new() -> Self {
            Graph {
                nodes: vec![],
                edges: vec![],
                attrs: HashMap::new(),
            }
        }

        pub fn get_node(self, node_name: &str) -> Result<Node, &'static str> {
            match self.nodes.iter().find(|n| n.name == node_name) {
                Some(n) => Ok(n.clone()),
                None => Err(""),
            }
        }

        pub fn with_nodes(mut self, nodes: &[Node]) -> Self {
            self.nodes = Vec::from(nodes);
            self
        }
        pub fn with_edges(mut self, edges: &[Edge]) -> Self {
            self.edges = Vec::from(edges);
            self
        }
        pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
            for attr in attrs {
                let (a, b) = *attr;
                self.attrs.insert(String::from(a), String::from(b));
            }
            self
        }
    }
    pub mod graph_items {
        pub mod edge {
            #[derive(Clone, Debug, PartialEq)]
            pub struct Edge {
                from: String,
                to: String,
                attrs: Vec<(String, String)>,
            }
            impl Edge {
                pub fn new(from: &str, to: &str) -> Self {
                    Edge {
                        from: from.to_string(),
                        to: to.to_string(),
                        attrs: vec![],
                    }
                }
                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    for attr in attrs {
                        let (a, b) = *attr;
                        self.attrs.push((String::from(a), String::from(b)));
                    }
                    self
                }
            }
        }
        pub mod node {

            #[derive(PartialEq, Debug, Clone)]
            pub struct Node {
                pub name: String,
                attrs: Vec<(String, String)>,
            }
            impl Node {
                pub fn new(name: &str) -> Self {
                    Node {
                        name: String::from(name),
                        attrs: vec![],
                    }
                }
                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    for attr in attrs {
                        let (a, b) = *attr;
                        self.attrs.push((String::from(a), String::from(b)));
                    }
                    self
                }
                pub fn get_attr(&self, attr_name: &str) -> Option<&str> {
                    match self.attrs.iter().find(|(key, _)| *key == attr_name) {
                        Some((_, val)) => Some(val),
                        None => None,
                    }
                }
            }
        }
    }
}
