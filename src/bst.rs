/// define binary tree node

#[derive(Debug)]
pub struct Node<T> {
    val: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

trait BinaryTree<T> {
    fn pre_order(&self, vec: &mut Vec<T>);
    fn in_order(&self, vec: &mut Vec<T>);
    fn post_order(&self, vec: &mut Vec<T>);
}

trait BinarySearchTree<T>: BinaryTree<T>
where
    T: PartialOrd,
{
    fn insert(&mut self, val: T);
}

impl<T> Node<T> {
    pub fn new(val: T) -> Self {
        Node {
            left: None,
            right: None,
            val: val,
        }
    }
}

impl<T> BinarySearchTree<T> for Node<T>
where
    T: PartialOrd + Copy,
{
    fn insert(&mut self, val: T) {
        if self.val > val {
            if let Some(ref mut left) = self.left {
                left.insert(val);
            } else {
                self.left = Some(Box::new(Node::new(val)));
            }
        } else {
            if let Some(ref mut right) = self.right {
                right.insert(val);
            } else {
                self.right = Some(Box::new(Node::new(val)));
            }
        }
    }
}

impl<T> BinaryTree<T> for Node<T>
where
    T: Copy,
{
    fn pre_order(&self, vec: &mut Vec<T>) {
        vec.push(self.val);
        if let Some(ref left) = self.left {
            left.pre_order(vec);
        }
        if let Some(ref right) = self.right {
            right.pre_order(vec);
        }
    }
    fn in_order(&self, vec: &mut Vec<T>) {
        if let Some(ref left) = self.left {
            left.in_order(vec);
        }
        vec.push(self.val);
        if let Some(ref right) = self.right {
            right.in_order(vec);
        }
    }
    fn post_order(&self, vec: &mut Vec<T>) {
        if let Some(ref left) = self.left {
            left.post_order(vec);
        }
        if let Some(ref right) = self.right {
            right.post_order(vec);
        }
        vec.push(self.val);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        let mut node = Node::<i32>::new(10);
        node.insert(5);
        node.insert(8);
        node.insert(15);
        node.insert(7);
        node.insert(3);
        node.insert(13);
        node.insert(20);
        let mut vec_pre = vec![];
        node.pre_order(&mut vec_pre);
        assert_eq!(vec_pre, vec![10, 5, 3, 8, 7, 15, 13, 20]);
        let mut vec_in = vec![];
        node.in_order(&mut vec_in);
        assert_eq!(vec_in, vec![3, 5, 7, 8, 10, 13, 15, 20]);
        let mut vec_post = vec![];
        node.post_order(&mut vec_post);
        assert_eq!(vec_post, vec![3, 7, 8, 5, 13, 20, 15, 10]);
    }
}
