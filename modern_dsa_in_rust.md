https://dsar.rantai.dev/docs/dsar/  
Merges the time-honored concepts of data structures and algorithms with the modern, powerful features of the Rust programming language.  

# Preface
The best way to learn is to teach. We embarked on this project with a deep-seated desire to unlearn,relearn,and master data structures and algorithms through the lens of modern technology and methodologies. By integrating AI into our learning process, we aim to transcend the limitation of conventional methods, allowing us to absorb and apply knowledge in a more intuitive and impactful way.  
We believe that mastering data structures and algorithms should be more than just an academic exercises; it should be a gateway to innovation and career advancement. Remember that learning is a continuous and evolving process.  
Weekly Study Plan: We recommend dedicating one week to each chapter to fully engage with the material, complete practical exercises, and solidify your understanding.

# Chapter 1 The Role of Algorithms in Modern Software

# Chapter 2 Introduction to Data Structures and Algorithms in Rust 
## Overview of Essential Data Structures 
Arrays, Vectors, Linked List, Hash Map, Binary Trees, and Graphs.
```rust 
let arr:[i32;5] = [1,2,3,4,5];
let mut vec = Vec::new();
vec.push(1);
use std::collections::LinkedList;
let mut list = LinkedList::new();
list.push_back(1);
use std::collections::HashMap;
let mut map = HashMap::new();
map.insert("key","value");

struct TreeNode {
    value: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}
```
Why Use Box? Rust requires that types know their size at compile time. With recursive types, the size may not be known. Box helps by allowing indirection, making recursive types possible.
```rust
enum List {
    Cons(i32, Box<List>),
    Nil,
}
use List::{Cons, Nil};
fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}
```

