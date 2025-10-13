use crate::group_subgroup::Group;
use crate::lookup_types::GlyphLookupEntry;
use proc_macro2::TokenStream;
use quote::quote;
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::process::Command;

pub fn dump(groups: &Vec<Group>) {
    let mut lookup_by_glyph: Vec<GlyphLookupEntry> = vec![];
    for g in groups {
        for s in &g.subgroups {
            for e in &s.emojis {
                lookup_by_glyph.push(GlyphLookupEntry::new(
                    &e.glyph,
                    &e.group,
                    &e.subgroup,
                    &e.name,
                ));
                for v in &e.variants {
                    lookup_by_glyph.push(GlyphLookupEntry::new(
                        &v.glyph,
                        &v.group,
                        &v.subgroup,
                        &v.name,
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
    static GLYPH_LOOKUP_MAP: phf::Map<&'static str, crate::Emoji> = phf::phf_map! {
        #(#lookup_by_glyph),*
    };
    };

    let path = "emoji/src/lookup_by_glyph.rs";
    let pb: PathBuf = path.into();
    File::create(pb)
        .unwrap()
        .write_all(format!("{}", dump).as_bytes())
        .unwrap();
    Command::new("rustfmt")
        .arg(path)
        .output()
        .expect("Failed to execute command");
}
