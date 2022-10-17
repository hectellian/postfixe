//! Stack implementation Module

#![allow(dead_code,unused_variables)]

#[derive(Debug)]
/// Defining a Stack Structure using Vec
pub struct Stack<T> {
    /// The stack is just a Vec
    pub stack: Vec<T>
}

/// Implementation of the Stack and its operations
impl<T> Stack<T> {
    /// Creates a new Stack
    pub fn new() -> Self {
        Stack { stack: Vec::new() }
    }

    /// Removes the top item (returns it)
    pub fn pop(&mut self) -> Option<T> {
        self.stack.pop()
    }
      
    /// Adds item in Stack
    pub fn push(&mut self, item: T) {
        self.stack.push(item)
    }

    /// Check if Stack is empty
    pub fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }

    /// Returns the lenght of the Stack
    pub fn length(&self) -> usize {
        self.stack.len()
    }

    /// Returns the top item
    pub fn peek(&self) -> Option<&T> {
        self.stack.last()
    }
}