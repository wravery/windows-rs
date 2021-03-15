use super::*;

pub struct TypeCache {
    files: &'static FileCache,

    types: BTreeMap<&'static str, BTreeMap<&'static str, TypeRow>>,
}

enum TypeRow {
    TypeDef((Row, Inclusion)),
    Function(Row),
    Constant(Row),
}

enum Inclusion {
    Included,
    NotIncluded,
}

impl TypeCache {
    pub fn get() -> &'static Self {
        use std::{mem::MaybeUninit, sync::Once};
        static ONCE: Once = Once::new();
        static mut VALUE: MaybeUninit<TypeCache> = MaybeUninit::uninit();

        ONCE.call_once(|| {
            // This is safe because `Once` provides thread-safe one-time initialization
            unsafe { VALUE = MaybeUninit::new(Self::new()) }
        });

        // This is safe because `call_once` has already been called.
        unsafe { &*VALUE.as_ptr() }
    }

    fn new() -> Self {
        panic!();
    }
}