#![doc = " ## Introduction  "]
#![doc = " 3511 emojis and 4580 emoji variants with localization data in 143 languages"]
#![doc = " This crate contains a huge amount of data about every emoji ever."]
#![doc = " Some of the data includes:"]
#![doc = " - Name"]
#![doc = " - Glyph"]
#![doc = " - Unicode Release Version"]
#![doc = " - Classification"]
#![doc = " - Variants"]
#![doc = " - Annotations in many languages"]
#![doc = ""]
#![doc = " This crate also provides functions for searching through emojis by"]
#![doc = " [name](lookup_by_name/index.html) and [glyph](lookup_by_glyph/index.html),"]
#![doc = " as well as several [fuzzy search](search/index.html) functions."]
#![doc = " ## Quickstart  "]
#![doc = " ```rust"]
#![doc = " fn main() {"]
#![doc = "    println!(\"{}\", emoji::food_and_drink::food_marine::CRAB.glyph);"]
#![doc = " }"]
#![doc = " ```"]
#![doc = " See more examples [here](https://github.com/Shizcow/emoji-rs/tree/master/examples/)."]
#![doc = " ## Languages  "]
#![doc = " By default, only English annotations are compiled in."]
#![doc = " To enable other languages, use the feature corresponding to that languge. An"]
#![doc = " exhaustive list of supported languages can be found"]
#![doc = " [here](https://github.com/Shizcow/emoji-rs/blob/master/emoji/Cargo.toml)."]
#[doc = " Emoji status qualifier  "]
#[doc = " In nearly every case, MinimallyQualified or Unqualified will show up in"]
#[doc = " emoji variants. A complete tool needs only to support all of the"]
#[doc = " FullyQualified emojis."]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Status {
	#[doc = " A qualified emoji character, or an emoji sequence in which each emoji"]
	#[doc = " character is qualified. Most emojis fall into this category."]
	FullyQualified,
	#[doc = " An emoji sequence in which the first character is qualified but the"]
	#[doc = " sequence is not fully qualified."]
	MinimallyQualified,
	#[doc = " An emoji that is neither fully-qualified nor minimally qualified."]
	Unqualified,
	#[doc = " Used for modifiers, such as skin tone modifiers."]
	Component,
}
impl std::fmt::Display for Status {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		use Status::*;
		write!(f, "{}", match self {
			Component => "Component",
			FullyQualified => "FullyQualified",
			MinimallyQualified => "MinimallyQualified",
			Unqualified => "Unqualified",
		})
	}
}
#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct Version {
	pub major: u8,
	pub minor: u8,
	pub patch: u8,
}
#[doc = " Represents the skin tone of an emoji."]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SkinTone {
	Default,
	Light,
	MediumLight,
	Medium,
	MediumDark,
	Dark,
}
#[doc = " The result of a lookup operation."]
#[doc = " Distinguishes between an emoji that has skin tones available (Toned)"]
#[doc = " and one that does not (Standard)."]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EmojiEntry {
	#[doc = " An emoji that has no skin tone variants (e.g. 😺)."]
	Standard(&'static Emoji),
	#[doc = " An emoji that has skin tone variants."]
	#[doc = " Contains the base emoji and the list of toned variants."]
	Toned(&'static Toned),
}
impl EmojiEntry {
	#[doc = " Helper to access the base emoji data regardless of type."]
	pub fn emoji(&self) -> &'static Emoji {
		match self {
			EmojiEntry::Standard(e) => e,
			EmojiEntry::Toned(t) => &t.emoji,
		}
	}

	#[doc = " Returns the variants (skin tones) if they exist."]
	pub fn tones(&self) -> Option<&'static [Emoji]> {
		match self {
			EmojiEntry::Standard(_) => None,
			EmojiEntry::Toned(t) => Some(t.tones),
		}
	}
}
#[doc = " A wrapper around an Emoji that has skin tone variants."]
#[doc = " Dereferences to the base Emoji."]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Toned {
	pub emoji: Emoji,
	pub tones: &'static [Emoji],
}
use core::fmt;
use std::ops::Deref;
impl Deref for Toned {
	type Target = Emoji;

	fn deref(&self) -> &Self::Target { &self.emoji }
}
impl Toned {
	#[doc = " Returns all skin tone variants for this emoji"]
	pub fn tones(&self) -> &'static [Emoji] { self.tones }
}
#[doc = " Contains all information about an emoji  "]
#[doc = " See the [CLDR](https://raw.githubusercontent.com/unicode-org/cldr/release-38/tools/java/org/unicode/cldr/util/data/emoji/emoji-test.txt) for specific examples of all fields except `variants`."]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Emoji {
	#[doc = " The u32 array representation of this emoji's UTF8 codepoint value  "]
	#[doc = " Ex: `1F441 200D 1F5E8 FE0F`"]
	pub codepoint:            &'static [u32],
	#[doc = " Qualification status"]
	pub status:               Status,
	#[doc = " The actual emoji text  "]
	#[doc = " Ex: 😺"]
	pub glyph:                &'static str,
	#[doc = " The Unicode release version which this emoji was introduced in"]
	pub introduction_version: Version,
	#[doc = " English [CLDR Short Name](https://unicode.org/emoji/format.html#col-name)"]
	#[doc = " (canonical) name of this emoji  "]
	#[doc = " Ex: `grinning cat`"]
	pub name:                 &'static str,
	#[doc = " General classification this emoji belongs to  "]
	#[doc = " Ex: `Smileys & Emotion`"]
	pub group:                Group,
	#[doc = " Specific classification this emoji belongs to  "]
	#[doc = " Ex: `cat-face`"]
	pub subgroup:             Subgroup,
	#[doc = " All variants of an emoji. If two emojis share the same name, one is a"]
	#[doc = " variant. Variants are always less qualified than their parent. Parents"]
	#[doc = " can be found from a variant via"]
	#[doc = " [emoji::lookup_by_glyph::lookup](lookup_by_glyph/fn.lookup.html)"]
	pub variants:             &'static [Emoji],
	#[doc = " Is this emoji a variant?"]
	pub is_variant:           bool,
	#[doc = " Localizatoin specific annotations"]
	pub annotations:          &'static [Annotation],
}
#[doc = " Annotation meta-data for each emoji"]
#[derive(Debug, PartialEq, Clone, Eq, Hash)]
pub struct Annotation {
	#[doc = " Language code of the associated data. Guarenteed to be found in"]
	#[doc = " [ANNOTATION_LANGS_AVAILABLE](constant.ANNOTATION_LANGS_AVAILABLE.html)"]
	pub lang:     &'static str,
	#[doc = " Localized name for an emoji  "]
	#[doc = " Ex: `fried shrimp`"]
	pub tts:      Option<&'static str>,
	#[doc = " Keywords associated with an emoji, in the localized language  "]
	#[doc = " Ex: `[\"fried shrimp\", \"shrimp\", \"prawn\"]`"]
	pub keywords: &'static [&'static str],
}
#[doc = " Defines functions for searching through and iterating over emojis by glyph  "]
#[doc = " Includes variants"]
pub mod lookup_by_glyph;
#[doc = " Defines functions for searching through and iterating over emojis by name  "]
#[doc = " Yields exact matches only, but is extremely fast  "]
#[doc = " Does not include variants"]
pub mod lookup_by_name;
#[doc = " Fuzzy search algorithms for general purpose searching"]
pub mod search;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Group {
	PeopleBody,
	Component,
	Symbols,
	Flags,
	FoodDrink,
	SmileysEmotion,
	AnimalsNature,
	TravelPlaces,
	Activities,
	Objects,
}

impl fmt::Display for Group {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let val = match self {
			Self::PeopleBody => "People & Body",
			Self::Component => "Component",
			Self::Symbols => "Symbols",
			Self::Flags => "Flags",
			Self::FoodDrink => "Food & Drink",
			Self::SmileysEmotion => "Smileys & Emotion",
			Self::AnimalsNature => "Animals & Nature",
			Self::TravelPlaces => "Travel & Places",
			Self::Activities => "Activities",
			Self::Objects => "Objects",
		};
		write!(f, "{}", val)
	}
}

impl Group {
	/// Returns an iterator over all variants.
	pub fn iter() -> impl Iterator<Item = Self> {
		[
			Self::PeopleBody,
			Self::Component,
			Self::Symbols,
			Self::Flags,
			Self::FoodDrink,
			Self::SmileysEmotion,
			Self::AnimalsNature,
			Self::TravelPlaces,
			Self::Activities,
			Self::Objects,
		]
		.iter()
		.copied()
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Subgroup {
	Dishware,
	TransportSign,
	SkinTone,
	PlaceMap,
	HandFingersClosed,
	Mail,
	PersonRole,
	FaceGlasses,
	AnimalMammal,
	Tool,
	PersonSymbol,
	PlaceOther,
	Currency,
	FoodFruit,
	Event,
	Gender,
	Phone,
	Sound,
	Alphanum,
	FaceConcerned,
	HandSingleFinger,
	Computer,
	HandFingersPartial,
	FaceHand,
	HandFingersOpen,
	AnimalReptile,
	AnimalMarine,
	PlaceBuilding,
	FaceCostume,
	FaceHat,
	BookPaper,
	Hands,
	PersonResting,
	FoodVegetable,
	Game,
	Heart,
	Lock,
	Medical,
	Music,
	Office,
	Keycap,
	SkyWeather,
	FoodAsian,
	CountryFlag,
	Writing,
	MusicalInstrument,
	Person,
	FaceSmiling,
	TransportGround,
	PlaceReligious,
	Arrow,
	FaceNegative,
	PlantOther,
	PlaceGeographic,
	Punctuation,
	AnimalBug,
	FaceAffection,
	FaceNeutralSkeptical,
	BodyParts,
	AnimalBird,
	FoodPrepared,
	ArtsCrafts,
	PersonGesture,
	Zodiac,
	Warning,
	Math,
	PlantFlower,
	TransportWater,
	LightVideo,
	AnimalAmphibian,
	PersonSport,
	CatFace,
	Drink,
	Money,
	Household,
	FoodSweet,
	Family,
	Emotion,
	Time,
	Clothing,
	HairStyle,
	PersonFantasy,
	Science,
	AvSymbol,
	PersonActivity,
	OtherSymbol,
	TransportAir,
	MonkeyFace,
	AwardMedal,
	FaceSleepy,
	FaceTongue,
	Sport,
	OtherObject,
	FaceUnwell,
	Hotel,
	Geometric,
	Flag,
	SubdivisionFlag,
	HandProp,
	Religion,
}
#[doc = r" All annotation languages"]
pub const ANNOTATION_LANGS_TOTAL: &'static [&'static str] = &[
	"af",
	"am",
	"ar",
	"ar_SA",
	"as",
	"ast",
	"az",
	"be",
	"bg",
	"bn",
	"br",
	"bs",
	"ca",
	"ccp",
	"ceb",
	"chr",
	"ckb",
	"cs",
	"cy",
	"da",
	"de",
	"de_CH",
	"doi",
	"el",
	"en",
	"en_001",
	"en_AU",
	"en_CA",
	"en_GB",
	"en_IN",
	"es",
	"es_419",
	"es_MX",
	"es_US",
	"et",
	"eu",
	"fa",
	"fi",
	"fil",
	"fo",
	"fr",
	"fr_CA",
	"ga",
	"gd",
	"gl",
	"gu",
	"ha",
	"ha_NE",
	"he",
	"hi",
	"hr",
	"hu",
	"hy",
	"ia",
	"id",
	"ig",
	"is",
	"it",
	"ja",
	"jv",
	"ka",
	"kab",
	"kk",
	"kl",
	"km",
	"kn",
	"ko",
	"kok",
	"ku",
	"ky",
	"lb",
	"lo",
	"lt",
	"lv",
	"mai",
	"mi",
	"mk",
	"ml",
	"mn",
	"mni",
	"mr",
	"ms",
	"mt",
	"my",
	"ne",
	"nl",
	"nn",
	"or",
	"pa",
	"pa_Arab",
	"pcm",
	"pl",
	"ps",
	"pt",
	"pt_PT",
	"qu",
	"rm",
	"ro",
	"root",
	"ru",
	"rw",
	"sa",
	"sat",
	"sd",
	"si",
	"sk",
	"sl",
	"so",
	"sq",
	"sr",
	"sr_Cyrl",
	"sr_Cyrl_BA",
	"sr_Latn",
	"sr_Latn_BA",
	"su",
	"sv",
	"sw",
	"sw_KE",
	"ta",
	"te",
	"tg",
	"th",
	"ti",
	"tk",
	"to",
	"tr",
	"tt",
	"ug",
	"uk",
	"ur",
	"uz",
	"vi",
	"wo",
	"xh",
	"yo",
	"yo_BJ",
	"yue",
	"yue_Hans",
	"zh",
	"zh_Hant",
	"zh_Hant_HK",
	"zu",
];
pub const UNICODE_VERSION: f32 = 17f32;
pub const UNICODE_RELEASE_TIME: &'static str = "2026-01-07T17:34:16.182535+00:00";
pub mod smileys_emotion {
	pub mod cat_face;
	pub mod emotion;
	pub mod face_affection;
	pub mod face_concerned;
	pub mod face_costume;
	pub mod face_glasses;
	pub mod face_hand;
	pub mod face_hat;
	pub mod face_negative;
	pub mod face_neutral_skeptical;
	pub mod face_sleepy;
	pub mod face_smiling;
	pub mod face_tongue;
	pub mod face_unwell;
	pub mod heart;
	pub mod monkey_face;
}
pub mod people_body {
	pub mod body_parts;
	pub mod family;
	pub mod hand_fingers_closed;
	pub mod hand_fingers_open;
	pub mod hand_fingers_partial;
	pub mod hand_prop;
	pub mod hand_single_finger;
	pub mod hands;
	pub mod person;
	pub mod person_activity;
	pub mod person_fantasy;
	pub mod person_gesture;
	pub mod person_resting;
	pub mod person_role;
	pub mod person_sport;
	pub mod person_symbol;
}
pub mod animals_nature {
	pub mod animal_amphibian;
	pub mod animal_bird;
	pub mod animal_bug;
	pub mod animal_mammal;
	pub mod animal_marine;
	pub mod animal_reptile;
	pub mod plant_flower;
	pub mod plant_other;
}
pub mod food_drink {
	pub mod dishware;
	pub mod drink;
	pub mod food_asian;
	pub mod food_fruit;
	pub mod food_prepared;
	pub mod food_sweet;
	pub mod food_vegetable;
}
pub mod travel_places {
	pub mod hotel;
	pub mod place_building;
	pub mod place_geographic;
	pub mod place_map;
	pub mod place_other;
	pub mod place_religious;
	pub mod sky_weather;
	pub mod time;
	pub mod transport_air;
	pub mod transport_ground;
	pub mod transport_water;
}
pub mod objects {
	pub mod book_paper;
	pub mod clothing;
	pub mod computer;
	pub mod household;
	pub mod light_video;
	pub mod lock;
	pub mod mail;
	pub mod medical;
	pub mod money;
	pub mod music;
	pub mod musical_instrument;
	pub mod office;
	pub mod other_object;
	pub mod phone;
	pub mod science;
	pub mod sound;
	pub mod tool;
	pub mod writing;
}
pub mod symbols {
	pub mod alphanum;
	pub mod arrow;
	pub mod av_symbol;
	pub mod currency;
	pub mod gender;
	pub mod geometric;
	pub mod keycap;
	pub mod math;
	pub mod other_symbol;
	pub mod punctuation;
	pub mod religion;
	pub mod transport_sign;
	pub mod warning;
	pub mod zodiac;
}
pub mod flags {
	pub mod country_flag;
	pub mod flag;
	pub mod subdivision_flag;
}
pub mod activities {
	pub mod arts_crafts;
	pub mod award_medal;
	pub mod event;
	pub mod game;
	pub mod sport;
}
pub mod component {
	pub mod hair_style;
	pub mod other_object;
	pub mod skin_tone;
}
