use std::{fs::File, io::{Read, Write}, path::PathBuf, process::Command};

use proc_macro2::TokenStream;
use quote::quote;

use crate::{group_subgroup::Group, lookup_types::GlyphLookupEntry};

pub fn dump(groups: &Vec<Group>) {
	let mut lookup_by_glyph: Vec<GlyphLookupEntry> = vec![];
	for g in groups {
		for s in &g.subgroups {
			for e in &s.emojis {
				let is_toned = !e.skin_tones.is_empty();

				// 1. Add entry for the base glyph
				lookup_by_glyph.push(GlyphLookupEntry::new(
					&e.glyph,
					&e.group,
					&e.subgroup,
					&e.name,
					is_toned,
				));

				// 2. Add entries for standard variants (e.g. gender)
				for v in &e.variants {
					lookup_by_glyph.push(GlyphLookupEntry::new(
						&v.glyph,
						&e.group,    // Use Parent Group
						&e.subgroup, // Use Parent Subgroup
						&e.name,     // Use Parent Name (Constant)
						is_toned,
					));
				}

				// 3. Add entries for Skin Tone variants
				// We map these glyphs to the Parent Toned Constant
				for t in &e.skin_tones {
					lookup_by_glyph.push(GlyphLookupEntry::new(
						&t.glyph,
						&e.group,
						&e.subgroup,
						&e.name, // Point to the Parent Constant
						is_toned,
					));
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
