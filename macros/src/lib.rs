use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::{quote, ToTokens, TokenStreamExt};
use syn::{parse_macro_input, ImplItem, ItemEnum, Result, ItemImpl, DeriveInput, punctuated::Punctuated, Path, Token, ItemStruct, ItemType, ItemTraitAlias, Item, AttributeArgs, parse::{ParseBuffer, ParseStream, Parse}, Data, Fields, Type, PathSegment, PathArguments, GenericArgument};

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
                    #(#orig::#variant_names => Self::#variant_names.into()),*,
                    _ => panic!("Unrecognized variant: {:?}", left)
                }
            }
        }

        impl From<#enum_name> for #orig {
            fn from(left: #enum_name) -> Self {
                match left {
                    #(#enum_name::#variant_names => Self::#variant_names.into()),*
                }
            }
        }
    };
    new_stream.extend(from_impl);
    TokenStream::from(new_stream)
}

#[proc_macro_attribute]
pub fn tuple_enum_original_mapping(
    args: TokenStream,
    input: TokenStream,
) -> TokenStream {
    let args = parse_macro_input!(args as syn::AttributeArgs);
    let input = parse_macro_input!(input as DeriveInput);
    let input2 = input.clone();

    let target_enum = match args.get(0) {
        Some(syn::NestedMeta::Meta(syn::Meta::Path(target_enum))) => target_enum.clone(),
        _ => panic!("Invalid arguments. Expecting target_enum."),
    };

    let enum_ident = input.ident;
    let variants = match input.data {
        Data::Enum(e) => e.variants,
        _ => panic!("This macro supports only enums."),
    };

    let conversion_variants_into = variants.clone().into_iter().map(|variant| {
        let variant_ident = variant.ident;
        let fields = variant.fields;

        match fields {
            syn::Fields::Unnamed(fields) => {
                let len = fields.unnamed.len();
                let field_indices: Vec<_> = (0..len)
                    .map(|i| syn::Ident::new(&format!("_{}", i), proc_macro2::Span::call_site()))
                    .collect();

                quote! {
                    #enum_ident::#variant_ident(#(#field_indices),*) => {
                        #target_enum::#variant_ident(#(#field_indices.into()),*)
                    },
                }
            }
            _ => panic!("This macro supports only tuple variants."),
        }
    });
    let conversion_variants_from = variants.into_iter().map(|variant| {
        let variant_ident = variant.ident;
        let fields = variant.fields;

        match fields {
            syn::Fields::Unnamed(fields) => {
                let len = fields.unnamed.len();
                let field_indices: Vec<_> = (0..len)
                    .map(|i| syn::Ident::new(&format!("_{}", i), proc_macro2::Span::call_site()))
                    .collect();

                quote! {
                    #target_enum::#variant_ident(#(#field_indices),*) => {
                        #enum_ident::#variant_ident(#(#field_indices.into()),*)
                    },
                }
            }
            _ => panic!("This macro supports only tuple variants."),
        }
    });

    let into_impl = quote! {
        impl Into<#target_enum> for #enum_ident {
            fn into(self) -> #target_enum {
                match self {
                    #(#conversion_variants_into)*
                }
            }
        }
        impl From<#target_enum> for #enum_ident {
            fn from(value: #target_enum) -> #enum_ident {
                match value {
                    #(#conversion_variants_from)*
                }
            }
        }
    };

    let output = quote! {
        #input2
        #into_impl
    };

    output.into()
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

fn get_last_path_segment(ty: &Type) -> Option<&PathSegment> {
    match ty {
        Type::Path(path) => path.path.segments.last(),
        _ => None,
    }
}

fn is_vec(ty: &Type) -> bool {
    match get_last_path_segment(ty) {
        Some(seg) => seg.ident == "Vec",
        _ => false,
    }
}

fn unwrap_item(ty: &Type) -> Option<&Type> {
    if !is_vec(ty) {
        return None;
    }
    match get_last_path_segment(ty) {
        Some(seg) => match seg.arguments {
            PathArguments::AngleBracketed(ref args) => {
                args.args.first().and_then(|arg| match arg {
                    &GenericArgument::Type(ref ty) => Some(ty),
                    _ => None,
                })
            }
            _ => None,
        },
        None => None,
    }
}

fn is_option(ty: &Type) -> bool {
    match get_last_path_segment(ty) {
        Some(seg) => seg.ident == "Option",
        _ => false,
    }
}

fn unwrap_option(ty: &Type) -> Option<&Type> {
    if !is_option(ty) {
        return None;
    }
    match get_last_path_segment(ty) {
        Some(seg) => match seg.arguments {
            PathArguments::AngleBracketed(ref args) => {
                args.args.first().and_then(|arg| match arg {
                    &GenericArgument::Type(ref ty) => Some(ty),
                    _ => None,
                })
            }
            _ => None,
        },
        None => None,
    }
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
    let input = parse_macro_input!(item as DeriveInput);
    let orig = parse_macro_input!(original as syn::Ident);
    
    let target_name = input.ident;
    let fields = match input.data {
        Data::Struct(s) => match s.fields {
            Fields::Named(named) => named.named,
            _ => panic!("This macro supports only named fields.")
        },
        _ => panic!("This macro supports only structs."),
    };

    let conversion_fields_from = fields.clone().into_iter().map(|field|{
        let field_name = field.ident.unwrap();
        let field_type = field.ty;
        match is_option(&field_type) {
            true => {
                let unwrapped_field_type = unwrap_option(&field_type);
                match unwrapped_field_type {
                    Some(field_type) => quote! {
                        #field_name: match value.#field_name {Some(f) => Some(f.into()), None => None}
                    },
                    None => panic!("Unwrap failed"),
                }
            },
            false => {
                match is_vec(&field_type) {
                    true => {
                        let item_type = unwrap_item(&field_type);
                        match item_type {
                            Some(item_type) => quote!{
                                #field_name: value.#field_name.into_iter().map(|i|{i.into()}).collect()
                            },
                            None => panic!("Unwrap failed")
                        }
                    },
                    false => quote!{#field_name: value.#field_name.into()},
                }
            }
        }
    });

    let conversion_fields_into = fields.into_iter().map(|field| {
        let field_name = field.ident.unwrap();
        let field_type = field.ty;
        match is_option(&field_type) {
            true => {
                let unwrapped_field_type = unwrap_option(&field_type);
                match unwrapped_field_type {
                    Some(field_type) => quote! {
                        #field_name: match self.#field_name {Some(f) => Some(f.into()), None => None}
                    },
                    None => panic!("Unwrap failed"),
                }
            },
            false => {
                match is_vec(&field_type) {
                    true => {
                        let item_type = unwrap_item(&field_type);
                        match item_type {
                            Some(item_type) => quote!{
                                #field_name: self.#field_name.into_iter().map(|i|{i.into()}).collect()
                            },
                            None => panic!("Unwrap")
                        }
                    },
                    false => quote!{#field_name: self.#field_name.into()},
                }
            }
        }
    });

    let into_impl = quote! {
        impl Into<#target_name> for #orig {
            fn into(self) -> #target_name {
                #target_name {
                    #(#conversion_fields_into),*
                }
            }
        }
        impl From<#target_name> for #orig {
            fn from(value: #target_name) -> #orig {
                #orig {
                    #(#conversion_fields_from),*
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
                self.0.into()
            }
        }
        impl From<#orig> for #target_name {
            fn from(value: #orig) -> #target_name {
                #target_name(
                    value
                ).into()
            }
        }
    };
    new_stream.extend(into_impl);
    
    TokenStream::from(new_stream)
}
