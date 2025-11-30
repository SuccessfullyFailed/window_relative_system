use syn::{ Expr, Field, FieldsNamed, Ident, ItemStruct, Lit, parse_macro_input, punctuated::Punctuated, Token, ExprLit };
use proc_macro::TokenStream;
use quote::quote;



#[proc_macro_attribute]
pub fn window_relative_profile(attr:TokenStream, item:TokenStream) -> TokenStream {
	let mut ast:ItemStruct = parse_macro_input!(item as ItemStruct);
	let struct_name:&Ident = &ast.ident;
	let args:Punctuated<Expr, syn::token::Comma> = parse_macro_input!(attr with Punctuated::<Expr, Token![,]>::parse_terminated);
	let arg_names:Vec<String> = args.into_iter().filter_map(|expr| {
		match expr {
			Expr::Lit(ExprLit { lit: Lit::Str(s), .. }) => Some(s.value()),
			Expr::Path(path) => path.path.get_ident().map(|i| i.to_string()),
			_ => None
		}
	}).collect();

	// Create extra fields.
	let injected_fields:Vec<Field> = vec![
		syn::parse_quote!(pub properties:window_relative_system::WindowRelativeProfileProperties),
		syn::parse_quote!(pub task_system:window_relative_system::TaskSystem),
		syn::parse_quote!(pub services:window_relative_system::WindowRelativeProfileServiceSet)
	];

	// Insert extra fields into the struct.
	if let syn::Fields::Named(FieldsNamed { ref mut named, .. }) = ast.fields {
		for f in injected_fields {
			named.push(f);
		}
	} else {
		return syn::Error::new_spanned(&ast, "window_relative_profile macro only works on a struct with named fields").to_compile_error().into();
	}

	// Implement traits for struct.
	let trait_impl:proc_macro2::TokenStream = quote! {
		use window_relative_system::{ WindowRelativeProfileSized as _ };
		impl window_relative_system::WindowRelativeProfileCore for #struct_name {
			#[inline]
			fn properties(&self) -> &window_relative_system::WindowRelativeProfileProperties { &self.properties }
			#[inline]
			fn properties_mut(&mut self) -> &mut window_relative_system::WindowRelativeProfileProperties { &mut self.properties }
			#[inline]
			fn task_system(&mut self) -> &window_relative_system::TaskSystem { &self.task_system }
			#[inline]
			fn task_system_mut(&mut self) -> &mut window_relative_system::TaskSystem { &mut self.task_system }
			#[inline]
			fn services(&mut self) -> &mut window_relative_system::WindowRelativeProfileServiceSet { &mut self.services }
			#[inline]
			fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
		}
	};

	// Implement Default implementation.
	let new_impl:proc_macro2::TokenStream = match arg_names.as_slice() {
		[id, title, process_name] => quote! {
			impl Default for #struct_name {
				fn default() -> Self {
					#struct_name {
						properties: window_relative_system::WindowRelativeProfileProperties::new(#id, #title, #process_name),
						task_system: window_relative_system::TaskSystem::new(),
						services: window_relative_system::WindowRelativeProfileServiceSet::new()
					}
				}
			}
		},
		_ => quote! { }
	};

	// Combine and return created tokens.
	TokenStream::from(quote! {
		#ast
		#trait_impl
		#new_impl
	})
}