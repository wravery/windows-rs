use super::*;

pub enum TypeInclusion {
    Included, // the type will be generated
    NotIncluded, // the type will be omitted
    NotGenerated, // the type will be declared as NotGenerated<T>
}

// TODO: replaces public TypeTree but keep it private and only expose methods
// on TypeReader itself - including gen()
pub struct TypeNamespace {
    pub namespace: &'static str,
    pub types: BTreeMap<&'static str, (TypeRow, TypeInclusion)>,
    pub namespaces: BTreeMap<&'static str, TypeNamespace>,
}

impl TypeNamespace {
    pub fn new(namespace: &'static str) -> Self {
        Self {
            namespace,
            types: BTreeMap::default(),
            namespaces: BTreeMap::default(),
        }
    }

    pub fn add_type(&mut self, namespace: &'static str, name: &'static str, def: TypeRow) {
        self.add_type_impl(namespace, 0, name, def)
    }

    pub fn include_types(&'static mut self, types: &[String]) {
        if types.is_empty() {
            for pair in self.types.values_mut() {
                pair.1 = TypeInclusion::Included;
            }
        } else {
            for name in types {
                if let Some(pair) = self.types.get_mut(name.as_str()) {
                    pair.1 = TypeInclusion::Included;
                } else {
                    // TODO: this needs to return a syntax error so the point of failure is highlighted.
                    panic!("Could not find type {}.{}", self.namespace, name);
                }
            }
        }
    }

    pub fn find_type(&self, namespace: &str, name: &str) -> Option<&(TypeRow, TypeInclusion)> {
        if let Some(next) = namespace.find('.') {
            if let Some(tree) = self.namespaces.get(&namespace[..next]) {
                return tree.find_type(&namespace[next + 1..], name);
            }
        } else {
            if let Some(tree) = self.namespaces.get(namespace) {
                return tree.types.get(name);
            }
        }

        None
    }

    pub fn find_lower_namespace(&mut self, namespace: &str) -> Option<&mut TypeNamespace> {
        if let Some(next) = namespace.find('.') {
            for (name, tree) in &mut self.namespaces {
                if name.to_lowercase() == &namespace[..next] {
                    return tree.find_lower_namespace(&namespace[next + 1..]);
                }
            }
        } else {
            for (name, tree) in &mut self.namespaces {
                if name.to_lowercase() == namespace {
                    return Some(tree);
                }
            }
        }

        None
    }

    fn add_type_impl(&mut self, namespace: &'static str, pos: usize, name: &'static str, def: TypeRow) {
        if let Some(next) = namespace[pos..].find('.') {
            let next = pos + next;
            self.namespaces
                .entry(&namespace[pos..next])
                .or_insert_with(|| Self::new(&namespace[..next]))
                .add_type_impl(namespace, next + 1, name, def);
        } else {
            self.namespaces
                .entry(&namespace[pos..])
                .or_insert_with(|| Self::new(namespace))
                .types
                .entry(name)
                .or_insert_with(|| (def, TypeInclusion::NotIncluded));
        }
    }
}
