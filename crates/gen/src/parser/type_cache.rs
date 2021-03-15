// use super::*;

// pub struct TypeCache {
//     // TODO: should be type tree instead
//     types: BTreeMap<&'static str, BTreeMap<&'static str, TypeRow>>,
// }

// // #[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Debug)]
// // pub struct CacheRow {
// //     pub index: u32,
// //     pub table_index: TableIndex,
// //     pub file_index: u16,
// // }


// enum TypeRow {
//     TypeDef((Row, Inclusion)),
//     Function(Row),
//     Constant(Row),
// }

// enum Inclusion {
//     Included,
//     NotIncluded,
// }

// impl TypeCache {
//     pub fn get() -> &'static Self {
//         use std::{mem::MaybeUninit, sync::Once};
//         static ONCE: Once = Once::new();
//         static mut VALUE: MaybeUninit<TypeCache> = MaybeUninit::uninit();

//         ONCE.call_once(|| {
//             // This is safe because `Once` provides thread-safe one-time initialization
//             unsafe { VALUE = MaybeUninit::new(Self{ types: BTreeMap::default() }) }
//         });

//         // This is safe because `call_once` has already been called.
//         unsafe { &*VALUE.as_ptr() }
//     }

//     fn prime() -> Self {
//         let files = FileCache::get();

//         let mut types = Default::default();

//         for (index, file) in files.0.iter().enumerate() {
//             let index = index as u16;
//             let row_count = file.type_def_table().row_count;

//             for row in 0..row_count {
//                 let def = Row::new(row, TableIndex::TypeDef, index);
//                 let namespace = reader.str(def, 2);
//                 let name = trim_tick(reader.str(def, 1));

//                 if namespace.is_empty() {
//                     continue;
//                 }


//             }
//         }

//         Self { 
//             types
//         }
//     }
// }

// fn trim_tick(name: &'static str) -> &'static str {
//     match name.as_bytes().get(name.len() - 2) {
//         Some(c) if *c == b'`' => &name[..name.len() - 2],
//         _ => name,
//     }
// }
