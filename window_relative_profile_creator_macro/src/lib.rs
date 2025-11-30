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
		syn::parse_quote!(pub services:window_relative_system::WindowRelativeProfileServiceSet),
		syn::parse_quote!(pub handlers:window_relative_system::WindowRelativeProfileHandlerSet<Self>)
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
		impl window_relative_system::WindowRelativeProfile for #struct_name {
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
			#[inline]
			fn run_handlers(&mut self, window:&window_relative_system::WindowController, event_name:&str) -> Result<(), Box<dyn std::error::Error>> {
				let handlers = self.handlers().handlers_cloned();
				let concrete_self:&mut #struct_name = self.as_any_mut().downcast_mut::<#struct_name>().expect("Type mismatch in run_handlers");
				for handler in handlers {
					handler.run(concrete_self, window, event_name)?;
				}
				Ok(())
			}
		}
		impl window_relative_system::WindowRelativeProfileSized for #struct_name {
			fn handlers(&self) -> &window_relative_system::WindowRelativeProfileHandlerSet<Self> { &self.handlers }
			fn handlers_mut(&mut self) -> &mut window_relative_system::WindowRelativeProfileHandlerSet<Self> { &mut self.handlers }
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
						services: window_relative_system::WindowRelativeProfileServiceSet::new(),
						handlers: window_relative_system::WindowRelativeProfileHandlerSet::new()
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