
pub mod graph {
    use graph_items::{edge::Edge, node::Node};


    pub mod graph_items {
        pub mod node {
            #[derive(Clone, Debug, PartialEq, Default)]
            pub struct Node {
                pub val: String,
                pub attrs: std::collections::HashMap<String, String>,
            }

            impl Node {
                pub fn new(val: &str) -> Self {
                    Self {
                        val: val.to_string(),
                        ..Default::default()
                    }
                }
            
                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    attrs.iter().for_each(|(i, j)| {
                        self.attrs.insert(i.to_string(), j.to_string());
                    });
                    self
                }

                pub fn attr(&self, val: &str) -> Option<&str> {
                    self.attrs.get(val).map(|s| s.as_str())
                }
            }
        }

        pub mod edge {
            #[derive(Clone, Debug, PartialEq, Default)]
            pub struct Edge {
                pub vals: (String, String),
                pub attrs: std::collections::HashMap<String, String>
            }

            impl Edge {
                pub fn new(val_1: &str, val_2: &str) -> Self {
                    Self {
                        vals: (val_1.to_string(), val_2.to_string()),
                        ..Default::default()
                    }
                }

                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    attrs.iter().for_each(|(i, j)| {
                        self.attrs.insert(i.to_string(), j.to_string());
                    });
                    self
                }

                pub fn attr(&self, val: &str) -> Option<&str> {
                    self.attrs.get(val).map(|s| &**s)
                }
            }
        }
    }

    #[derive(Default)]
    pub struct Graph {
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        pub attrs: std::collections::HashMap<String, String>
    }

    impl Graph {
        pub fn new() -> Self {
            Self::default()
        }

        pub fn with_nodes(mut self, nodes: &Vec<Node>) -> Self {
            nodes.iter().for_each(|x| {
                self.nodes.push(x.clone())
            });
            self
        }

        pub fn with_edges(mut self, edges: &Vec<Edge>) -> Self {
            edges.iter().for_each(|x| {
                self.edges.push(x.clone())
            });
            self
        }

        pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
            attrs.iter().for_each(|(i, j)| {
                self.attrs.insert(i.to_string(), j.to_string());
            });
            self
        }

        pub fn node(&self, val: &str) -> Option<&Node> {
            self.nodes.iter().find(|x| {
                x.val == val
            })
        }
    }
}
