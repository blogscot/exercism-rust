pub mod graph {

  use self::graph_items::edge::Edge;
  use self::graph_items::node::Node;
  use std::collections::HashMap;

  pub mod graph_items {

    pub mod edge {

      #[derive(Clone, Debug, PartialEq)]
      pub struct Edge {
        start: String,
        end: String,
        attrs: Vec<(String, String)>,
      }

      impl Edge {
        pub fn new(start: &str, end: &str) -> Self {
          let start = start.to_string();
          let end = end.to_string();
          let attrs = Vec::new();
          Edge { start, end, attrs }
        }
        pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
          self.attrs = attrs
            .iter()
            .map(|attr| (attr.0.to_string(), attr.1.to_string()))
            .collect();
          self
        }
      }
    }

    pub mod node {

      #[derive(Clone, Debug, PartialEq)]
      pub struct Node {
        label: String,
        attrs: Vec<(String, String)>,
      }

      impl Node {
        pub fn new(label: &str) -> Self {
          let label = label.to_string();
          let attrs = Vec::new();
          Node { label, attrs }
        }
        pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
          self.attrs = attrs
            .iter()
            .map(|attr| (attr.0.to_string(), attr.1.to_string()))
            .collect();
          self
        }
      }
    }
  }

  #[derive(Default)]
  pub struct Graph {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
    pub attrs: HashMap<String, String>,
  }

  impl Graph {
    pub fn new() -> Self {
      Graph::default()
    }
    pub fn with_nodes(mut self, nodes: &[Node]) -> Self {
      self.nodes = nodes.to_vec();
      self
    }
    pub fn with_edges(mut self, edges: &[Edge]) -> Self {
      self.edges = edges.to_vec();
      self
    }
    pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
      self.attrs = attrs.iter().fold(HashMap::new(), |mut acc, attr| {
        acc.insert((attr.0).to_string(), (attr.1).to_string());
        acc
      });
      self
    }
  }
}
