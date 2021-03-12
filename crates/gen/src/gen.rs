use super::*;
use std::iter::FromIterator;

pub enum Gen {
    Absolute,
    Relative(&'static str),
}

impl Gen {
    pub fn namespace(&self, namespace: &str) -> TokenStream {
        match self {
            Self::Absolute => {
                let mut tokens = TokenStream::new();

                for namespace in namespace.split('.') {
                    let namespace = to_ident(&to_snake(namespace));

                    tokens.combine(&quote! { #namespace:: });
                }

                tokens
            }
            Self::Relative(relative) => {
                if namespace == *relative {
                    return TokenStream::new();
                }

                let mut tokens = Vec::new();
                let mut relative = relative.split('.').peekable();
                let mut namespace = namespace.split('.').peekable();

                while relative.peek() == namespace.peek() {
                    if relative.next().is_none() {
                        break;
                    }
                    namespace.next();
                }

                let count = relative.count();

                if count > 0 {
                    tokens.resize(tokens.len() + count, quote! { super:: });
                }

                tokens.extend(namespace.map(|namespace| {
                    let namespace = to_ident(&to_snake(namespace));
                    quote! { #namespace:: }
                }));

                TokenStream::from_iter(tokens)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_namespace() {
        let reader = TypeReader::get();
        let t = reader.resolve_type("Windows.Foundation", "IStringable");

        assert_eq!(
            t.gen_name(&Gen::absolute(&TypeTree::from_namespace("")))
                .as_str(),
            "windows :: foundation :: IStringable"
        );

        assert_eq!(
            t.gen_name(&Gen::relative("Windows", &TypeTree::from_namespace("")))
                .as_str(),
            "foundation :: IStringable"
        );

        assert_eq!(
            t.gen_name(&Gen::relative(
                "Windows.Foundation",
                &TypeTree::from_namespace("")
            ))
            .as_str(),
            "IStringable"
        );

        assert_eq!(
            t.gen_name(&Gen::relative(
                "Windows.Foundation.Collections",
                &TypeTree::from_namespace("")
            ))
            .as_str(),
            "super :: IStringable"
        );
    }
}
