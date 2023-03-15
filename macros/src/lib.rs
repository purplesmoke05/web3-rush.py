use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::{quote, ToTokens, TokenStreamExt};
use syn::{parse_macro_input, ImplItem, ItemEnum, ItemImpl, DeriveInput, punctuated::Punctuated, Path, Token, ItemStruct, ItemType, ItemTraitAlias, Item, AttributeArgs, parse::ParseBuffer};

/// Add mappings to and from another enum that has the exact same fields.
///
/// # Example
///
/// ```rust
/// use web3_rush_macros::enum_original_mapping;
///
/// #[derive(PartialEq, Debug)]
/// pub enum Foo {
///   A,
///   B
/// }
/// #[enum_original_mapping(Foo)]
/// #[derive(PartialEq, Debug)]
/// pub enum Bar {
///   A,
///   B,
/// }
///
/// let a = Bar::A;
/// let b = Foo::B;
/// assert_eq!(Foo::from(a), Foo::A);
/// assert_eq!(Bar::from(b), Bar::B);
///
#[proc_macro_attribute]
pub fn enum_original_mapping(original: TokenStream, item: TokenStream) -> TokenStream {
    let mut new_stream = proc_macro2::TokenStream::from(item.clone());
    let ast = parse_macro_input!(item as ItemEnum);
    let enum_name = ast.ident;
    let orig = parse_macro_input!(original as Ident);
    let variant_names: Vec<Ident> = ast.variants.into_iter().map(|v| v.ident).collect();
    let from_impl = quote! {
        impl From<#orig> for #enum_name {
            fn from(left: #orig) -> Self {
                match left {
                    #(#orig::#variant_names => Self::#variant_names),*,
                    _ => panic!("Unrecognized variant: {:?}", left)
                }
            }
        }

        impl From<#enum_name> for #orig {
            fn from(left: #enum_name) -> Self {
                match left {
                    #(#enum_name::#variant_names => Self::#variant_names),*
                }
            }
        }
    };
    new_stream.extend(from_impl);
    TokenStream::from(new_stream)
}

/// Impl IntoPy<PyObject> for an ADT where each variant is a newtype.
///
/// # Example
///
/// ```rust
/// use web3_rush_macros::EnumIntoPy;
///
/// #[derive(PartialEq, Debug, EnumIntoPy)]
/// pub enum Foo {
///   A(u8),
///   B(u8)
/// }
///
#[proc_macro_derive(EnumIntoPy)]
pub fn enum_into_py(item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as ItemEnum);
    let enum_name = ast.ident;
    let variant_names: Vec<Ident> = ast.variants.into_iter().map(|v| v.ident).collect();
    let into_py_impl = quote! {
        impl IntoPy<PyObject> for #enum_name {
            fn into_py(self, py: Python<'_>) -> PyObject {
                match self {
                    #(Self::#variant_names(x) => x.into_py(py)),*,
                }
            }
        }
    };
    into_py_impl.to_token_stream().into()
}


/// Add mappings to and from another struct that has the exact same fields.
///
/// # Example
///
/// ```rust
/// use web3_rush_macros::struct_original_mapping;
///
/// #[derive(PartialEq, Debug)]
/// pub struct Foo {
///   a: String,
///   b: u64,
/// }
/// #[struct_original_mapping(Foo)]
/// #[derive(PartialEq, Debug)]
/// pub struct Bar {
///   a: String,
///   b: u64,
/// }
///
/// let a: Foo = Bar{a: "".to_owned(), b: 0}.into();
/// let b: Bar = Foo{a: "".to_owned(), b: 0}.into();;
/// assert_eq!(a, b.into());
///
#[proc_macro_attribute]
pub fn struct_original_mapping(original: TokenStream, item: TokenStream) -> TokenStream {
    let mut new_stream = proc_macro2::TokenStream::from(item.clone());
    let ast = parse_macro_input!(item as ItemStruct);
    let target_name = ast.ident;
    let orig = parse_macro_input!(original as Ident);
    let field_names: Vec<Ident> = ast.fields.into_iter().map(|v| v.ident.expect("tuple struct not supported")).collect();

    let into_impl = quote! {
        impl Into<#target_name> for #orig {
            fn into(self) -> #target_name {
                #target_name {
                    #(#field_names: self.#field_names.into()),*
                }
            }
        }
        impl From<#target_name> for #orig {
            fn from(value: #target_name) -> #orig {
                #orig {
                    #(#field_names: value.#field_names.into()),*
                }
            }
        }
    };
    new_stream.extend(into_impl);
    TokenStream::from(new_stream)
}


/// Add mappings to and from another tuple struct that has inner fields.
///
/// # Example
///
/// ```rust
/// use web3_rush_macros::tuple_struct_original_mapping;
///
/// #[derive(PartialEq, Debug, Clone)]
/// pub struct Foo {
///   a: String,
///   b: u64,
/// }
/// #[tuple_struct_original_mapping(Foo)]
/// #[derive(PartialEq, Debug)]
/// pub struct Bar(pub Foo);
///
/// let a = Foo{a: "".to_owned(), b: 0};
/// let b: Bar = Bar(a.clone());
/// assert_eq!(a, b.into());
///
#[proc_macro_attribute]
pub fn tuple_struct_original_mapping(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut new_stream = proc_macro2::TokenStream::from(item.clone());
    let ast = parse_macro_input!(item as ItemStruct);
    let target_name = ast.ident;
    let orig = parse_macro_input!(attr as Ident);

    let into_impl = quote! {
        impl Into<#orig> for #target_name {
            fn into(self) -> #orig {
                self.0
            }
        }
        impl From<#orig> for #target_name {
            fn from(value: #orig) -> #target_name {
                #target_name(
                    value
                )
            }
        }
    };
    new_stream.extend(into_impl);
    
    TokenStream::from(new_stream)
}
