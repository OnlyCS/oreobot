use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

struct SelectMenuMeta {
    idents: Vec<syn::Ident>,
    labels: Vec<String>,
    values: Vec<String>,
    types: Vec<String>,
}

fn parse_meta(
    variants: syn::punctuated::Punctuated<syn::Variant, syn::token::Comma>,
	enum_ident: &syn::Ident,
) -> SelectMenuMeta {
    let mut idents = vec![];
    let mut labels = vec![];
    let mut values = vec![];
    let mut types = vec![];

    for variant in &variants {
        let attrs = &variant.attrs;

        let mut label = None;
        let mut ty_str = None;
        let ident = &variant.ident;
        let value = format!("oreo_selectoption_{}_{}", enum_ident.to_string(), ident.to_string());

        for thisattr in attrs {
			match &thisattr.meta { 
				syn::Meta::NameValue(
					syn::MetaNameValue { 
						value: syn::Expr::Lit(
							syn::ExprLit { 
								lit: syn::Lit::Str(str_lit), 
								.. 
							}
						),
						path,
						.. 
					}
				) => {
					let str_value = str_lit.value();
					let name = path.get_ident();
		
					let Some(name) = name else {
						continue;
					};
		
					match name.to_string().as_str() {
						"label" => label = Some(str_value),
						"ty" => ty_str = Some(str_value),
						_ => continue,
					}
				}, 
				_ => continue,
			};
        }

        idents.push(ident.clone());
        labels.push(label.unwrap_or(value.clone()));
        values.push(value);
        types.push(ty_str.unwrap());
    }

    SelectMenuMeta {
        idents,
        labels,
        values,
        types,
    }
}


#[proc_macro_attribute]
pub fn updatable(args: TokenStream, input: TokenStream) -> TokenStream {
	let input = parse_macro_input!(input as DeriveInput);
	let args_ts = proc_macro2::TokenStream::from(args);

	let data = &input.data;
	let ident = &input.ident;
	let mut rewritten_idents = vec![];
	let mut rewritten_types = vec![];

	let syn::Data::Struct(s) = data else {
		panic!("updatable can only be derived for structs");
	};

	let fields = &s.fields;

	for field in fields {
		let ty = &field.ty;
		let ident = field.ident.as_ref().unwrap().to_string();
		let ident_camel = ident.split('_').map(|spl| spl.chars().enumerate().map(|(idx, c)| if idx == 0 { c.to_ascii_uppercase() } else { c }).collect::<String>()).collect::<String>();

		let new_ident = syn::Ident::new(&ident_camel, field.ident.as_ref().unwrap().span());

		rewritten_idents.push(new_ident.clone());
		rewritten_types.push(ty.clone());

	}

	let new_ident = syn::Ident::new(&format!("{}Update", ident.to_string()), ident.span());

	quote! {
		#args_ts
		#input

		pub enum #new_ident {
			#(#rewritten_idents (#rewritten_types)),*
		}
	}.into()
}

#[proc_macro_derive(SelectMenuOptions, attributes(label, ty))]
pub fn derive(input: TokenStream) -> TokenStream {
    let DeriveInput { data, ident, .. } = parse_macro_input!(input as DeriveInput);

    match data {
        syn::Data::Enum(enu) => {
            let SelectMenuMeta {
                idents,
                labels,
                values,
                types,
            } = parse_meta(enu.variants, &ident);

            quote! {
                impl #ident {
                    pub fn options() -> Vec<serenity::CreateSelectMenuOption> {
                        let labels = vec![#(#labels),*];
                        let values = vec![#(#values),*];

                        labels.iter().enumerate().map(|(idx, n)| {
                            let mut option = serenity::CreateSelectMenuOption::default();
                            option.label(n.to_string());
                            option.value(&values[idx]);
							option
                        }).collect_vec()
                    }

					fn __type_str_of(&self) -> &'static str {
						let types = vec![#(#types),*];
						let idents = vec![#(#ident::#idents),*];
						let idx = idents.iter().position(|n| n == self).unwrap();
						types[idx]
					}

					pub fn type_row<S>(&self, custom_id: S) -> serenity::CreateActionRow
					where
						S: ToString
					{
						let mut row = serenity::CreateActionRow::default();

						match self.__type_str_of() {
							"String" => {
								let mut textbox = serenity::CreateInputText::default();

								textbox.custom_id(custom_id);
								textbox.label("Enter a value");
								textbox.required(true);
								textbox.placeholder("Lorem ipsum dolor sit amet");

								row.add_input_text(textbox);
							},
							"bool" => {
								let mut select = serenity::CreateSelectMenu::default();

								select.custom_id(custom_id);
								select.placeholder("Select an option");
								select.options(|options| {
									let mut option_true = serenity::CreateSelectMenuOption::default();
									let mut option_false = serenity::CreateSelectMenuOption::default();

									option_true.label("Yes");
									option_true.value("true");

									option_false.label("No");
									option_false.value("false");

									options.add_option(option_true);
									options.add_option(option_false);

									options
								});

								row.add_select_menu(select);
							},
							"i32" => {
								let mut textbox = serenity::CreateInputText::default();

								textbox.custom_id(custom_id);
								textbox.label("Enter any whole number");
								textbox.required(true);
								textbox.placeholder("12");

								row.add_input_text(textbox);
							},
							"u32" => {
								let mut textbox = serenity::CreateInputText::default();

								textbox.custom_id(custom_id);
								textbox.label("Enter any whole. positive number");
								textbox.required(true);
								textbox.placeholder("18");

								row.add_input_text(textbox);
							},
							"f64" => {
								let mut textbox = serenity::CreateInputText::default();

								textbox.custom_id(custom_id);
								textbox.label("Enter any decimal number");
								textbox.required(true);
								textbox.placeholder("3.14");

								row.add_input_text(textbox);
							}
							_ => panic!("Invalid type")
						}

						row
					}

					pub fn parse_as_bool(&self, value: &str) -> Result<bool, AnyError> {
						let ty = self.__type_str_of();

						if ty != "bool" {
							bail!(anyhow!("Invalid type"));
						}

						match value {
							"true" => Ok(true),
							"false" => Ok(false),
							_ => bail!(anyhow!("Invalid value"))
						}
					}
					
					pub fn parse_as_i32(&self, value: &str) -> Result<i32, AnyError> {
						let ty = self.__type_str_of();

						if ty != "i32" {
							bail!(anyhow!("Invalid type"));
						}

						match value.parse::<i32>() {
							Ok(n) => Ok(n),
							Err(_) => bail!(anyhow!("Invalid value"))
						}
					}

					pub fn parse_as_u32(&self, value: &str) -> Result<u32, AnyError> {
						let ty = self.__type_str_of();

						if ty != "u32" {
							bail!(anyhow!("Invalid type"));
						}

						match value.parse::<u32>() {
							Ok(n) => Ok(n),
							Err(_) => bail!(anyhow!("Invalid value"))
						}
					}

					pub fn parse_as_f64(&self, value: &str) -> Result<f64, AnyError> {
						let ty = self.__type_str_of();

						if ty != "f64" {
							bail!(anyhow!("Invalid type"));
						}

						match value.parse::<f64>() {
							Ok(n) => Ok(n),
							Err(_) => bail!(anyhow!("Invalid value"))
						}
					}
				}
				
                impl std::str::FromStr for #ident {
                    type Err = AnyError;

                    fn from_str(from: &str) -> Result<Self, Self::Err> {
                        let values = vec![#(#values),*];
                        let idents = vec![#(#ident::#idents),*];

                        let idx = values.iter().position(|n| n.to_string() == from.to_string()).unwrap();
                        Ok(idents[idx])
                    }
                }
            }
            .into()
        }
        _ => panic!("SelectMenuOptions can only be derived for enums"),
    }
}
