use proc_macro2::{Ident, Span, TokenStream};
use quote::{ToTokens, quote};

use crate::sanitize;
pub struct GlyphLookupEntry<'a> {
	pub glyph:    &'a str,
	pub group:    &'a str,
	pub subgroup: &'a str,
	pub name:     &'a str,
	pub is_toned: bool,
}

impl<'a> GlyphLookupEntry<'a> {
	pub fn new(
		glyph: &'a str,
		group: &'a str,
		subgroup: &'a str,
		name: &'a str,
		is_toned: bool,
	) -> Self {
		Self { glyph, group, subgroup, name, is_toned }
	}
}

impl<'a> ToTokens for GlyphLookupEntry<'a> {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		let glyph = &self.glyph;
		let group = Ident::new(&sanitize(&self.group.to_string()).to_lowercase(), Span::call_site());
		let subgroup =
			Ident::new(&sanitize(&self.subgroup.to_string()).to_lowercase(), Span::call_site());
		let name = Ident::new(&sanitize(&self.name.to_string()).to_uppercase(), Span::call_site());

		if self.is_toned {
			(quote! {
					#glyph => crate::EmojiEntry::Toned(&crate::#group::#subgroup::#name)
			})
			.to_tokens(tokens);
		} else {
			(quote! {
					#glyph => crate::EmojiEntry::Standard(&crate::#group::#subgroup::#name)
			})
			.to_tokens(tokens);
		}
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

		if self.is_toned {
			(quote! {
					#namestr => crate::EmojiEntry::Toned(&crate::#group::#subgroup::#name_ident)
			})
			.to_tokens(tokens);
		} else {
			(quote! {
					#namestr => crate::EmojiEntry::Standard(&crate::#group::#subgroup::#name_ident)
			})
			.to_tokens(tokens);
		}
	}
}
