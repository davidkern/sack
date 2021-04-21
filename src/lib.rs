use std::marker::PhantomData;
use typed_arena::Arena;


pub struct Sack<T> {
    arena: Arena<T>,
    marker: PhantomData<T>,
}

impl<T> Sack<T> {
    pub fn new() -> Self {
        Sack {
            arena: Arena::new(),
            marker: PhantomData,
        }
    }

    pub fn with_capacity(n: usize) -> Self {
        Sack {
            arena: Arena::with_capacity(n),
            marker: PhantomData,
        }
    }

    pub fn alloc(&self, value: T) -> &mut T {
        self.arena.alloc(value)
    }
}
