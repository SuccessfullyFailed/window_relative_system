use syn::{ Data, DeriveInput, Field, Fields, Ident, parse_macro_input, token, punctuated::Punctuated };
use proc_macro_crate::{ FoundCrate, crate_name };
use quote::quote;



/// Allow usage of #[derive(WindowRelativeProfileEssentials)] to simplify the creation of a window-relative profile.
#[proc_macro_derive(WindowRelativeProfileEssentials)]
pub fn derive_profile(input:proc_macro::TokenStream) -> proc_macro::TokenStream {
	let input:DeriveInput = parse_macro_input!(input as DeriveInput);
	impl_profile(&input)
}



/// Implement the window-relative essential definitions for the given derive input.
fn impl_profile(input:&DeriveInput) -> proc_macro::TokenStream {
	let name:&Ident = &input.ident;
	let crate_ident:Ident = {
		match crate_name("window_relative_system") {
			Ok(FoundCrate::Itself) => Ident::new("crate", proc_macro2::Span::call_site()),
			_ => Ident::new("window_relative_system", proc_macro2::Span::call_site())
		}
	};

	// Get the named fields of the derived struct.
	let fields:&Punctuated<Field, token::Comma> = {
		if let Data::Struct(data) = &input.data {
			if let Fields::Named(fields) = &data.fields {
				&fields.named
			} else {
				return syn::Error::new_spanned(&input.ident, "WindowRelativeProfile requires named fields").to_compile_error().into()
			}
		} else {
			return syn::Error::new_spanned(&input.ident, "WindowRelativeProfile can only be derived for structs").to_compile_error().into();
		}
	};

	// Validate required fields exist.
	const REQUIRED_FIELDS:&[&str] = &["name", "process_name", "task_system", "status"];
	let field_names:Vec<String> = fields.iter().map(|field| field.ident.as_ref().unwrap().to_string()).collect();
	for required_field in REQUIRED_FIELDS {
		if !field_names.contains(&required_field.to_string()) {
			return syn::Error::new(proc_macro2::Span::call_site(), &format!("Missing field `{required_field}`")).to_compile_error().into();
		}
	}
	
	// Create and return implementation.
	let expanded:proc_macro2::TokenStream = quote! {
		impl #crate_ident::WindowRelativeProfileEssentials for #name {
			fn name(&self) -> &str { &self.name }
			fn process_name(&self) -> &str { &self.process_name }
			fn task_system(&self) -> &#crate_ident::TaskSystem { &self.task_system }
			fn task_system_mut(&mut self) -> &mut #crate_ident::TaskSystem { &mut self.task_system }
			fn status(&self) -> &#crate_ident::ProfileStatus { &self.status }
			fn status_mut(&mut self) -> &mut #crate_ident::ProfileStatus { &mut self.status }
		}
	};
	expanded.into()
}