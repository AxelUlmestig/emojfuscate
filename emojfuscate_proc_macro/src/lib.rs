use proc_macro2::{Span, TokenStream};
use quote::{quote, quote_spanned};
use std::iter::once;
use syn::spanned::Spanned;
use syn::{parse_macro_input, Data, DeriveInput, Fields, GenericParam, Ident};

#[proc_macro_derive(Emojfuscate)]
pub fn derive_emojfuscate(raw_input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(raw_input as DeriveInput);
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

                let generics = input
                    .generics
                    .params
                    .iter()
                    .filter_map(|p| match p {
                        GenericParam::Type(type_param) => {
                            let ident = &type_param.ident;
                            Some(quote! {#ident})
                        }
                        _ => None,
                    })
                    .chain(iterator_names);

                let (_, ty_generics, _) = input.generics.split_for_impl();

                /*
                // For a struct with named fields, e.g.

                struct Person<A> {
                    age: u8,
                    name: String,
                    luggage: A
                }

                // It should generate code that looks like this:

                impl<I1, I2, I3, A> Emojfuscate<Chain<Chain<I1, I2>, I3>> for Person<A>
                where
                    u8: Emojfuscate<I1>,
                    String: Emojfuscate<I2>,
                    A: Emojfuscate<I3>,
                    I1: Iterator<Item = ByteOrBreak>,
                    I2: Iterator<Item = ByteOrBreak>,
                    I3: Iterator<Item = ByteOrBreak>,
                {
                    fn emojfuscate_stream(self) -> EncodeBytesAsEmoji<Chain<Chain<I1, I2>, I3>> {
                        return self.age
                            .emojfuscate_stream()
                            .chain_emoji_bytes(self.name.emojfuscate_stream())
                            .chain_emoji_bytes(self.luggage.emojfuscate_stream());
                    }
                }
                */

                quote! {
                    impl<#(#generics),*> Emojfuscate<#iterator_chain> for #name #ty_generics
                    where
                        #(#field_types)*
                    {
                        fn emojfuscate_stream(self) -> EncodeBytesAsEmoji<#iterator_chain> {
                            #chained_fields
                        }
                    }
                }
            }
            Fields::Unnamed(ref fields) => {
                let iterator_names = fields.unnamed.iter().enumerate().map(|(i, f)| {
                    let ident = Ident::new(&format!("I{}", i), Span::call_site());
                    quote_spanned! {f.span()=>#ident}
                });

                let fields_with_names = fields.unnamed.iter().enumerate().map(|(i, f)| {
                    let name = Ident::new(&format!("field{}", i), Span::call_site());
                    quote_spanned! {f.span()=>#name}
                });

                let chained_fields = fields_with_names
                    .clone()
                    .map(|field_name| {
                        quote_spanned! {field_name.span()=>#field_name.emojfuscate_stream()}
                    })
                    .reduce(|prev, f| {
                        quote_spanned! {f.span()=>#prev.chain_emoji_bytes(#f)}
                    });

                let field_types = fields.unnamed.iter().zip(iterator_names.clone()).map(
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

                let generics = input
                    .generics
                    .params
                    .iter()
                    .filter_map(|p| match p {
                        GenericParam::Type(type_param) => {
                            let ident = &type_param.ident;
                            Some(quote! {#ident})
                        }
                        _ => None,
                    })
                    .chain(iterator_names);

                let (_, ty_generics, _) = input.generics.split_for_impl();

                /*
                // For a tuple struct, e.g.
                struct Person<A>(u8, String, A);

                // It should generate code that looks like this:

                impl<I1, I2, I3, A> Emojfuscate<Chain<Chain<I1, I2>, I3>> for Person<A>
                where
                    u8: Emojfuscate<I1>,
                    String: Emojfuscate<I2>,
                    A: Emojfuscate<I3>,
                    I1: Iterator<Item = ByteOrBreak>,
                    I2: Iterator<Item = ByteOrBreak>,
                    I3: Iterator<Item = ByteOrBreak>,
                {
                    fn emojfuscate_stream(self) -> EncodeBytesAsEmoji<Chain<Chain<I1, I2>, I3>> {
                        let Person(field0, field1, field2) = self;

                        return field0
                            .emojfuscate_stream()
                            .chain_emoji_bytes(field1.emojfuscate_stream())
                            .chain_emoji_bytes(field2.emojfuscate_stream());
                    }
                }
                */

                quote! {
                    impl<#(#generics),*> Emojfuscate<#iterator_chain> for #name #ty_generics
                    where
                        #(#field_types)*
                    {
                        fn emojfuscate_stream(self) -> EncodeBytesAsEmoji<#iterator_chain> {
                            let #name (#(#fields_with_names),*) = self;

                            #chained_fields
                        }
                    }
                }
            }
            Fields::Unit => {
                let generics = input
                    .generics
                    .params
                    .iter()
                    .filter_map(|p| match p {
                        GenericParam::Type(type_param) => {
                            let ident = &type_param.ident;
                            Some(quote! {#ident})
                        }
                        _ => None,
                    })
                    .chain(once(quote! {I}));

                let (_, ty_generics, _) = input.generics.split_for_impl();

                quote! {
                    impl<#(#generics),*> Emojfuscate<I> for #name #ty_generics
                    where
                        (): Emojfuscate<I>,
                        I: Iterator<Item = ByteOrBreak>,
                    {
                        fn emojfuscate_stream(self) -> EncodeBytesAsEmoji<I> {
                            ().emojfuscate_stream()
                        }
                    }
                }
            }
        },
        _ => panic!("not implemented"),
    };

    return proc_macro::TokenStream::from(expanded);
}

#[proc_macro_derive(ConstructFromEmoji)]
pub fn derive_construct_from_emoji(raw_input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(raw_input as DeriveInput);
    let name = input.ident;

    let (_, ty_generics, _) = input.generics.split_for_impl();

    let demojfuscated_fields = demojfuscate_fields(&input.data, &name);

    // <I, ...> where ... are any generics from the type implementing ConstructFromEmoji
    let generics = input
        .generics
        .params
        .iter()
        .filter_map(|p| match p {
            GenericParam::Type(type_param) => {
                let ident = &type_param.ident;
                Some(quote! {#ident})
            }
            _ => None,
        })
        .chain(once(quote! {I}));

    let implementations = input.generics.params.iter().filter_map(|p| match p {
        GenericParam::Type(type_param) => {
            let ident = &type_param.ident;
            Some(quote! {#ident: ConstructFromEmoji<#ident, I>,
            })
        }
        _ => None,
    });

    /*
    // For a struct with named fields, e.g.

    struct Person {
        age: u8,
        name: String,
        luggage: A
    }

    // It should generate code that looks like this:

    impl <A, I> ConstructFromEmoji <Person<A>, I> for Person <A>
    where
        I: Iterator <Item = u8>,
        A: ConstructFromEmoji <A, I>,
    {
        fn construct_from_emoji(mut byte_stream : DecodeEmojiToBytes <I>)
            -> Result <(Person<A>, DecodeEmojiToBytes <I>), FromEmojiError>
        {
            let age = match u8::construct_from_emoji(byte_stream)
                {
                    Err(err) => return Err(err),
                    Ok((result, new_byte_stream)) => {
                        byte_stream = new_byte_stream;
                        result
                    }
                };

            let name = match String::construct_from_emoji(byte_stream)
                {
                    Err(err) => return Err(err),
                    Ok((result, new_byte_stream)) => {
                        byte_stream = new_byte_stream;
                        result
                    }
                };

            let luggage = match A::construct_from_emoji(byte_stream)
                {
                    Err(err) => return Err(err),
                    Ok((result, new_byte_stream)) => {
                        byte_stream = new_byte_stream;
                        result
                    }
                };

            return Ok((Person { age : age, name : name, luggage : luggage, }, byte_stream));
        }
    }
    */

    let expanded = quote! {
        impl<#(#generics),*> ConstructFromEmoji<#name #ty_generics, I> for #name #ty_generics
        where
            I: Iterator<Item = u8>,
            #(#implementations)*
        {
            fn construct_from_emoji(
                mut byte_stream: DecodeEmojiToBytes<I>,
            ) -> Result<(#name #ty_generics, DecodeEmojiToBytes<I>), FromEmojiError> {
                #demojfuscated_fields
            }
        }
    };

    return proc_macro::TokenStream::from(expanded);
}

fn demojfuscate_fields(data: &Data, type_name: &Ident) -> TokenStream {
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
                        #type_name {
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
                        #name
                    }
                });

                quote! {
                    #(#declare_fields)*

                    return Ok((
                        #type_name (
                            #(#field_constructors),*
                        ),
                        byte_stream
                    ));

                }
            }
            Fields::Unit => {
                quote!(
                    return Ok((
                        #type_name,
                        byte_stream
                    ));
                )
            }
        },
        Data::Enum(_) | Data::Union(_) => unimplemented!(),
    }
}
