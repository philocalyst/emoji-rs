use crate::group_subgroup::Group;
use crate::sanitize;
use heck::ToUpperCamelCase;
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::process::Command;

pub fn dump(
    groups: &Vec<Group>,
    group_map: &HashMap<u8, String>,
    subgroup_map: &HashMap<u8, String>,
    annotation_langs: &Vec<&str>,
    version: f32,
    date: String,
) {
    let mut fs = String::new();
    let mut f = File::open("generate/src/library_header.rs").unwrap();
    f.read_to_string(&mut fs).unwrap();
    let ts: TokenStream = fs.parse().unwrap();

    // Generate Enums
    let mut group_variants = vec![];
    for (_, key) in group_map {
        let ident =
            proc_macro2::Ident::new(&key.to_upper_camel_case(), proc_macro2::Span::call_site());
        group_variants.push(quote! { #ident });
    }

    let mut subgroup_variants = vec![];
    for (_, key) in subgroup_map {
        let ident =
            proc_macro2::Ident::new(&key.to_upper_camel_case(), proc_macro2::Span::call_site());
        subgroup_variants.push(quote! { #ident });
    }

    let dump = quote! {
        #ts

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum Group {
            #(#group_variants),*
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum Subgroup {
            #(#subgroup_variants),*
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
    File::create(pb)
        .unwrap()
        .write_all(format!("{}", dump).as_bytes())
        .unwrap();
    Command::new("rustfmt")
        .arg(path)
        .output()
        .expect("Failed to execute command");

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
            File::create(pb)
                .unwrap()
                .write_all(format!("{}", dump).as_bytes())
                .unwrap();
            Command::new("rustfmt")
                .arg(path)
                .output()
                .expect("Failed to execute command");
        }
    }
}
