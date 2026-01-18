use std::ops::{Add, Deref};

use darling::ast::NestedMeta;
use darling::util::PathList;
use darling::FromMeta;
use syn::{Attribute, Ident, Meta};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct IdentList(Vec<Ident>);

impl IdentList {
    pub fn new<T: Into<Ident>>(values: Vec<T>) -> Self {
        Self(values.into_iter().map(T::into).collect())
    }
}

impl Deref for IdentList {
    type Target = Vec<Ident>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Vec<Ident>> for IdentList {
    fn from(values: Vec<Ident>) -> Self {
        Self(values)
    }
}

impl FromMeta for IdentList {
    fn from_list(items: &[NestedMeta]) -> darling::Result<Self> {
        let values = items
            .iter()
            .map(|item| match item {
                NestedMeta::Meta(Meta::Path(path)) => match path.get_ident() {
                    Some(ident) => Ok(ident.clone()),
                    None => Err(darling::Error::unexpected_type("non ident").with_span(item)),
                },
                _ => {
                    Err(darling::Error::unexpected_type("non path, expected ident").with_span(item))
                }
            })
            .collect::<darling::Result<Vec<Ident>>>()?;

        if values.is_empty() {
            return Err(darling::Error::too_few_items(1));
        }

        Ok(Self::new(values))
    }
}


// Source - https://stackoverflow.com/a/56264023
// Posted by David Bernard, modified by community. See post 'Timeline' for change history
// Retrieved 2026-01-17, License - CC BY-SA 4.0

pub fn extract_type_from_option(ty: &syn::Type) -> Option<&syn::Type> {
    use syn::{GenericArgument, Path, PathArguments, PathSegment};

    fn extract_type_path(ty: &syn::Type) -> Option<&Path> {
        match *ty {
            syn::Type::Path(ref typepath) if typepath.qself.is_none() => Some(&typepath.path),
            _ => None,
        }
    }

    // TODO store (with lazy static) the vec of string
    // TODO maybe optimization, reverse the order of segments
    fn extract_option_segment(path: &Path) -> Option<&PathSegment> {
        let idents_of_path = path
            .segments
            .iter()
            .into_iter()
            .fold(String::new(), |mut acc, v| {
                acc.push_str(&v.ident.to_string());
                acc.push('|');
                acc
            });
        vec!["Option|", "std|option|Option|", "core|option|Option|"]
            .into_iter()
            .find(|s| &idents_of_path == *s)
            .and_then(|_| path.segments.last())
    }

    extract_type_path(ty)
        .and_then(|path| extract_option_segment(path))
        .and_then(|path_seg| {
            let type_params = &path_seg.arguments;
            // It should have only on angle-bracketed param ("<String>"):
            match *type_params {
                PathArguments::AngleBracketed(ref params) => params.args.first(),
                _ => None,
            }
        })
        .and_then(|generic_arg| match *generic_arg {
            GenericArgument::Type(ref ty) => Some(ty),
            _ => None,
        })
}

#[derive(Clone, Debug, Default, PartialEq)]
pub enum ForwardAttrsFilter {
    // forward all attributes
    #[default]
    All,

    // forward only specific attributes
    Some(PathList),

    // forward all attributes except specific ones
    Not(PathList),
}

impl Add for &ForwardAttrsFilter {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        // The lhs should be the smaller scope (field, arg)
        // The rhs should be the larger scope (arg, container)
        // Currently, we implement this simply by always using the lhs if it is not `All`
        // TODO: A more appropriate way should be discussed and implemented
        match (self, rhs) {
            (ForwardAttrsFilter::All, _) => rhs,
            _ => self,
        }
    }
}

impl FromMeta for ForwardAttrsFilter {
    fn from_list(items: &[NestedMeta]) -> darling::Result<Self> {
        if items.len() == 1 {
            match &items[0] {
                NestedMeta::Meta(Meta::Path(path)) => {
                    if path.is_ident("*") {
                        Ok(Self::All)
                    } else {
                        Ok(Self::Some(PathList::from_list(&[items[0].clone()])?))
                    }
                }
                NestedMeta::Meta(Meta::List(list)) => {
                    if list.path.is_ident("not") {
                        Ok(Self::Not(PathList::from_list(
                            &NestedMeta::parse_meta_list(list.tokens.clone())?[..],
                        )?))
                    } else {
                        Err(
                            darling::Error::unknown_value("expected `not(attr1, attr2, ...)`")
                                .with_span(&items[0]),
                        )
                    }
                }
                _ => Err(darling::Error::unexpected_type("non path or list").with_span(&items[0])),
            }
        } else {
            Ok(Self::Some(PathList::from_list(items)?))
        }
    }
}

pub fn filter_forward_attrs<'a>(
    attrs: impl Iterator<Item = &'a Attribute> + 'a,
    filter: &'a ForwardAttrsFilter,
) -> impl Iterator<Item = &'a Attribute> + 'a {
    attrs.filter(move |attr| match filter {
        ForwardAttrsFilter::All => true,
        ForwardAttrsFilter::Some(allowed) => allowed.contains(attr.path()),
        ForwardAttrsFilter::Not(not_allowed) => !not_allowed.contains(attr.path()),
    })
}
