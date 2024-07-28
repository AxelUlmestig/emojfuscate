use proc_macro2::{Span, TokenStream};
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{
    parse_macro_input, parse_quote, Data, DeriveInput, Fields, GenericParam, Generics, Ident,
};

#[proc_macro_derive(ConstructFromEmoji)]
pub fn derive_construct_from_emoji(raw_input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Parse the input tokens into a syntax tree.
    let input = parse_macro_input!(raw_input as DeriveInput);

    // Used in the quasi-quotation below as `#name`.
    let name = input.ident;

    // // Add a bound `T: ConstructFromEmoji` to every type parameter T.
    // let generics = add_trait_bounds(input.generics);
    // let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    // // Generate an expression to sum up the heap size of each field.
    let demojfuscated_fields = demojfuscate_fields(&input.data, &name);

    let expanded = quote! {
        impl<I> ConstructFromEmoji<#name, I> for #name
        where
            I: Iterator<Item = u8>,
        {
            fn construct_from_emoji(
                mut byte_stream: DecodeEmojiToBytes<I>,
            ) -> Result<(#name, DecodeEmojiToBytes<I>), FromEmojiError> {
                #demojfuscated_fields
            }
        }
    };

    // Hand the output tokens back to the compiler.
    return proc_macro::TokenStream::from(expanded);
}

// Generate an expression to sum up the heap size of each field.
fn demojfuscate_fields(data: &Data, name: &Ident) -> TokenStream {
    match *data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => {
                let declare_fields = fields.named.iter().map(|f| {
                    let name = &f.ident;
                    let field_type = &f.ty;
                    quote_spanned! {f.span()=>
                        let #name = match #field_type::construct_from_emoji(byte_stream) {
                            Err(err) => return Err(err),
                            Ok((result, new_byte_stream)) => {
                                byte_stream = new_byte_stream;
                                result
                            }
                        };

                    }
                });

                let field_constructors = fields.named.iter().map(|f| {
                    let name = &f.ident;

                    quote_spanned! {f.span()=>
                        #name: #name,
                    }
                });

                quote! {
                    #(#declare_fields)*

                    return Ok((
                        #name {
                            #(#field_constructors)*
                        },
                        byte_stream
                    ));
                }
            }
            Fields::Unnamed(ref fields) => {
                let declare_fields = fields.unnamed.iter().enumerate().map(|(i, f)| {
                    let name = Ident::new(&format!("field{}", i), Span::call_site());
                    let field_type = &f.ty;
                    quote_spanned! {f.span()=>
                        let #name = match #field_type::construct_from_emoji(byte_stream) {
                            Err(err) => return Err(err),
                            Ok((result, new_byte_stream)) => {
                                byte_stream = new_byte_stream;
                                result
                            }
                        };

                    }
                });

                let field_constructors = fields.unnamed.iter().enumerate().map(|(i, f)| {
                    let name = Ident::new(&format!("field{}", i), Span::call_site());

                    quote_spanned! {f.span()=>
                        #name,
                    }
                });

                quote! {
                    #(#declare_fields)*

                    return Ok((
                        #name (
                            #(#field_constructors)*
                        ),
                        byte_stream
                    ));

                }
            }
            Fields::Unit => {
                quote!(
                    return Ok((
                        #name,
                        byte_stream
                    ));
                )
            }
        },
        Data::Enum(_) | Data::Union(_) => unimplemented!(),
    }
}
