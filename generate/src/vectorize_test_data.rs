use crate::group_subgroup::*;
use crate::Annotation;
use crate::Emoji;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Deserialize, Debug)]
pub struct RawEmoji {
    pub label: String,
    pub hexcode: String,
    pub emoji: String,
    pub group: Option<u8>,
    pub subgroup: Option<u8>,
    pub version: Option<f32>,
    pub skins: Option<Vec<RawEmoji>>,
}

#[derive(Deserialize, Debug)]
struct RawGroup {
    key: String,
    order: u8,
}

#[derive(Deserialize, Debug)]
struct RawSubgroup {
    key: String,
    order: u8,
}

#[derive(Deserialize, Debug)]
struct RawMessages {
    groups: Vec<RawGroup>,
    subgroups: Vec<RawSubgroup>,
}

/// Returns: (Date, Version, Groups, GroupMap, SubgroupMap)
pub async fn vectorize_test_data(
    _client: &reqwest::Client,
    annotations: &HashMap<String, Vec<Annotation>>,
) -> reqwest::Result<(
    String,
    f32,
    Vec<Group>,
    HashMap<u8, String>,
    HashMap<u8, String>,
)> {
    // HARDCODED PATH per request
    let base_path = "/Users/philocalyst/Projects/emoji-rs/emojibase/packages/data/en";

    // 1. Load Metadata (Groups/Subgroups)
    let msgs_path = Path::new(base_path).join("messages.raw.json");
    let msgs_content = fs::read_to_string(msgs_path).expect("Failed to read messages.raw.json");
    let messages: RawMessages =
        serde_json::from_str(&msgs_content).expect("Failed to parse messages");

    // Maps for Order ID -> Key (e.g. 0 -> "smileys-emotion")
    let group_map: HashMap<u8, String> = messages
        .groups
        .into_iter()
        .map(|g| (g.order, g.key))
        .collect();

    let subgroup_map: HashMap<u8, String> = messages
        .subgroups
        .into_iter()
        .map(|s| (s.order, s.key))
        .collect();

    // 2. Load Data
    let data_path = Path::new(base_path).join("data.raw.json");
    let data_content = fs::read_to_string(data_path).expect("Failed to read data.raw.json");
    let raw_emojis: Vec<RawEmoji> =
        serde_json::from_str(&data_content).expect("Failed to parse data");

    // 3. Transform
    let mut groups: Vec<Group> = vec![];
    let mut current_version = 0.0;

    let get_or_create_group = |groups: &mut Vec<Group>, name: String| -> usize {
        if let Some(idx) = groups.iter().position(|g| g.name == name) {
            idx
        } else {
            groups.push(Group::new(name));
            groups.len() - 1
        }
    };

    let get_or_create_subgroup = |subgroups: &mut Vec<Subgroup>, name: String| -> usize {
        if let Some(idx) = subgroups.iter().position(|s| s.name == name) {
            idx
        } else {
            subgroups.push(Subgroup::new(name));
            subgroups.len() - 1
        }
    };

    for raw in raw_emojis {
        if let Some(v) = raw.version {
            if v > current_version {
                current_version = v;
            }
        }

        let group_name = raw
            .group
            .and_then(|id| group_map.get(&id))
            .cloned()
            .unwrap_or_else(|| "component".to_string());
        let subgroup_name = raw
            .subgroup
            .and_then(|id| subgroup_map.get(&id))
            .cloned()
            .unwrap_or_else(|| "other-object".to_string());

        let mut emoji = Emoji::from_raw(
            &raw,
            annotations,
            group_name.clone(),
            subgroup_name.clone(),
            false, // is_variant
        );

        // Handle skin tones (Toned variants)
        if let Some(skins) = &raw.skins {
            for skin in skins {
                let variant = Emoji::from_raw(
                    skin,
                    annotations,
                    group_name.clone(),
                    subgroup_name.clone(),
                    true, // is_variant
                );
                emoji.add_skin_tone(variant);
            }
        }

        // Add to structure
        let g_idx = get_or_create_group(&mut groups, group_name);
        let s_idx = get_or_create_subgroup(&mut groups[g_idx].subgroups, subgroup_name);
        groups[g_idx].subgroups[s_idx].emojis.push(emoji);
    }

    // Emojibase data doesn't include the release date in data.raw.json.
    // Using current time.
    let date = chrono::Utc::now().to_rfc3339();

    Ok((date, current_version, groups, group_map, subgroup_map))
}
