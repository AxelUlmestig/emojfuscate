use proc_macro2::{Span, TokenStream};
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{parse_macro_input, Data, DeriveInput, Fields, Ident};

#[proc_macro_derive(Emojfuscate)]
pub fn derive_emojfuscate(raw_input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Parse the input tokens into a syntax tree.
    let input = parse_macro_input!(raw_input as DeriveInput);

    // Used in the quasi-quotation below as `#name`.
    let name = input.ident;

    let expanded = match input.data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => {
                let iterator_names = fields.named.iter().enumerate().map(|(i, f)| {
                    let ident = Ident::new(&format!("I{}", i), Span::call_site());
                    quote_spanned! {f.span()=>#ident}
                });

                let chained_fields = fields
                    .named
                    .iter()
                    .map(|f| {
                        let name = &f.ident;
                        quote_spanned! {f.span()=>self.#name.emojfuscate_stream()}
                    })
                    .reduce(|prev, f| {
                        quote_spanned! {f.span()=>#prev.chain_emoji_bytes(#f)}
                    });

                let field_types = fields.named.iter().zip(iterator_names.clone()).map(
                    |(f, iterator_type_name)| {
                        let field_type = &f.ty;

                        quote_spanned! {f.span()=>
                            #field_type: Emojfuscate<#iterator_type_name>,
                            #iterator_type_name: Iterator<Item = ByteOrBreak>,
                        }
                    },
                );

                let iterator_chain = iterator_names.clone().reduce(|chain, element| {
                    quote! {Chain<#chain, #element>}
                });

                /*
                For a struct with named fields, e.g.

                struct Person {
                    age: u8,
                    name: String,
                    is_cool: bool
                }

                It should generate code that looks like this:

                impl<I1, I2, I3> Emojfuscate<Chain<Chain<I1, I2>, I3>> for Person
                where
                    u8: Emojfuscate<I1>,
                    String: Emojfuscate<I2>,
                    bool: Emojfuscate<I3>,
                    I1: Iterator<Item = ByteOrBreak>,
                    I2: Iterator<Item = ByteOrBreak>,
                    I3: Iterator<Item = ByteOrBreak>,
                {
                    fn emojfuscate_stream(self) -> EncodeBytesAsEmoji<Chain<I1, I2>> {
                        return self.age
                            .emojfuscate_stream()
                            .chain_emoji_bytes(self.name.emojfuscate_stream())
                            .chain_emoji_bytes(self.is_cool.emojfuscate_stream());
                    }
                }
                */

                quote! {
                    impl<#(#iterator_names),*> Emojfuscate<#iterator_chain> for #name
                    where
                        #(#field_types)*
                    {
                        fn emojfuscate_stream(self) -> EncodeBytesAsEmoji<#iterator_chain> {
                            #chained_fields
                        }
                    }
                }
            }
            _ => panic!("not implemented"),
        },
        _ => panic!("not implemented"),
    };

    /*
    let expanded = quote! {
        impl<I> Emojfuscate<I> for #name
        where
            I: Iterator<Item = ByteOrBreak>,
        {
            fn emojfuscate_stream(self) -> EncodeBytesAsEmoji<I> {
                panic!("not implemented");
            }
        }
    };
    */

    // Hand the output tokens back to the compiler.
    return proc_macro::TokenStream::from(expanded);
}

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
