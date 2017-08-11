/*
    ARENA PROTOTYPE
    Try running this in playground
*/

use std::ops::{Index, IndexMut};

trait Valid {
    fn is_valid(&self) -> bool;
    fn set_valid(&mut self, val: bool);
}

#[derive(Debug, PartialEq)]
struct Foo {
    data: u32,
    valid: bool
}

impl Valid for Foo {
    fn is_valid(&self) -> bool {
        return self.valid;
    }
    
    fn set_valid(&mut self, val: bool) {
        self.valid = val;
    }
}

#[derive(Debug)]
struct Arena<T: PartialEq + Valid> {
    data: Vec<T>,
    idx_reuse_stack: Vec<usize>,
}

impl<T> Arena<T>
where T: PartialEq + Valid {
    pub fn new() -> Arena<T> {
        let data = Vec::new();
        let idx_stack = Vec::new();
        Arena { data: data, idx_reuse_stack: idx_stack }
    }
    
    pub fn add(&mut self, item: T) -> usize {
        let idx: usize;
        if self.idx_reuse_stack.is_empty() {
            idx = self.data.len();
            self.data.push(item);
        } else {
            idx = self.idx_reuse_stack.pop().unwrap();
            self.data[idx] = item;
        }
        return idx;
    }
    
    pub fn remove(&mut self, idx: usize) {
        if idx < self.data.len() && self.data[idx].is_valid() {
            self.data[idx].set_valid(false);
            self.idx_reuse_stack.push(idx);
        }
    }
}

impl<T> Index<usize> for Arena<T> 
where T: PartialEq + Valid {
    type Output = T;

    fn index(&self, node: usize) -> &T {
        &self.data[node]
    }
}

impl<T> IndexMut<usize> for Arena<T>
where T: PartialEq + Valid {
    fn index_mut(&mut self, node: usize) -> &mut T {
        &mut self.data[node]
    }
}

fn main() {
    let mut arena: Arena<Foo> = Arena::new();
    let foo = Foo { 
        data: 69, // nice
        valid: true,
    }

    let idx = arena.add(foo);
    println!("{:?}", arena);
    arena.remove(idx);
    println!("{:?}", arena);

    arena.add(Foo { data: 420, valid: true });
    {   // test mutation
        let mut item = &mut arena[idx];
        println!("{:?}", item);
        item.data = 42;
    }
    println!("{:?}", arena);
}