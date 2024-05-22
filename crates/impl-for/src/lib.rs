use std::{collections::HashMap, mem};

use darling::{export::NestedMeta, FromMeta, ToTokens};
use proc_macro2::Ident;
use replace_types::{ReplaceTypes, VisitMut};
use syn::{parse_macro_input, Attribute, ItemImpl, Path, TypePath};

fn parse_substitutions(
    nested: impl AsRef<[NestedMeta]>,
) -> darling::Result<HashMap<TypePath, TypePath>> {
    let substitutions = HashMap::<Ident, TypePath>::from_list(nested.as_ref())?;

    let substitutions: HashMap<TypePath, TypePath> = substitutions
        .into_iter()
        .map(|(ident, type_path)| {
            (
                TypePath {
                    qself: None,
                    path: Path::from(ident),
                },
                type_path,
            )
        })
        .collect();

    Ok(substitutions)
}

/// Repeat an implementation with type substitutions
///
/// ## Example
///
/// ```
/// pub trait IntoBytes {
///     fn into_bytes(self) -> Vec<u8>;
/// }
///
/// #[impl_for(T = "i8")]
/// #[impl_for(T = "u8")]
/// #[impl_for(T = "i16")]
/// #[impl_for(T = "u16")]
/// #[impl_for(T = "i32")]
/// #[impl_for(T = "u32")]
/// #[impl_for(T = "i64")]
/// #[impl_for(T = "u64")]
/// #[impl_for(T = "isize")]
/// #[impl_for(T = "usize")]
/// impl IntoBytes for T {
///     fn into_bytes(self) -> Vec<u8> {
///         let mut buf = ::itoa::Buffer::new();
///         let s = buf.format(self);
///         s.as_bytes().to_vec()
///     }
/// }
/// ```
#[proc_macro_attribute]
pub fn impl_for(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let mut input = parse_macro_input!(input as ItemImpl);

    let mut errors: Vec<darling::Error> = Vec::new();
    let mut substitutions_list: Vec<HashMap<TypePath, TypePath>> = Vec::new();

    match NestedMeta::parse_meta_list(args.into())
        .map_err(darling::Error::from)
        .and_then(parse_substitutions)
    {
        Ok(substitutions) => {
            substitutions_list.push(substitutions);
        }
        Err(err) => {
            errors.push(err);
        }
    }

    let mut attrs: Vec<Attribute> = Vec::new();

    let input_attrs = mem::take(&mut input.attrs);

    for attr in input_attrs.into_iter() {
        if attr.path().is_ident("impl_for") {
            match attr.meta.require_list() {
                Ok(list) => {
                    match NestedMeta::parse_meta_list(list.tokens.to_owned())
                        .map_err(darling::Error::from)
                        .and_then(parse_substitutions)
                    {
                        Ok(substitutions) => {
                            substitutions_list.push(substitutions);
                        }
                        Err(err) => {
                            errors.push(err);
                        }
                    }
                }
                Err(err) => {
                    errors.push(err.into());
                }
            }
        } else {
            attrs.push(attr);
        }
    }

    if !errors.is_empty() {
        return darling::Error::multiple(errors).write_errors().into();
    }

    input.attrs = attrs;

    substitutions_list
        .into_iter()
        .map(|substitutions| {
            let mut item_impl = input.clone();
            ReplaceTypes::new(substitutions).visit_item_impl_mut(&mut item_impl);
            proc_macro::TokenStream::from(item_impl.into_token_stream())
        })
        .collect::<proc_macro::TokenStream>()
}
