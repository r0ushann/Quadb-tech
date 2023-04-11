#[derive(Debug)]
struct Node {
    value: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(value: i32, left: Option<Box<Node>>, right: Option<Box<Node>>) -> Node {
        Node { value, left, right }
    }
}

fn max_depth(node: &Option<Box<Node>>) -> i32 {
    match node {
        None => 0,
        Some(node) => {
            let left_depth = max_depth(&node.left);
            let right_depth = max_depth(&node.right);

            1 + left_depth.max(right_depth)
        }
    }
}

fn main() {
                          

    let root = Node::new(
        5,
        Some(Box::new(Node::new(3, Some(Box::new(Node::new(1, None, None))), Some(Box::new(Node::new(4, None, None)))))),
        Some(Box::new(Node::new(8, None, Some(Box::new(Node::new(10, None, None)))))),
    );

    let max_depth = max_depth(&Some(Box::new(root)));

    println!("The maximum depth of the binary tree is: {}", max_depth);


}


 //                                 5
    //                             / \
    //                            3   8
    //                           / \   \
    //                          1   4   10