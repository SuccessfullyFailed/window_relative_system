use proc_macro::TokenStream;
use quote::quote;
use syn::{ Field, FieldsNamed, Ident, ItemStruct, parse_macro_input };



#[proc_macro_attribute]
pub fn window_relative_profile(attr:TokenStream, item:TokenStream) -> TokenStream {
	let mut ast:ItemStruct = parse_macro_input!(item as ItemStruct);
	let struct_name:&Ident = &ast.ident;
	let arg_names:Vec<String> = attr.into_iter().filter_map(|tt| if let proc_macro::TokenTree::Ident(ident) = tt { Some(ident.to_string()) } else { None }).collect();

	// ---- Create the injected fields ----
	let injected_fields: Vec<Field> = vec![
		syn::parse_quote!(pub properties:window_relative_system::WindowRelativeProfileProperties),
		syn::parse_quote!(pub task_system:window_relative_system::TaskSystem),
		syn::parse_quote!(pub services:window_relative_system::WindowRelativeProfileServiceSet<Self>)
	];

	// ---- Insert into the struct ----
	if let syn::Fields::Named(FieldsNamed { ref mut named, .. }) = ast.fields {
		for f in injected_fields {
			named.push(f);
		}
	} else {
		return syn::Error::new_spanned(&ast, "window_relative_profile only works on a struct with named fields").to_compile_error().into();
	}

	// ---- Implement the trait ----
	let trait_impl:proc_macro2::TokenStream = quote! {
		use window_relative_system::{ WindowRelativeProfileHandlerList as _ };
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
			fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
			#[inline]
			fn trigger_service_event_handlers_with_window(&mut self, event_name:&str, window:&window_relative_system::WindowController) -> Result<(), Box<dyn std::error::Error>> {
				let services = self.services().clone();
				let concrete_self:&mut #struct_name = self.as_any_mut().downcast_mut::<#struct_name>().expect("Type mismatch in run_handlers");
				let expired = services.run(concrete_self, window, event_name)?;
				for index in expired.into_iter().rev() {
					self.services.remove(index);
				}
				Ok(())
			}
		}
		impl window_relative_system::WindowRelativeProfileHandlerList for #struct_name {
			fn services(&mut self) -> &mut window_relative_system::WindowRelativeProfileServiceSet<Self> {
				&mut self.services
			}
		}
	};

	// Implement Default.
	let new_impl:proc_macro2::TokenStream = match arg_names.as_slice() {
		[id, title, process_name] => quote! {
			impl Default for TestCore {
				fn default() -> Self {
					TestCore {
						properties: window_relative_system::WindowRelativeProfileProperties::new(#id, #title, #process_name),
						task_system: window_relative_system::TaskSystem::new(),
						services: window_relative_system::WindowRelativeProfileServiceSet::new()
					}
				}
			}
		},
		_ => quote! { }
	};

	// ---- Output modified struct + trait impl ----
	TokenStream::from(quote! {
		#ast
		#trait_impl
		#new_impl
	})
}