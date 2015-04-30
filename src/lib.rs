extern crate crc;

use crc::crc32;
use std::cmp::Ordering;

struct Node {
    id : String,
    hash_key: u32,
}

impl Node {

fn new(id: String) -> Node {
    Node { id: id, hash_key: crc32::checksum_ieee(id.as_bytes()) }
}

}

impl Eq for Node {}

impl PartialEq for Node {
    fn eq(&self, other: &Node) -> bool {
      self.id.eq(&other.id)
    }
}

impl Ord for Node {

    fn cmp(&self, other: &Node) -> Ordering {
      self.id.cmp(&other.id)
    }

}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Node) -> Option<Ordering> {
      self.id.partial_cmp(&other.id)
    }
}


type Nodes = Vec<Node>;

struct Ring {
    nodes: Nodes
}

impl Ring {

//fn new_ring () -> &Ring {
fn new () -> Ring {
   // Ring { nodes : Vec<Node>::new() }
    Ring { nodes : Nodes::new() }

}


fn add_node(&mut self, id : String) {
      let node = Node::new(id);
      self.nodes.push(node);
      self.nodes.sort(); 
}

}

#[test]
fn it_works() {
}
