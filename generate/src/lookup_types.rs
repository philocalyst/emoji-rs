use proc_macro2::{Ident, Span, TokenStream};
use quote::{ToTokens, quote};

use crate::sanitize;
pub struct GlyphLookupEntry {
	pub glyph:    String,
	pub group:    String,
	pub subgroup: String,
	pub name:     String,
	pub is_toned: bool,
}

impl GlyphLookupEntry {
	pub fn new(glyph: String, group: String, subgroup: String, name: String, is_toned: bool) -> Self {
		Self { glyph, group, subgroup, name, is_toned }
	}

	pub fn to_array_element_tokens(&self) -> proc_macro2::TokenStream {
		let group = Ident::new(&sanitize(&self.group).to_lowercase(), Span::call_site());
		let subgroup = Ident::new(&sanitize(&self.subgroup).to_lowercase(), Span::call_site());
		let name = Ident::new(&sanitize(&self.name).to_uppercase(), Span::call_site());

		quote! { &crate::#group::#subgroup::#name }
	}
}

pub fn to_array_tokens(entries: &[GlyphLookupEntry]) -> proc_macro2::TokenStream {
	let count = entries.len();
	let elements = entries.iter().map(|e| e.to_array_element_tokens());

	quote! {
			/// All emoji entries defined in the system.
			pub const ALL_EMOJI: [&crate::Emoji; #count] = [
					#(#elements),*
			];
	}
}

impl ToTokens for GlyphLookupEntry {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		let glyph = &self.glyph;
		let group = Ident::new(&sanitize(&self.group.to_string()).to_lowercase(), Span::call_site());
		let subgroup =
			Ident::new(&sanitize(&self.subgroup.to_string()).to_lowercase(), Span::call_site());
		let name = Ident::new(&sanitize(&self.name.to_string()).to_uppercase(), Span::call_site());

		(quote! {
				#glyph => &crate::#group::#subgroup::#name
		})
		.to_tokens(tokens);
	}
}

pub struct NameLookupEntry<'a> {
	pub group:    &'a str,
	pub subgroup: &'a str,
	pub name:     &'a str,
	pub is_toned: bool,
}

impl<'a> NameLookupEntry<'a> {
	pub fn new(group: &'a str, subgroup: &'a str, name: &'a str, is_toned: bool) -> Self {
		Self { group, subgroup, name, is_toned }
	}
}

impl<'a> ToTokens for NameLookupEntry<'a> {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		let group = Ident::new(&sanitize(&self.group.to_string()).to_lowercase(), Span::call_site());
		let subgroup =
			Ident::new(&sanitize(&self.subgroup.to_string()).to_lowercase(), Span::call_site());
		let name_ident = Ident::new(
			// Renamed to avoid clash with `name` field
			&sanitize(&self.name.to_string()).to_uppercase(),
			Span::call_site(),
		);
		let namestr = &self.name;

		(quote! {
				#namestr => &crate::#group::#subgroup::#name_ident
		})
		.to_tokens(tokens);
	}
}
