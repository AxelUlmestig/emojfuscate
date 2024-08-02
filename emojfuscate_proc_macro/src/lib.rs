use proc_macro2::Span;
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
        Data::Enum(ref data) => {
            /*
            For an enum that looks like this:

            enum Animal {
                Cat{ likes_cuddles: bool, name: String},
                Dog(u32),
                Lizard
            }

            the generated code should look something like this:

            impl<IA, IB> Emojfuscate<Chain<Chain<Once<ByteOrBreak>, IA>, IB>> for Animal
            where
                Option<(bool, String)>: Emojfuscate<IA>,
                Option<u32>: Emojfuscate<IB>,
                IA: Iterator<Item = ByteOrBreak>,
                IB: Iterator<Item = ByteOrBreak>,
            {
                fn emojfuscate_stream(self) -> EncodeBytesAsEmoji<Chain<Chain<Once<ByteOrBreak>, IA>, IB>> {
                    match self {
                        Animal::Cat(b,s) => {
                            0u8
                                .emojfuscate_stream()
                                .chain_emoji_bytes(Some((b,s)).emojfuscate_stream())
                                .chain_emoji_bytes(None.emojfuscate_stream())
                        },
                        Animal::Dog(i) => {
                            1u8
                                .emojfuscate_stream()
                                .chain_emoji_bytes(None.emojfuscate_stream())
                                .chain_emoji_bytes(Some(i).emojfuscate_stream())
                        },
                        Animal::Lizard => {
                            2u8
                                .emojfuscate_stream()
                                .chain_emoji_bytes(None.emojfuscate_stream())
                                .chain_emoji_bytes(None.emojfuscate_stream())
                        }
                    }
                }
            }
            */

            let mut trait_constraints = data
                .variants
                .iter()
                .enumerate()
                .filter_map(|(variant_index, variant)| match variant.fields {
                    Fields::Named(ref fields) => {
                        let mut field_types = fields.named.iter().map(|f| &f.ty);
                        let iterator_name =
                            Ident::new(&format!("I{}", variant_index), Span::call_site());

                        Some(if field_types.len() == 1 {
                            let only_field = field_types.next().unwrap();

                            quote! {
                                Option<#only_field>: Emojfuscate<#iterator_name>,
                                #iterator_name: Iterator<Item = ByteOrBreak>
                            }
                        } else {
                            quote! {
                                Option<(#(#field_types),*)>: Emojfuscate<#iterator_name>,
                                #iterator_name: Iterator<Item = ByteOrBreak>
                            }
                        })
                    }
                    Fields::Unnamed(ref fields) => {
                        let mut field_types = fields.unnamed.iter().map(|f| &f.ty);
                        let iterator_name =
                            Ident::new(&format!("I{}", variant_index), Span::call_site());

                        Some(if field_types.len() == 1 {
                            let only_field = field_types.next().unwrap();

                            quote! {
                                Option<#only_field>: Emojfuscate<#iterator_name>,
                                #iterator_name: Iterator<Item = ByteOrBreak>
                            }
                        } else {
                            quote! {
                                Option<(#(#field_types),*)>: Emojfuscate<#iterator_name>,
                                #iterator_name: Iterator<Item = ByteOrBreak>
                            }
                        })
                    }
                    Fields::Unit => None,
                })
                .peekable();

            let function_body =
                data
                    .variants
                    .iter()
                    .enumerate()
                    .map(|(variant_index_usize, variant)| {
                        let variant_index = variant_index_usize as u8;

                        match variant.fields {
                            Fields::Named(ref fields) => {
                                let variant_name = &variant.ident;
                                let field_names = fields.named.iter()
                                        .map(|f| {
                                            let field_name = &f.ident;
                                            quote_spanned! {f.span()=>#field_name}
                                        });

                                let fields_to_emojfuscate =
                                    data.variants.iter().enumerate()
                                    .filter(|(_, v)| match v.fields {
                                        Fields::Unit => false,
                                        _ => true,
                                    }).map(|(i, _)| {
                                    if i as u8 == variant_index {
                                        let field_names2 = fields.named.iter()
                                                .map(|f| {
                                                    let field_name = &f.ident;
                                                    quote_spanned! {f.span()=>#field_name}
                                                });

                                        quote! {
                                            .chain_emoji_bytes(Some((#(#field_names2),*)).emojfuscate_stream())
                                        }
                                    } else {
                                        quote! {
                                            .chain_emoji_bytes(None.emojfuscate_stream())
                                        }
                                    }
                                });

                                quote! {
                                    #name::#variant_name{#(#field_names),*} => {
                                        #variant_index.emojfuscate_stream()
                                        #(#fields_to_emojfuscate)*
                                    }
                                }
                            }
                            Fields::Unnamed(ref fields) => {
                                let variant_name = &variant.ident;
                                let field_names = fields.unnamed.iter();

                                let fields_to_emojfuscate =
                                    data.variants.iter().enumerate()
                                    .filter(|(_, v)| match v.fields {
                                        Fields::Unit => false,
                                        _ => true,
                                    }).map(|(i, _)| {
                                    if i as u8 == variant_index {
                                        let field_names2 = fields.unnamed.iter();

                                        quote! {
                                            .chain_emoji_bytes(Some((#(#field_names2),*)).emojfuscate_stream())
                                        }
                                    } else {
                                        quote! {
                                            .chain_emoji_bytes(None.emojfuscate_stream())
                                        }
                                    }
                                });

                                quote! {
                                    #name::#variant_name(#(#field_names),*) => {
                                        #variant_index.emojfuscate_stream()
                                        #(#fields_to_emojfuscate)*
                                    }
                                }
                            },
                            Fields::Unit => {
                                let fields_to_emojfuscate =
                                    data.variants.iter().enumerate()
                                    .filter(|(_, v)| match v.fields {
                                        Fields::Unit => false,
                                        _ => true,
                                    }).map(|_| {
                                        quote! {
                                            .chain_emoji_bytes(None.emojfuscate_stream())
                                        }
                                });

                                let variant_name = &variant.ident;

                                quote! {
                                    #name::#variant_name => {
                                        #variant_index.emojfuscate_stream()
                                        #(#fields_to_emojfuscate)*
                                    }
                                }
                            }
                        }
                    });

            let iterator_names = data
                .variants
                .iter()
                .enumerate()
                .filter(|(_, v)| match v.fields {
                    Fields::Unit => false,
                    _ => true,
                })
                .map(|(i, _)| {
                    let ident = Ident::new(&format!("I{}", i), Span::call_site());
                    quote! {#ident}
                });

            let iterator_chain_type = once(quote! {Once<ByteOrBreak>})
                .chain(iterator_names.clone())
                .reduce(|chain, element| {
                    quote! {Chain<#chain, #element>}
                });

            let where_clause = match trait_constraints.peek() {
                Some(_) => {
                    quote! {
                        where
                            #(#trait_constraints),*
                    }
                }
                None => {
                    quote! {}
                }
            };

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

            quote! {
                impl<#(#generics),*> Emojfuscate<#iterator_chain_type> for #name #ty_generics
                #where_clause
                {
                    fn emojfuscate_stream(self) -> EncodeBytesAsEmoji<#iterator_chain_type> {
                        match self {
                            #(#function_body)*
                        }
                    }
                }
            }
        }
        Data::Union(_) => unimplemented!(),
    };

    return proc_macro::TokenStream::from(expanded);
}

#[proc_macro_derive(ConstructFromEmoji)]
pub fn derive_construct_from_emoji(raw_input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(raw_input as DeriveInput);
    let name = input.ident;

    let (_, ty_generics, _) = input.generics.split_for_impl();

    let demojfuscated_fields = match input.data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => {
                let declare_fields = fields.named.iter().map(|f| {
                    let field_name = &f.ident;
                    let field_type = &f.ty;
                    quote_spanned! {f.span()=>
                        let #field_name = match #field_type::construct_from_emoji(byte_stream) {
                            Err(err) => return Err(err),
                            Ok((result, new_byte_stream)) => {
                                byte_stream = new_byte_stream;
                                result
                            }
                        };

                    }
                });

                let field_constructors = fields.named.iter().map(|f| {
                    let field_name = &f.ident;

                    quote_spanned! {f.span()=>
                        #field_name: #field_name,
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
                    let field_name = Ident::new(&format!("field{}", i), Span::call_site());
                    let field_type = &f.ty;
                    quote_spanned! {f.span()=>
                        let #field_name = match #field_type::construct_from_emoji(byte_stream) {
                            Err(err) => return Err(err),
                            Ok((result, new_byte_stream)) => {
                                byte_stream = new_byte_stream;
                                result
                            }
                        };

                    }
                });

                let field_constructors = fields.unnamed.iter().enumerate().map(|(i, f)| {
                    let field_name = Ident::new(&format!("field{}", i), Span::call_site());

                    quote_spanned! {f.span()=>
                        #field_name
                    }
                });

                quote! {
                    #(#declare_fields)*

                    return Ok((
                        #name (
                            #(#field_constructors),*
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
        Data::Enum(ref data) => {
            let parsed_data = data.variants.iter().enumerate().filter_map(|(variant_index, variant)|
                match variant.fields {
                    Fields::Named(ref fields) => {
                        let mut field_types = fields.named.iter().map(|f| &f.ty);
                        let constructor_name =
                            Ident::new(&format!("constructor{}", variant_index), Span::call_site());

                        Some(if field_types.len() == 1 {
                            let only_field = field_types.next().unwrap();

                            quote! {
                                let #constructor_name =
                                    match Option::<#only_field>::construct_from_emoji(byte_stream) {
                                        Err(err) => return Err(err),
                                        Ok((x, new_byte_stream)) => {
                                            byte_stream = new_byte_stream;
                                            x
                                        }
                                    };
                            }
                        } else {
                            quote! {
                                let #constructor_name =
                                    match Option::<(#(#field_types),*)>::construct_from_emoji(byte_stream) {
                                        Err(err) => return Err(err),
                                        Ok((x, new_byte_stream)) => {
                                            byte_stream = new_byte_stream;
                                            x
                                        }
                                    };
                            }
                        })
                    }
                    Fields::Unnamed(ref fields) => {
                        let mut field_types = fields.unnamed.iter().map(|f| &f.ty);
                        let constructor_name =
                            Ident::new(&format!("constructor{}", variant_index), Span::call_site());

                        Some(if field_types.len() == 1 {
                            let only_field = field_types.next().unwrap();

                            quote! {
                                let #constructor_name =
                                    match Option::<#only_field>::construct_from_emoji(byte_stream) {
                                        Err(err) => return Err(err),
                                        Ok((x, new_byte_stream)) => {
                                            byte_stream = new_byte_stream;
                                            x
                                        }
                                    };
                            }
                        } else {
                            quote! {
                                let #constructor_name =
                                    match Option::<(#(#field_types),*)>::construct_from_emoji(byte_stream) {
                                        Err(err) => return Err(err),
                                        Ok((x, new_byte_stream)) => {
                                            byte_stream = new_byte_stream;
                                            x
                                        }
                                    };
                            }
                        })
                    }
                    Fields::Unit => None,
                },
            );

            let constructors = data
                .variants
                .iter()
                .enumerate()
                .map(|(variant_index, variant)| {
                    let variant_name = &variant.ident;

                    match variant.fields {
                        Fields::Named(ref fields) => {
                            let field_names = fields.named.iter().map(|f| &f.ident);
                            let index = variant_index as u8;

                            let pattern_matching_data =
                                data
                                    .variants
                                    .iter()
                                    .enumerate()
                                    .filter(|(_, v)| match v.fields {
                                        Fields::Unit => false,
                                        _ => true,
                                    })
                                    .map(|(i, _)| {
                                        if i as u8 == variant_index as u8 {
                                            let field_names2 = fields.named.iter().map(|f| &f.ident);

                                            quote! {
                                                Some((#(#field_names2),*))
                                            }
                                        } else {
                                            quote! {None}
                                        }
                                    });


                            quote! {
                                (#index #(, #pattern_matching_data)*) => Ok((#name::#variant_name{#(#field_names),*}, byte_stream))
                            }
                        }
                        Fields::Unnamed(ref fields) => {
                            let field_names = fields.unnamed.iter().enumerate().map(|(field_index, _)| { Ident::new(&format!("x{}", field_index), Span::call_site()) });
                            let index = variant_index as u8;

                            let pattern_matching_data =
                                data
                                    .variants
                                    .iter()
                                    .enumerate()
                                    .filter(|(_, v)| match v.fields {
                                        Fields::Unit => false,
                                        _ => true,
                                    })
                                    .map(|(i, _)| {
                                        if i as u8 == variant_index as u8 {
                                            let field_names2 = fields.unnamed.iter().enumerate().map(|(field_index, _)| { Ident::new(&format!("x{}", field_index), Span::call_site()) });

                                            quote! {
                                                Some((#(#field_names2),*))
                                            }
                                        } else {
                                            quote! {None}
                                        }
                                    });


                            quote! {
                                (#index #(, #pattern_matching_data)*) => Ok((#name::#variant_name(#(#field_names),*), byte_stream))
                            }
                        }
                        Fields::Unit => {
                            let index = variant_index as u8;

                            let pattern_matching_data =
                                data
                                    .variants
                                    .iter()
                                    .enumerate()
                                    .filter(|(_, v)| match v.fields {
                                        Fields::Unit => false,
                                        _ => true,
                                    })
                                    .map(|_| { quote! {None} });

                            quote! {
                                (#index #(, #pattern_matching_data)*) => Ok((#name::#variant_name, byte_stream))
                            }
                        }
                    }
                });

            let parsed_data_variable_names = data.variants.iter().enumerate().filter_map(
                |(variant_index, variant)| match variant.fields {
                    Fields::Named(_) | Fields::Unnamed(_) => Some(Ident::new(
                        &format!("constructor{}", variant_index),
                        Span::call_site(),
                    )),
                    Fields::Unit => None,
                },
            );

            quote! {
                let constructor_discriminator =
                    match u8::construct_from_emoji(byte_stream) {
                        Err(err) => return Err(err),
                        Ok((n, new_byte_stream)) => {
                            byte_stream = new_byte_stream;
                            n
                        }
                    };

                #(#parsed_data)*

                match (constructor_discriminator #(, #parsed_data_variable_names)*) {
                    #(#constructors),*,
                    _ => Err(FromEmojiError::UnexpectedInput("Constructor choice and data don't agree when demojfuscating #name".to_string()))
                }
            }
        }
        Data::Union(_) => unimplemented!(),
    };

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
