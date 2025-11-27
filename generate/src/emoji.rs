use crate::sanitize;
use heck::ToUpperCamelCase;
use itertools::Itertools;
use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, ToTokens};
use std::collections::HashMap;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub enum SkinTone {
    Default,
    Light,
    MediumLight,
    Medium,
    MediumDark,
    Dark,
}

impl SkinTone {
    pub fn from_codepoint(cp: &[u32]) -> Self {
        if cp.contains(&0x1F3FB) {
            return SkinTone::Light;
        }
        if cp.contains(&0x1F3FC) {
            return SkinTone::MediumLight;
        }
        if cp.contains(&0x1F3FD) {
            return SkinTone::Medium;
        }
        if cp.contains(&0x1F3FE) {
            return SkinTone::MediumDark;
        }
        if cp.contains(&0x1F3FF) {
            return SkinTone::Dark;
        }
        SkinTone::Default
    }

    pub fn as_token(&self) -> TokenStream {
        match self {
            SkinTone::Default => quote!(crate::SkinTone::Default),
            SkinTone::Light => quote!(crate::SkinTone::Light),
            SkinTone::MediumLight => quote!(crate::SkinTone::MediumLight),
            SkinTone::Medium => quote!(crate::SkinTone::Medium),
            SkinTone::MediumDark => quote!(crate::SkinTone::MediumDark),
            SkinTone::Dark => quote!(crate::SkinTone::Dark),
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct Emoji {
    pub codepoint: Vec<u32>, // Correct type
    pub status: Status,
    pub glyph: String,
    pub introduction_version: Version,
    pub name: String,
    pub variants: Vec<Emoji>,   // Non-skin tone variants
    pub skin_tones: Vec<Emoji>, // Skin tone variants
    pub annotations: Vec<Annotation>,
    pub is_variant: bool,
    pub group: String,
    pub subgroup: String,
    pub tone_val: SkinTone, // Used internally, not exported in struct
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct Version {
    pub major: u8,
    pub minor: u8,
    pub patch: u8,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VersionParseError {
    WrongPartCount,
    InvalidNumber(ParseIntError),
}

impl std::fmt::Display for VersionParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VersionParseError::WrongPartCount => {
                write!(f, "expected exactly three dot-separated parts (e.g. 1.2.3)")
            }
            VersionParseError::InvalidNumber(e) => write!(f, "invalid number: {}", e),
        }
    }
}

impl std::error::Error for VersionParseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            VersionParseError::InvalidNumber(e) => Some(e),
            _ => None,
        }
    }
}

impl FromStr for Version {
    type Err = VersionParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.trim().split('.').collect();
        if parts.len() != 3 {
            return Err(VersionParseError::WrongPartCount);
        }

        let parse_u8 = |p: &str| -> Result<u8, VersionParseError> {
            p.parse::<u8>().map_err(VersionParseError::InvalidNumber)
        };

        let major = parse_u8(parts[0])?;
        let minor = parse_u8(parts[1])?;
        let patch = parse_u8(parts[2])?;

        Ok(Version {
            major,
            minor,
            patch,
        })
    }
}

impl Emoji {
    pub fn from_raw(
        raw: &crate::vectorize_test_data::RawEmoji,
        annotations_map: &HashMap<String, Vec<Annotation>>,
        group: String,
        subgroup: String,
        is_variant: bool,
    ) -> Self {
        // Parse hexcode "1F1E6-1F1E7" -> Vec<u32>
        let codepoint: Vec<u32> = raw
            .hexcode
            .split('-')
            .map(|s| u32::from_str_radix(s, 16).expect("Invalid hex"))
            .collect();

        let tone_val = SkinTone::from_codepoint(&codepoint);

        let version_float = raw.version.unwrap_or(0.0);
        let major = version_float as u8;
        let minor = ((version_float - major as f32) * 10.0) as u8;

        let annotations = match annotations_map.get(&raw.emoji) {
            None => vec![],
            Some(a) => a.to_vec(),
        };

        Self {
            codepoint,
            status: Status::FullyQualified,
            glyph: raw.emoji.clone(),
            introduction_version: Version {
                major,
                minor,
                patch: 0,
            },
            name: raw.label.clone(),
            variants: vec![],
            skin_tones: vec![],
            annotations,
            is_variant,
            group,
            subgroup,
            tone_val,
        }
    }

    pub fn add_variant(&mut self, variant: Emoji) {
        self.variants.push(variant);
    }

    pub fn add_skin_tone(&mut self, variant: Emoji) {
        self.skin_tones.push(variant);
    }

    pub fn ident(&self) -> String {
        sanitize(&self.name).to_uppercase()
    }

    /// Generates the tokens for the Emoji struct (without the Toned wrapper)
    fn tokens_struct(&self) -> TokenStream {
        let glyph = &self.glyph;
        let cp = &self.codepoint;
        let name = &self.name;
        let group_ident = Ident::new(&&self.group.to_upper_camel_case(), Span::call_site());
        let subgroup_ident = Ident::new(&&self.subgroup.to_upper_camel_case(), Span::call_site());

        let major = self.introduction_version.major;
        let minor = self.introduction_version.minor;
        let patch = self.introduction_version.patch;

        let variants: Vec<TokenStream> = self.variants.iter().map(|e| e.tokens_struct()).collect();
        let annotations = &self.annotations;
        let is_variant = &self.is_variant;

        // Note: No 'skin_tones' field here. They are only in the wrapper.
        quote! {
            crate::Emoji {
                glyph: #glyph,
                codepoint: &[#(#cp),*],
                status: crate::Status::FullyQualified,
                introduction_version: crate::Version {
                    major: #major,
                    minor: #minor,
                    patch: #patch,
                },
                name: #name,
                group: crate::Group::#group_ident,
                subgroup: crate::Subgroup::#subgroup_ident,
                is_variant: #is_variant,
                variants: &[#(#variants),*],
                annotations: &[#(#annotations),*],
            }
        }
    }
}

impl ToTokens for Emoji {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ident = Ident::new(&self.ident(), Span::call_site());
        let emoji_struct = self.tokens_struct();
        let glyph = &self.glyph;

        if !self.skin_tones.is_empty() {
            // It is a Toned emoji
            let tones: Vec<TokenStream> =
                self.skin_tones.iter().map(|e| e.tokens_struct()).collect();

            (quote! {
                #[doc = #glyph]
                pub const #ident: crate::Toned = crate::Toned {
                    emoji: #emoji_struct,
                    tones: &[#(#tones),*]
                };
            })
            .to_tokens(tokens);
        } else {
            // It is a regular Emoji
            (quote! {
                #[doc = #glyph]
                pub const #ident: crate::Emoji = #emoji_struct;
            })
            .to_tokens(tokens);
        }
    }
}

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
pub enum Status {
    Component,
    FullyQualified,
    MinimallyQualified,
    Unqualified,
}
impl Status {
    pub fn new(name: &str) -> Self {
        use Status::*;
        match name {
            "component" => Component,
            "fully-qualified" => FullyQualified,
            "minimally-qualified" => MinimallyQualified,
            "unqualified" => Unqualified,
            unknown => panic!("Unknown qualifier {}", unknown),
        }
    }
}
impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use Status::*;
        write!(
            f,
            "{}",
            match self {
                Component => "Component",
                FullyQualified => "FullyQualified",
                MinimallyQualified => "MinimallyQualified",
                Unqualified => "Unqualified",
            }
        )
    }
}
#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct Annotation {
    pub lang: String,
    pub tts: Option<String>,
    pub keywords: Vec<String>,
}
impl Annotation {
    pub fn new(lang: String, tts: Option<String>, keywords: String) -> Self {
        let mut s = Self {
            lang,
            tts,
            keywords: vec![],
        };
        s.add_keywords(keywords);
        s
    }
    pub fn add_keywords(&mut self, keywords: String) {
        let mut v = keywords.split("|").map(|a| a.trim().to_owned()).collect();
        self.keywords.append(&mut v);
        self.keywords.sort();
        self.keywords.dedup();
    }
}
impl ToTokens for Annotation {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let lang = &self.lang;
        let tts = match &self.tts {
            None => quote! {
            None
                },
            Some(tts) => quote! {
            Some(#tts)
                },
        };
        let keywords = &self.keywords;
        (quote! {
        #[cfg(feature = #lang)]
            crate::Annotation {
        lang: #lang,
        tts: #tts,
        keywords: &[#(#keywords),*],
            }
        })
        .to_tokens(tokens);
    }
}
