pub struct Stack<T> {
    top: Option<Box<StackNode<T>>>,
}

struct StackNode<T> {
    val: T,
    next: Option<Box<StackNode<T>>>,
}

impl<T> StackNode<T> {
    fn new(t: T) -> Self {
        StackNode { val: t, next: None }
    }
}

impl<T> Stack<T> {
    /// new stack
    pub fn new() -> Self {
        Stack { top: None }
    }
    /// push elem
    pub fn push(&mut self, t: T) {
        let mut node = StackNode::new(t);
        node.next = self.top.take();
        self.top = Some(Box::new(node));
    }
    /// pop elem
    pub fn pop(&mut self) -> Option<T> {
        let op_bx_node = self.top.take();
        let val = match op_bx_node {
            Some(mut bx_node) => {
                self.top = bx_node.next.take();
                Some(bx_node.val)
            }
            None => None,
        };
        val
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn stack_test() {
        let mut vec = Vec::new();
        let mut stack = Stack::<i32>::new();
        stack.push(123);
        stack.push(456);
        stack.push(789);
        while let Some(val) = stack.pop() {
            vec.push(val);
        }
        assert_eq!(vec, vec![789, 456, 123])
    }
}
