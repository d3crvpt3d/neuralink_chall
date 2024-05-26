pub mod huffman{

  pub fn get_huffman_tree(data: &Vec<u8>) -> Node{
    Node::new(None, None, 0)
	}

	struct Node{
		left	:	Option<Box<Node>>,
		right	:	Option<Box<Node>>,
		cost	:	u32,
	}

  impl Node {
    fn new(left: Option<Box<Node>>, right: Option<Box<Node>>, cost: u32) -> Node {
      Node{
        left,
        right,
        cost
      }
    }
}

}