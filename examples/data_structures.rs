// PlayJS Rust Example: Data Structures
// Demonstrates custom data structures: Stack, Queue, and Linked List

use std::fmt::Display;

// ========== Stack Implementation ==========
struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Stack { items: Vec::new() }
    }

    fn push(&mut self, item: T) {
        self.items.push(item);
    }

    fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }

    fn peek(&self) -> Option<&T> {
        self.items.last()
    }

    fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    fn size(&self) -> usize {
        self.items.len()
    }
}

// ========== Queue Implementation ==========
struct Queue<T> {
    items: Vec<T>,
}

impl<T> Queue<T> {
    fn new() -> Self {
        Queue { items: Vec::new() }
    }

    fn enqueue(&mut self, item: T) {
        self.items.push(item);
    }

    fn dequeue(&mut self) -> Option<T> {
        if self.items.is_empty() {
            None
        } else {
            Some(self.items.remove(0))
        }
    }

    fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    fn size(&self) -> usize {
        self.items.len()
    }
}

// ========== Linked List Implementation ==========
#[derive(Debug)]
struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

#[derive(Debug)]
struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
    size: usize,
}

impl<T: Display> LinkedList<T> {
    fn new() -> Self {
        LinkedList {
            head: None,
            size: 0,
        }
    }

    fn push_front(&mut self, data: T) {
        let new_node = Box::new(Node {
            data,
            next: self.head.take(),
        });
        self.head = Some(new_node);
        self.size += 1;
    }

    fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            self.size -= 1;
            node.data
        })
    }

    fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    fn len(&self) -> usize {
        self.size
    }

    fn display(&self) {
        let mut current = &self.head;
        print!("List: ");
        while let Some(node) = current {
            print!("{} -> ", node.data);
            current = &node.next;
        }
        println!("None");
    }
}

// ========== Binary Tree Implementation ==========
#[derive(Debug)]
struct TreeNode<T> {
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

impl<T: Ord + Display> TreeNode<T> {
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }

    fn insert(&mut self, value: T) {
        if value < self.value {
            match &mut self.left {
                Some(node) => node.insert(value),
                None => self.left = Some(Box::new(TreeNode::new(value))),
            }
        } else {
            match &mut self.right {
                Some(node) => node.insert(value),
                None => self.right = Some(Box::new(TreeNode::new(value))),
            }
        }
    }

    fn in_order_traversal(&self) {
        if let Some(left) = &self.left {
            left.in_order_traversal();
        }
        print!("{} ", self.value);
        if let Some(right) = &self.right {
            right.in_order_traversal();
        }
    }
}

fn main() {
    println!("=== Data Structures Demo ===\n");

    // Stack Demo
    println!("--- Stack Demo ---");
    let mut stack = Stack::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    println!("Stack size: {}", stack.size());
    println!("Peek: {:?}", stack.peek());
    println!("Pop: {:?}", stack.pop());
    println!("Pop: {:?}", stack.pop());
    println!("Stack size after pops: {}\n", stack.size());

    // Queue Demo
    println!("--- Queue Demo ---");
    let mut queue = Queue::new();
    queue.enqueue("First");
    queue.enqueue("Second");
    queue.enqueue("Third");
    println!("Queue size: {}", queue.size());
    println!("Dequeue: {:?}", queue.dequeue());
    println!("Dequeue: {:?}", queue.dequeue());
    println!("Queue size after dequeues: {}\n", queue.size());

    // Linked List Demo
    println!("--- Linked List Demo ---");
    let mut list = LinkedList::new();
    list.push_front(10);
    list.push_front(20);
    list.push_front(30);
    list.push_front(40);
    list.display();
    println!("List length: {}", list.len());
    println!("Pop front: {:?}", list.pop_front());
    list.display();
    println!();

    // Binary Search Tree Demo
    println!("--- Binary Search Tree Demo ---");
    let mut root = TreeNode::new(50);
    root.insert(30);
    root.insert(70);
    root.insert(20);
    root.insert(40);
    root.insert(60);
    root.insert(80);

    print!("In-order traversal: ");
    root.in_order_traversal();
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stack() {
        let mut stack = Stack::new();
        stack.push(1);
        stack.push(2);
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.peek(), Some(&1));
    }

    #[test]
    fn test_queue() {
        let mut queue = Queue::new();
        queue.enqueue(1);
        queue.enqueue(2);
        assert_eq!(queue.dequeue(), Some(1));
        assert_eq!(queue.dequeue(), Some(2));
    }

    #[test]
    fn test_linked_list() {
        let mut list = LinkedList::new();
        list.push_front(1);
        list.push_front(2);
        assert_eq!(list.len(), 2);
        assert_eq!(list.pop_front(), Some(2));
    }
}
