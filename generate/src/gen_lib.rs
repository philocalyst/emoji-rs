use std::{collections::HashMap, fs::File, io::{Read, Write}, path::PathBuf, process::Command};

use heck::ToUpperCamelCase;
use proc_macro2::TokenStream;
use quote::quote;

use crate::{group_subgroup::Group, sanitize};

pub fn dump(
	groups: &Vec<Group>,
	group_map: &HashMap<u32, String>,
	subgroup_map: &HashMap<u32, String>,
	annotation_langs: &Vec<&str>,
	version: f32,
	date: String,
) {
	let mut fs = String::new();
	let mut f = File::open("generate/src/library_header.rs").unwrap();
	f.read_to_string(&mut fs).unwrap();
	let ts: TokenStream = fs.parse().unwrap();

	// Generate Enums
	let mut groups_vec: Vec<_> = group_map.iter().collect();
	groups_vec.sort_by_key(|(id, _)| *id);

	let mut group_variants = vec![];
	let mut group_display_arms = vec![];
	let mut group_iter_variants = vec![];

	for (_, key) in groups_vec {
		let ident = proc_macro2::Ident::new(&key.to_upper_camel_case(), proc_macro2::Span::call_site());
		group_variants.push(quote! { #ident });
		group_display_arms.push(quote! { Group::#ident => #key });
		group_iter_variants.push(quote! { Group::#ident });
	}

	let mut subgroups_vec: Vec<_> = subgroup_map.iter().collect();
	subgroups_vec.sort_by_key(|(id, _)| *id);

	let mut subgroup_variants = vec![];
	let mut subgroup_display_arms = vec![];
	let mut subgroup_iter_variants = vec![];

	for (_, key) in subgroups_vec {
		let ident = proc_macro2::Ident::new(&key.to_upper_camel_case(), proc_macro2::Span::call_site());
		subgroup_variants.push(quote! { #ident });
		subgroup_display_arms.push(quote! { Subgroup::#ident => #key });
		subgroup_iter_variants.push(quote! { Subgroup::#ident });
	}

	let dump = quote! {
			#ts

			#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
			pub enum Group {
					#(#group_variants),*
			}

			impl std::fmt::Display for Group {
				fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
					match self {
						#(#group_display_arms),*
					}.fmt(f)
				}
			}

			impl Group {
				pub fn iter() -> impl Iterator<Item = Group> {
					[
						#(#group_iter_variants),*
					].iter().copied()
				}
			}

			#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
			pub enum Subgroup {
					#(#subgroup_variants),*
			}

			impl std::fmt::Display for Subgroup {
				fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
					match self {
						#(#subgroup_display_arms),*
					}.fmt(f)
				}
			}

			impl Subgroup {
				pub fn iter() -> impl Iterator<Item = Subgroup> {
					[
						#(#subgroup_iter_variants),*
					].iter().copied()
				}
			}

			/// All annotation languages
			pub const ANNOTATION_LANGS_TOTAL: &'static [&'static str] = &[
					#(#annotation_langs),*
			];
			pub const UNICODE_VERSION: f32 = #version;
			pub const UNICODE_RELEASE_TIME: &'static str = #date;
			#(#groups)*
	};

	let path = "emoji/src/lib.rs";
	let pb: PathBuf = path.into();
	File::create(pb).unwrap().write_all(format!("{}", dump).as_bytes()).unwrap();
	Command::new("rustfmt").arg(path).output().expect("Failed to execute command");

	// ... existing subgroup file generation ...
	for g in groups {
		let dir = format!("emoji/src/{}", sanitize(&g.name).to_lowercase());
		let pb: PathBuf = dir.clone().into();
		if !pb.exists() {
			std::fs::create_dir(dir).unwrap();
		}
		for s in &g.subgroups {
			let emojis = &s.emojis;
			let path = format!(
				"emoji/src/{}/{}.rs",
				sanitize(&g.name).to_lowercase(),
				sanitize(&s.name).to_lowercase()
			);
			let pb: PathBuf = path.clone().into();
			let dump = quote! {
					#(#emojis)*
			};
			File::create(pb).unwrap().write_all(format!("{}", dump).as_bytes()).unwrap();
			Command::new("rustfmt").arg(path).output().expect("Failed to execute command");
		}
	}
}
