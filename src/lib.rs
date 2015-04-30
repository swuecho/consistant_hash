extern crate crc;

use crc::crc32;
use std::cmp::Ordering;

struct Node {
    id : &'static str,
    hash_key: u32,
}

impl Node {

fn new(id: &'static str ) -> Node {
    Node { id: id, hash_key: crc32::checksum_ieee(id.as_bytes()) }
}

}

impl Eq for Node {}

impl PartialEq for Node {
    fn eq(&self, other: &Node) -> bool {
        // why eq is not right
      self.id == other.id
    }
}

impl Ord for Node {

    fn cmp(&self, other: &Node) -> Ordering {
      self.id.cmp(other.id)
    }

}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Node) -> Option<Ordering> {
        // precedence
      self.id.partial_cmp(other.id)
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


fn add_node(&mut self, id : &'static str) {
      let node = Node::new(id);
      self.nodes.push(node);
      self.nodes.sort(); 
}

fn get(&self, key: &'static str) -> &str {
     let hash = crc32::checksum_ieee(key.as_bytes());
     //
     let result = self.nodes.binary_search_by( |node | node.hash_key.cmp(&hash) );

     let mut index = match result {
        Ok(i)  => i,
        Err(j) => j,
     };

     if index == self.nodes.len() {
        index = 0;
     }

     self.nodes[index].id
}

}

#[test]
fn it_works() {
}
