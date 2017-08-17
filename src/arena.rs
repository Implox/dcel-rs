use std::ops::{Index, IndexMut};
use std::convert::From;

/// Auto-implementing trait for types that are wrappers around
/// the usize type. Used for ensuring that IDs for one type of
/// arena cannot be used inside one of another type.
pub trait ArenaId: Copy + From<usize> + Into<usize> {}
impl<T> ArenaId for T where T: Copy + From<usize> + Into<usize> {}

/// Given a type name (preferably in the form `FooId` where Foo is
/// a type stored in an arena), generates a wrapper around usize
/// that satisfies the ArenaId trait.
macro_rules! arena_id {
    ($T:ident) => {
        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        pub struct $T(usize);

        impl Into<usize> for $T {
            fn into(self) -> usize {
                let $T(idx) = self;
                idx
            }
        }

        impl From<usize> for $T {
            fn from(id: usize) -> Self {
                $T(id)
            }
        }
    }
}

/// Trait for checking if an item in an Arena has been deleted.
pub trait Deleteable {
    fn is_deleted(&self) -> bool;
    fn set_deleted(&mut self);
}

/// Automatically implements the Deleteable trait for a type
/// that has a boolean field named `deleted`.
/// WILL FAIL WITHOUT THE `deleted` FIELD!!!
macro_rules! make_deleteable {
    ($T:ident) => {
        impl Deleteable for $T {
            fn is_deleted(&self) -> bool {
                return self.deleted;
            }
             
            fn set_deleted(&mut self) {
                self.deleted = true;
            }
        }
    }
}

#[derive(Debug)]
pub struct Arena<T, Idx> 
where T: Deleteable, Idx: ArenaId {
    data: Vec<T>,
    idx_reuse_stack: Vec<Idx>,
}

impl<T, Idx> Arena<T, Idx> 
where T: Deleteable,
      Idx: ArenaId {
    pub fn new() -> Arena<T, Idx> {
        let data = Vec::new();
        let idx_stack = Vec::new();
        Arena { data: data, idx_reuse_stack: idx_stack }
    }
    
    pub fn add(&mut self, item: T) -> Idx {
        let idx: Idx;
        if self.idx_reuse_stack.is_empty() {
            idx = self.data.len().into();
            self.data.push(item);
        } else {
            idx = self.idx_reuse_stack.pop().unwrap();
            self.data[idx.into()] = item;
        }
        return idx;
    }
    
    pub fn remove(&mut self, idx: Idx) {
        let index: usize = idx.into();
        if index < self.data.len() && self.data[index].is_deleted() {
            self.data[index].set_deleted();
            self.idx_reuse_stack.push(idx);
        }
    }
}

impl<T, Idx> Index<Idx> for Arena<T, Idx> 
where T: Deleteable,
      Idx: ArenaId {
    type Output = T;

    fn index(&self, node: Idx) -> &T {
        let value = &self.data[node.into()];

        // Hacky code
        if value.is_deleted() {
            panic!("Value has been deleted!");
        }

        return value;
    }
}

impl<T, Idx> IndexMut<Idx> for Arena<T, Idx>
where T: Deleteable,
      Idx: ArenaId {
    fn index_mut(&mut self, node: Idx) -> &mut T {
        let value = &mut self.data[node.into()];

        // More hacky code
        if value.is_deleted() {
            panic!("Value has been deleted!");
        }

        return value;
    }
}