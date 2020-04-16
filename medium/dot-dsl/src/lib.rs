pub mod graph {
  use graph_items::edge::Edge;
  use graph_items::node::Node;

  pub struct Graph<'a> {
    pub nodes: Vec<Node<'a>>,
    pub edges: Vec<Edge<'a>>,
    pub attrs: Vec<String>,
  }

  impl<'a> Graph<'a> {
    pub fn new() -> Self {
      Self {
        nodes: Vec::new(),
        edges: Vec::new(),
        attrs: Vec::new(),
      }
    }

    pub fn with_nodes(mut self, nodes: &'a Vec<Node>) -> Self {
      self.nodes = nodes.to_vec();
      self
    }

    pub fn with_edges(mut self, edges: &'a Vec<Edge>) -> Self {
      self.edges = edges.to_vec();
      self
    }
  }

  pub mod graph_items {
    pub mod edge {
      #[derive(Debug, PartialEq, Clone)]
      pub struct Edge<'a> {
        from: &'a str,
        to: &'a str,
        attrs: Vec<(&'a str, &'a str)>,
      }

      impl<'a> Edge<'a> {
        pub fn new(from: &'a str, to: &'a str) -> Self {
          Self {
            from,
            to,
            attrs: Vec::new(),
          }
        }
      }
    }

    pub mod node {
      #[derive(Debug, PartialEq, Clone)]
      pub struct Node<'a> {
        name: &'a str,
        attrs: Vec<(&'a str, &'a str)>,
      }

      impl<'a> Node<'a> {
        pub fn new(name: &'a str) -> Self {
          Self {
            name: name,
            attrs: Vec::new(),
          }
        }

        pub fn with_attrs(mut self, attrs: &'a [(&str, &str)]) -> Self {
          self.attrs = attrs.to_vec();
          self
        }
      }
    }
  }
}
