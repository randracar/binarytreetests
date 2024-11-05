struct Node<T> {
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T: Ord + std::fmt::Debug> Node<T> {
    fn count_nodes_and_leaves(&self,nodes: &mut i32, leaves: &mut i32) {
        *nodes += 1;
        match (&self.left, &self.right) {
            (Some(left), Some(right)) => {
                // Recursively count in both subtrees
                left.count_nodes_and_leaves(nodes, leaves);
                right.count_nodes_and_leaves(nodes, leaves);
            }
            (Some(left), None) => {
                // Left child only
                left.count_nodes_and_leaves(nodes, leaves);
            }
            (None, Some(right)) => {
                // Right child only
                right.count_nodes_and_leaves(nodes, leaves);
            }
            (None, None) => {
                // Leaf node (no children)
                *leaves += 1;
            }
        }
    }

    fn insert(&mut self, value: T) {
        if value < self.value {
            // insere na subarvore da esquerda
            match self.left {
                Some(ref mut left_node) => {
                    left_node.insert(value);
                }
                None => {
                    self.left = Some(Box::new(Node::new(value)));
                }
            }
        } else if value > self.value {
            //insere na subarvore da direita
            match self.right {
                Some(ref mut right_node) => {
                    right_node.insert(value);
                }
                None => {
                    self.right = Some(Box::new(Node::new(value)));
                }
            }
        }
    }

    fn new(value: T) -> Self {
        Node {
            value,
            left: None,
            right: None,
        }
    }

    fn print_node(&self, level: usize) {
        let indent = "    ".repeat(level);
        println!("{}{:?}", indent, self.value);

        if let Some(ref left) = self.left {
            left.print_node(level + 1);
        }

        if let Some(ref right) = self.right {
            right.print_node(level + 1);
        }
    }
    
    fn in_order(&self) {
        if let Some(ref left) = self.left {
            left.in_order();
        }
        println!("{:?}", self.value);
        if let Some(ref right) = self.right {
            right.in_order();
        }
    } 

    fn pre_order(&self) {
        println!("{:?}", self.value);
        if let Some(ref left) = self.left {
            left.pre_order();
        } 
        if let Some(ref right) = self.right {
            right.pre_order();
        }
    }

    fn post_order(&self) {
        if let Some(ref left) = self.left {
            left.post_order();
        }
        if let Some(ref right) = self.right {
            right.post_order();
        }
        println!("{:?}", self.value);
    }


    fn min_value(&self) -> &T {
        match self.left {
            Some(ref left_node) => left_node.min_value(),
            None => &self.value,
        }
    }

    fn max_value(&self) -> &T {
        match self.right {
            Some(ref right_node) => right_node.max_value(),
            None => &self.value,
        }
    }
}

struct BinaryTree<T> {
    root: Option<Box<Node<T>>>,
    nodes: i32,
    leaves: i32,
}

impl<T: Ord + std::fmt::Debug> BinaryTree<T> {
    fn new() -> Self {
        BinaryTree { root: None, nodes: 0, leaves: 0}
    }

    fn count_nodes_and_leaves(&mut self) {
        if let Some(ref mut root) = self.root {
            root.count_nodes_and_leaves(&mut self.nodes, &mut self.leaves);
            println!("Number of nodes: {}", self.nodes);
            println!("Number of leaves: {}", self.leaves);
        }
    }
    
    fn insert(&mut self, value: T) {
        match self.root {
            Some(ref mut node) => {
                node.insert(value);
            }
            None => {
                self.root = Some(Box::new(Node::new(value))); 
            }
        }
    }

    fn print_tree(&self) {
        if let Some(ref root) = self.root {
            root.print_node(0);
        } else {
            println!("Tem nada na arvore");
        }
    }

    fn in_order(&self) {
        if let Some(ref root) = self.root {
            root.in_order();
        }
    }

    fn pre_order(&self) {
        if let Some(ref root) = self.root {
            root.pre_order();
        }
    }

    fn post_order(&self) {
        if let Some(ref root) = self.root {
            root.post_order();
        }
    }

    fn min_value(&self) -> Option<&T> {
        self.root.as_ref().map(|node| node.min_value())
    }

    fn max_value(&self) -> Option<&T> {
        self.root.as_ref().map(|node| node.max_value())
    }
}

fn main() {
    let mut tree = BinaryTree::new();
    tree.insert(10);
    tree.insert(5);
    tree.insert(15);
    tree.insert(2);
    tree.insert(7);
    tree.insert(12);
    tree.insert(20);
    tree.insert(4);
    tree.insert(6);
    tree.insert(9);
    tree.insert(1);

    println!("arvore binaria tem o root: {}", tree.root.as_ref().unwrap().value);
    println!("estrutura da arvore: ");
    tree.print_tree();

    println!("In-Order Traversal:");
    tree.in_order();

    println!("\nPre-Order Traversal:");
    tree.pre_order();

    println!("\nPost-Order Traversal:");
    tree.post_order();

    if let Some(min) = tree.min_value() {
        println!("Min val of tree: {}", min);
    } else {
        println!("tree empty lol");
    }

    if let Some(max) = tree.max_value() {
        println!("Max val of tree: {}", max);
    } else {
        println!("tree empty lol");
    }

    tree.count_nodes_and_leaves();
} 