use std::{collections::HashSet, fs::File, io::{Read, Write}, path::PathBuf, process::Command};

use proc_macro2::TokenStream;
use quote::quote;

use crate::{group_subgroup::Group, lookup_types::GlyphLookupEntry};

pub fn dump(groups: &Vec<Group>) {
	let mut lookup_by_glyph: Vec<GlyphLookupEntry> = vec![];
	let mut seen: HashSet<String> = HashSet::new(); // Prevent duplicates

	for g in groups {
		for s in &g.subgroups {
			for e in &s.emojis {
				let is_toned = !e.skin_tones.is_empty();

				// Helper to add entries without duplicates
				let mut add_entry = |glyph: String| {
					if seen.insert(glyph.clone()) {
						lookup_by_glyph.push(GlyphLookupEntry::new(
							glyph,
							e.group.clone(),
							e.subgroup.clone(),
							e.name.clone(),
							is_toned,
						));
					}
				};

				// 1. Add the fully qualified form
				add_entry(e.glyph.clone());

				// 2. Add the canonical/base form (without variation selectors)
				let canonical = e.canonical_glyph();
				if canonical != e.glyph {
					add_entry(canonical);
				}

				// 3. Handle standard variants (e.g., gender)
				for v in &e.variants {
					add_entry(v.glyph.clone());
					let v_canonical = v.canonical_glyph();
					if v_canonical != v.glyph {
						add_entry(v_canonical);
					}
				}

				// 4. Handle skin tone variants
				for t in &e.skin_tones {
					add_entry(t.glyph.clone());
					let t_canonical = t.canonical_glyph();
					if t_canonical != t.glyph {
						add_entry(t_canonical);
					}
				}
			}
		}
	}

	let mut fs = String::new();
	let mut f = File::open("generate/src/glyph_lookup_header.rs").unwrap();
	f.read_to_string(&mut fs).unwrap();
	let ts: TokenStream = fs.parse().unwrap();

	let dump = quote! {
			#ts
			static GLYPH_LOOKUP_MAP: phf::Map<&'static str, crate::EmojiEntry> = phf::phf_map! {
					#(#lookup_by_glyph),*
			};
	};

	let path = "emoji/src/lookup_by_glyph.rs";
	let pb: PathBuf = path.into();
	File::create(pb).unwrap().write_all(format!("{}", dump).as_bytes()).unwrap();
	Command::new("rustfmt").arg(path).output().expect("Failed to execute command");
}
