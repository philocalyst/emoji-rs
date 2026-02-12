use std::{collections::HashMap, error::Error, process::Command};

use sea_query::{Alias, Expr, Iden, MysqlQueryBuilder, Order, Query};
use serde::Deserialize;

use crate::{Annotation, Emoji, emoji::{SkinTone, Status, Version}, group_subgroup::{Group, Subgroup}};

// Helper for loading from Dolt
fn run_query<T: for<'de> Deserialize<'de>>(query: &str) -> Result<Vec<T>, Box<dyn Error>> {
	let output =
		Command::new("dolt").args(&["sql", "-q", query, "-r", "csv"]).current_dir("two").output()?;

	if !output.status.success() {
		return Err(format!("Dolt query failed: {}", String::from_utf8_lossy(&output.stderr)).into());
	}

	let stdout = String::from_utf8(output.stdout)?;
	let mut reader = csv::ReaderBuilder::new().from_reader(stdout.as_bytes());
	let mut results = Vec::new();
	for result in reader.deserialize() {
		let record: T = result?;
		results.push(record);
	}
	Ok(results)
}

#[derive(Iden)]
enum EmojiGroups {
	Table,
	Id,
	Name,
}

#[derive(Iden)]
enum EmojiSubgroups {
	Table,
	Id,
	GroupId,
	Name,
}

#[derive(Iden)]
enum Emojis {
	Table,
	Hexcode,
	Emoji,
	#[iden = "type"]
	Type,
	Version,
	GroupId,
	SubgroupId,
	Ordering,
	ColorOrder,
}

#[derive(Iden)]
enum EmojiTranslations {
	Table,
	Hexcode,
	Locale,
	Label,
}

#[derive(Iden)]
enum EmojiTags {
	Table,
	Hexcode,
	Tag,
	Locale,
}

#[derive(Iden)]
enum EmojiSkinToneVariants {
	Table,
	Hexcode,
	Emoji,
	BaseHexcode,
	Ordering,
	ColorOrder,
	Tone,
	Version,
}

#[derive(Iden)]
enum EmojiGenderVariants {
	Table,
	Hexcode,
	Emoji,
	BaseHexcode,
	Ordering,
	ColorOrder,
	Gender,
	Version,
}

#[derive(Debug, Deserialize)]
struct DbGroup {
	id:   u32,
	name: String,
}

#[derive(Debug, Deserialize)]
struct DbSubgroup {
	id:       u32,
	group_id: u32,
	name:     String,
}

#[derive(Debug, Deserialize)]
struct DbEmoji {
	hexcode:     String,
	emoji:       String,
	#[serde(rename = "type")]
	type_:       Option<u32>,
	version:     Option<f32>,
	group_id:    Option<u32>,
	subgroup_id: Option<u32>,
	ordering:    Option<u32>,
	color_order: Option<u32>,
}

#[derive(Debug, Deserialize)]
struct DbTranslation {
	hexcode: String,
	locale:  String,
	label:   Option<String>,
}

#[derive(Debug, Deserialize)]
struct DbTag {
	hexcode: String,
	tag:     String,
	locale:  String,
}

#[derive(Debug, Deserialize)]
struct DbSkinToneVariant {
	hexcode:      String,
	emoji:        String,
	base_hexcode: Option<String>,
	ordering:     Option<u32>,
	color_order:  Option<u32>,
	tone:         Option<String>,
	version:      Option<f32>,
}

#[derive(Debug, Deserialize)]
struct DbGenderVariant {
	hexcode:      String,
	emoji:        String,
	base_hexcode: Option<String>,
	ordering:     Option<u32>,
	color_order:  Option<u32>,
	gender:       Option<u32>,
	version:      Option<f32>,
}

pub fn load_all()
-> Result<(String, f32, Vec<Group>, HashMap<u32, String>, HashMap<u32, String>), Box<dyn Error>> {
	// Fetch Groups
	let query = Query::select()
		.columns([EmojiGroups::Id, EmojiGroups::Name])
		.from(EmojiGroups::Table)
		.to_string(MysqlQueryBuilder);
	let db_groups: Vec<DbGroup> = run_query(&query)?;
	let group_map: HashMap<u32, String> = db_groups.iter().map(|g| (g.id, g.name.clone())).collect();

	// Fetch Subgroups
	let query = Query::select()
		.columns([EmojiSubgroups::Id, EmojiSubgroups::GroupId, EmojiSubgroups::Name])
		.from(EmojiSubgroups::Table)
		.to_string(MysqlQueryBuilder);
	let db_subgroups: Vec<DbSubgroup> = run_query(&query)?;
	let subgroup_map: HashMap<u32, String> =
		db_subgroups.iter().map(|s| (s.id, s.name.clone())).collect();

	// Fetch Emojis
	let query = Query::select()
		.columns([
			Emojis::Hexcode,
			Emojis::Emoji,
			Emojis::Type,
			Emojis::Version,
			Emojis::GroupId,
			Emojis::SubgroupId,
			Emojis::Ordering,
			Emojis::ColorOrder,
		])
		.from(Emojis::Table)
		.order_by(Emojis::ColorOrder, Order::Asc)
		.to_string(MysqlQueryBuilder);
	let db_emojis: Vec<DbEmoji> = run_query(&query)?;

	// Fetch Variants
	let query = Query::select()
		.columns([
			EmojiSkinToneVariants::Hexcode,
			EmojiSkinToneVariants::Emoji,
			EmojiSkinToneVariants::BaseHexcode,
			EmojiSkinToneVariants::Ordering,
			EmojiSkinToneVariants::ColorOrder,
			EmojiSkinToneVariants::Tone,
			EmojiSkinToneVariants::Version,
		])
		.from(EmojiSkinToneVariants::Table)
		.order_by(EmojiSkinToneVariants::ColorOrder, Order::Asc)
		.to_string(MysqlQueryBuilder);
	let skin_tones: Vec<DbSkinToneVariant> = run_query(&query)?;

	let query = Query::select()
		.columns([
			EmojiGenderVariants::Hexcode,
			EmojiGenderVariants::Emoji,
			EmojiGenderVariants::BaseHexcode,
			EmojiGenderVariants::Ordering,
			EmojiGenderVariants::ColorOrder,
			EmojiGenderVariants::Gender,
			EmojiGenderVariants::Version,
		])
		.from(EmojiGenderVariants::Table)
		.order_by(EmojiGenderVariants::ColorOrder, Order::Asc)
		.to_string(MysqlQueryBuilder);
	let gender_variants: Vec<DbGenderVariant> = run_query(&query)?;

	// Fetch Annotations
	let query = Query::select()
		.columns([EmojiTranslations::Hexcode, EmojiTranslations::Locale, EmojiTranslations::Label])
		.from(EmojiTranslations::Table)
		.to_string(MysqlQueryBuilder);
	let translations: Vec<DbTranslation> = run_query(&query)?;

	let query = Query::select()
		.columns([EmojiTags::Hexcode, EmojiTags::Tag, EmojiTags::Locale])
		.from(EmojiTags::Table)
		.to_string(MysqlQueryBuilder);
	let tags: Vec<DbTag> = run_query(&query)?;

	// Organize Annotations
	let mut annotations_map: HashMap<String, HashMap<String, Annotation>> = HashMap::new();

	// Map hexcode to Glyph.
	let mut hex_to_glyph: HashMap<String, String> = HashMap::new();
	for e in &db_emojis {
		hex_to_glyph.insert(e.hexcode.clone(), e.emoji.clone());
	}
	for e in &skin_tones {
		hex_to_glyph.insert(e.hexcode.clone(), e.emoji.clone());
	}
	for e in &gender_variants {
		hex_to_glyph.insert(e.hexcode.clone(), e.emoji.clone());
	}

	// Process Translations
	for t in translations {
		if let Some(glyph) = hex_to_glyph.get(&t.hexcode) {
			let entry = annotations_map
				.entry(glyph.clone())
				.or_insert_with(HashMap::new)
				.entry(t.locale.clone())
				.or_insert_with(|| Annotation::new(t.locale.clone(), None, "".to_string()));

			if let Some(label) = t.label {
				entry.tts = Some(label);
			}
		}
	}

	// Process Tags
	for t in tags {
		if let Some(glyph) = hex_to_glyph.get(&t.hexcode) {
			let entry = annotations_map
				.entry(glyph.clone())
				.or_insert_with(HashMap::new)
				.entry(t.locale.clone())
				.or_insert_with(|| Annotation::new(t.locale.clone(), None, "".to_string()));

			entry.add_keywords(t.tag);
		}
	}

	// Convert annotations map to vector format expected by Emoji::from_raw
	let mut annotations_vec_map: HashMap<String, Vec<Annotation>> = HashMap::new();
	for (glyph, map) in annotations_map {
		let mut ve: Vec<Annotation> = map.into_iter().map(|(_, v)| v).collect();
		// Sort by lang to be deterministic
		ve.sort_by(|a, b| a.lang.cmp(&b.lang));
		annotations_vec_map.insert(glyph, ve);
	}

	// Build Emoji Objects
	let mut groups: Vec<Group> = vec![];
	let mut max_version = 0.0;

	// Since we need to put emojis into groups/subgroups structure, it's better to
	// keep track of them.

	// Create Groups and Subgroups structure
	for g in &db_groups {
		groups.push(Group::new(g.name.clone()));
	}
	// Initialize subgroups in groups
	for s in &db_subgroups {
		if let Some(g_name) = group_map.get(&s.group_id) {
			if let Some(g_idx) = groups.iter().position(|g| g.name == *g_name) {
				groups[g_idx].subgroups.push(Subgroup::new(s.name.clone()));
			}
		}
	}

	// Store base emojis to attach variants later.
	// We need mutable access to emojis inside groups.
	// Map hexcode -> (group_idx, subgroup_idx, emoji_idx)
	let mut emoji_locations: HashMap<String, (usize, usize, usize)> = HashMap::new();

	for raw in db_emojis {
		if let Some(v) = raw.version {
			if v > max_version {
				max_version = v;
			}
		}

		let group_name = raw
			.group_id
			.and_then(|id| group_map.get(&id))
			.cloned()
			.unwrap_or_else(|| "component".to_string());

		let subgroup_name = raw
			.subgroup_id
			.and_then(|id| subgroup_map.get(&id))
			.cloned()
			.unwrap_or_else(|| "other-object".to_string());

		let codepoint: Vec<u32> =
			raw.hexcode.split('-').map(|s| u32::from_str_radix(s, 16).unwrap_or(0)).collect();
		let tone_val = SkinTone::from_codepoint(&codepoint);

		let version_float = raw.version.unwrap_or(0.0);
		let major = version_float as u8;
		let minor = ((version_float - major as f32) * 10.0) as u8;

		let annotations = annotations_vec_map.get(&raw.emoji).cloned().unwrap_or_default();

		let emoji = Emoji {
			codepoint,
			status: Status::FullyQualified, // Assuming
			glyph: raw.emoji.clone(),
			introduction_version: Version { major, minor, patch: 0 },
			name: "".to_string(),
			variants: vec![],
			skin_tones: vec![],
			gender_variants: vec![],
			annotations: annotations.clone(),
			is_variant: false,
			group: group_name.clone(),
			subgroup: subgroup_name.clone(),
			tone_val,
		};

		// Find English name
		let mut name = "unknown".to_string();
		for ann in &emoji.annotations {
			if ann.lang == "en" {
				if let Some(tts) = &ann.tts {
					name = tts.clone();
					break;
				}
			}
		}

		// Update name
		let mut emoji = emoji;
		emoji.name = name;

		// Add to groups
		if let Some(g_idx) = groups.iter().position(|g| g.name == group_name) {
			if let Some(s_idx) = groups[g_idx].subgroups.iter().position(|s| s.name == subgroup_name) {
				groups[g_idx].subgroups[s_idx].emojis.push(emoji);
				emoji_locations.insert(
					raw.hexcode.clone(),
					(g_idx, s_idx, groups[g_idx].subgroups[s_idx].emojis.len() - 1),
				);
			}
		}
	}

	// Process Skin Tone Variants
	for variant in skin_tones {
		if let Some(base_hex) = &variant.base_hexcode {
			if let Some((g_idx, s_idx, e_idx)) = emoji_locations.get(base_hex) {
				// Create variant emoji
				let codepoint: Vec<u32> =
					variant.hexcode.split('-').map(|s| u32::from_str_radix(s, 16).unwrap_or(0)).collect();
				let tone_val = SkinTone::from_codepoint(&codepoint);
				let version_float = variant.version.unwrap_or(0.0);
				let major = version_float as u8;
				let minor = ((version_float - major as f32) * 10.0) as u8;
				let annotations = annotations_vec_map.get(&variant.emoji).cloned().unwrap_or_default();

				// Get parent group/subgroup
				let parent = &groups[*g_idx].subgroups[*s_idx].emojis[*e_idx];
				let group_name = parent.group.clone();
				let subgroup_name = parent.subgroup.clone();

				let mut emoji = Emoji {
					codepoint,
					status: Status::FullyQualified,
					glyph: variant.emoji.clone(),
					introduction_version: Version { major, minor, patch: 0 },
					name: "".to_string(),
					variants: vec![],
					skin_tones: vec![],
					gender_variants: vec![],
					annotations: annotations.clone(),
					is_variant: true,
					group: group_name,
					subgroup: subgroup_name,
					tone_val,
				};

				// Find English name
				let mut name = "unknown".to_string();
				for ann in &emoji.annotations {
					if ann.lang == "en" {
						if let Some(tts) = &ann.tts {
							name = tts.clone();
							break;
						}
					}
				}
				emoji.name = name;

				groups[*g_idx].subgroups[*s_idx].emojis[*e_idx].add_skin_tone(emoji);
			}
		}
	}

	// Process Gender Variants
	for variant in gender_variants {
		if let Some(base_hex) = &variant.base_hexcode {
			if let Some((g_idx, s_idx, e_idx)) = emoji_locations.get(base_hex) {
				// Create variant emoji
				let codepoint: Vec<u32> =
					variant.hexcode.split('-').map(|s| u32::from_str_radix(s, 16).unwrap_or(0)).collect();
				let tone_val = SkinTone::from_codepoint(&codepoint);
				let version_float = variant.version.unwrap_or(0.0);
				let major = version_float as u8;
				let minor = ((version_float - major as f32) * 10.0) as u8;
				let annotations = annotations_vec_map.get(&variant.emoji).cloned().unwrap_or_default();

				let parent = &groups[*g_idx].subgroups[*s_idx].emojis[*e_idx];
				let group_name = parent.group.clone();
				let subgroup_name = parent.subgroup.clone();

				let mut emoji = Emoji {
					codepoint,
					status: Status::FullyQualified,
					glyph: variant.emoji.clone(),
					introduction_version: Version { major, minor, patch: 0 },
					name: "".to_string(),
					variants: vec![],
					skin_tones: vec![],
					gender_variants: vec![],
					annotations: annotations.clone(),
					is_variant: true,
					group: group_name,
					subgroup: subgroup_name,
					tone_val,
				};

				// Find English name
				let mut name = "unknown".to_string();
				for ann in &emoji.annotations {
					if ann.lang == "en" {
						if let Some(tts) = &ann.tts {
							name = tts.clone();
							break;
						}
					}
				}
				emoji.name = name;

				groups[*g_idx].subgroups[*s_idx].emojis[*e_idx].add_gender_variant(emoji);
			}
		}
	}

	let date = chrono::Utc::now().to_rfc3339();

	Ok((date, max_version, groups, group_map, subgroup_map))
}
