extern crate crc;

use crc::crc32;
use std::cmp::Ordering;
// *********************************************//
// Node
// *********************************************//
struct Node {
    id : &'static str,
    hash_key: u32,
}

impl Node {

fn new(id: &'static str ) -> Node {
    Node { id: id, hash_key: hash_key(id) }
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

// ***************** Nodes ************************//

type Nodes = Vec<Node>;

// ***************** Ring ***********************//
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
      // todo , use search
}

fn search (&self, id: &str) -> usize {
     let hash = hash_key(id);
     //
     let result = self.nodes.binary_search_by( |node | node.hash_key.cmp(&id) );

     let index = match result {
        Ok(i)  => i,
        Err(j) => j,
     };

     index

}

fn get(&self, id: &'static str) -> &str {
     let mut index = self.search(id);

     if index == self.nodes.len() {
        index = 0;
     }

     self.nodes[index].id
}


fn remove ( &self, id: &str) -> Option<bool> {
     let mut index = self.search(id);
     self.remove(index);
}



}

fn hash_key (id: &str) -> u32 {
     crc32::checksum_ieee(id.as_bytes()) 
}

#[test]
fn it_works() {
}
