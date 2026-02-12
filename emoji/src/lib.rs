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
	#[doc = " Skin tone variants for this emoji"]
	pub skin_tones:           Option<&'static [Emoji]>,
	#[doc = " Gender variants for this emoji"]
	pub gender_variants:      Option<&'static [Emoji]>,
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
	SmileysEmotion,
	PeopleBody,
	Component,
	AnimalsNature,
	FoodDrink,
	TravelPlaces,
	Activities,
	Objects,
	Symbols,
	Flags,
}
impl std::fmt::Display for Group {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match self {
			Group::SmileysEmotion => "smileys-emotion",
			Group::PeopleBody => "people-body",
			Group::Component => "component",
			Group::AnimalsNature => "animals-nature",
			Group::FoodDrink => "food-drink",
			Group::TravelPlaces => "travel-places",
			Group::Activities => "activities",
			Group::Objects => "objects",
			Group::Symbols => "symbols",
			Group::Flags => "flags",
		}
		.fmt(f)
	}
}
impl Group {
	pub fn iter() -> impl Iterator<Item = Group> {
		[
			Group::SmileysEmotion,
			Group::PeopleBody,
			Group::Component,
			Group::AnimalsNature,
			Group::FoodDrink,
			Group::TravelPlaces,
			Group::Activities,
			Group::Objects,
			Group::Symbols,
			Group::Flags,
		]
		.iter()
		.copied()
	}
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Subgroup {
	FaceSmiling,
	FaceAffection,
	FaceTongue,
	FaceHand,
	FaceNeutralSkeptical,
	FaceSleepy,
	FaceUnwell,
	FaceHat,
	FaceGlasses,
	FaceConcerned,
	FaceNegative,
	FaceCostume,
	CatFace,
	MonkeyFace,
	Heart,
	Emotion,
	HandFingersOpen,
	HandFingersPartial,
	HandSingleFinger,
	HandFingersClosed,
	Hands,
	HandProp,
	BodyParts,
	Person,
	PersonGesture,
	PersonRole,
	PersonFantasy,
	PersonActivity,
	PersonSport,
	PersonResting,
	Family,
	PersonSymbol,
	SkinTone,
	HairStyle,
	AnimalMammal,
	AnimalBird,
	AnimalAmphibian,
	AnimalReptile,
	AnimalMarine,
	AnimalBug,
	PlantFlower,
	PlantOther,
	FoodFruit,
	FoodVegetable,
	FoodPrepared,
	FoodAsian,
	FoodSweet,
	Drink,
	Dishware,
	PlaceMap,
	PlaceGeographic,
	PlaceBuilding,
	PlaceReligious,
	PlaceOther,
	TransportGround,
	TransportWater,
	TransportAir,
	Hotel,
	Time,
	SkyWeather,
	Event,
	AwardMedal,
	Sport,
	Game,
	ArtsCrafts,
	Clothing,
	Sound,
	Music,
	MusicalInstrument,
	Phone,
	Computer,
	LightVideo,
	BookPaper,
	Money,
	Mail,
	Writing,
	Office,
	Lock,
	Tool,
	Science,
	Medical,
	Household,
	OtherObject,
	TransportSign,
	Warning,
	Arrow,
	Religion,
	Zodiac,
	AvSymbol,
	Gender,
	Math,
	Punctuation,
	Currency,
	OtherSymbol,
	Keycap,
	Alphanum,
	Geometric,
	Flag,
	CountryFlag,
	SubdivisionFlag,
}
impl std::fmt::Display for Subgroup {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match self {
			Subgroup::FaceSmiling => "face-smiling",
			Subgroup::FaceAffection => "face-affection",
			Subgroup::FaceTongue => "face-tongue",
			Subgroup::FaceHand => "face-hand",
			Subgroup::FaceNeutralSkeptical => "face-neutral-skeptical",
			Subgroup::FaceSleepy => "face-sleepy",
			Subgroup::FaceUnwell => "face-unwell",
			Subgroup::FaceHat => "face-hat",
			Subgroup::FaceGlasses => "face-glasses",
			Subgroup::FaceConcerned => "face-concerned",
			Subgroup::FaceNegative => "face-negative",
			Subgroup::FaceCostume => "face-costume",
			Subgroup::CatFace => "cat-face",
			Subgroup::MonkeyFace => "monkey-face",
			Subgroup::Heart => "heart",
			Subgroup::Emotion => "emotion",
			Subgroup::HandFingersOpen => "hand-fingers-open",
			Subgroup::HandFingersPartial => "hand-fingers-partial",
			Subgroup::HandSingleFinger => "hand-single-finger",
			Subgroup::HandFingersClosed => "hand-fingers-closed",
			Subgroup::Hands => "hands",
			Subgroup::HandProp => "hand-prop",
			Subgroup::BodyParts => "body-parts",
			Subgroup::Person => "person",
			Subgroup::PersonGesture => "person-gesture",
			Subgroup::PersonRole => "person-role",
			Subgroup::PersonFantasy => "person-fantasy",
			Subgroup::PersonActivity => "person-activity",
			Subgroup::PersonSport => "person-sport",
			Subgroup::PersonResting => "person-resting",
			Subgroup::Family => "family",
			Subgroup::PersonSymbol => "person-symbol",
			Subgroup::SkinTone => "skin-tone",
			Subgroup::HairStyle => "hair-style",
			Subgroup::AnimalMammal => "animal-mammal",
			Subgroup::AnimalBird => "animal-bird",
			Subgroup::AnimalAmphibian => "animal-amphibian",
			Subgroup::AnimalReptile => "animal-reptile",
			Subgroup::AnimalMarine => "animal-marine",
			Subgroup::AnimalBug => "animal-bug",
			Subgroup::PlantFlower => "plant-flower",
			Subgroup::PlantOther => "plant-other",
			Subgroup::FoodFruit => "food-fruit",
			Subgroup::FoodVegetable => "food-vegetable",
			Subgroup::FoodPrepared => "food-prepared",
			Subgroup::FoodAsian => "food-asian",
			Subgroup::FoodSweet => "food-sweet",
			Subgroup::Drink => "drink",
			Subgroup::Dishware => "dishware",
			Subgroup::PlaceMap => "place-map",
			Subgroup::PlaceGeographic => "place-geographic",
			Subgroup::PlaceBuilding => "place-building",
			Subgroup::PlaceReligious => "place-religious",
			Subgroup::PlaceOther => "place-other",
			Subgroup::TransportGround => "transport-ground",
			Subgroup::TransportWater => "transport-water",
			Subgroup::TransportAir => "transport-air",
			Subgroup::Hotel => "hotel",
			Subgroup::Time => "time",
			Subgroup::SkyWeather => "sky-weather",
			Subgroup::Event => "event",
			Subgroup::AwardMedal => "award-medal",
			Subgroup::Sport => "sport",
			Subgroup::Game => "game",
			Subgroup::ArtsCrafts => "arts-crafts",
			Subgroup::Clothing => "clothing",
			Subgroup::Sound => "sound",
			Subgroup::Music => "music",
			Subgroup::MusicalInstrument => "musical-instrument",
			Subgroup::Phone => "phone",
			Subgroup::Computer => "computer",
			Subgroup::LightVideo => "light-video",
			Subgroup::BookPaper => "book-paper",
			Subgroup::Money => "money",
			Subgroup::Mail => "mail",
			Subgroup::Writing => "writing",
			Subgroup::Office => "office",
			Subgroup::Lock => "lock",
			Subgroup::Tool => "tool",
			Subgroup::Science => "science",
			Subgroup::Medical => "medical",
			Subgroup::Household => "household",
			Subgroup::OtherObject => "other-object",
			Subgroup::TransportSign => "transport-sign",
			Subgroup::Warning => "warning",
			Subgroup::Arrow => "arrow",
			Subgroup::Religion => "religion",
			Subgroup::Zodiac => "zodiac",
			Subgroup::AvSymbol => "av-symbol",
			Subgroup::Gender => "gender",
			Subgroup::Math => "math",
			Subgroup::Punctuation => "punctuation",
			Subgroup::Currency => "currency",
			Subgroup::OtherSymbol => "other-symbol",
			Subgroup::Keycap => "keycap",
			Subgroup::Alphanum => "alphanum",
			Subgroup::Geometric => "geometric",
			Subgroup::Flag => "flag",
			Subgroup::CountryFlag => "country-flag",
			Subgroup::SubdivisionFlag => "subdivision-flag",
		}
		.fmt(f)
	}
}
impl Subgroup {
	pub fn iter() -> impl Iterator<Item = Subgroup> {
		[
			Subgroup::FaceSmiling,
			Subgroup::FaceAffection,
			Subgroup::FaceTongue,
			Subgroup::FaceHand,
			Subgroup::FaceNeutralSkeptical,
			Subgroup::FaceSleepy,
			Subgroup::FaceUnwell,
			Subgroup::FaceHat,
			Subgroup::FaceGlasses,
			Subgroup::FaceConcerned,
			Subgroup::FaceNegative,
			Subgroup::FaceCostume,
			Subgroup::CatFace,
			Subgroup::MonkeyFace,
			Subgroup::Heart,
			Subgroup::Emotion,
			Subgroup::HandFingersOpen,
			Subgroup::HandFingersPartial,
			Subgroup::HandSingleFinger,
			Subgroup::HandFingersClosed,
			Subgroup::Hands,
			Subgroup::HandProp,
			Subgroup::BodyParts,
			Subgroup::Person,
			Subgroup::PersonGesture,
			Subgroup::PersonRole,
			Subgroup::PersonFantasy,
			Subgroup::PersonActivity,
			Subgroup::PersonSport,
			Subgroup::PersonResting,
			Subgroup::Family,
			Subgroup::PersonSymbol,
			Subgroup::SkinTone,
			Subgroup::HairStyle,
			Subgroup::AnimalMammal,
			Subgroup::AnimalBird,
			Subgroup::AnimalAmphibian,
			Subgroup::AnimalReptile,
			Subgroup::AnimalMarine,
			Subgroup::AnimalBug,
			Subgroup::PlantFlower,
			Subgroup::PlantOther,
			Subgroup::FoodFruit,
			Subgroup::FoodVegetable,
			Subgroup::FoodPrepared,
			Subgroup::FoodAsian,
			Subgroup::FoodSweet,
			Subgroup::Drink,
			Subgroup::Dishware,
			Subgroup::PlaceMap,
			Subgroup::PlaceGeographic,
			Subgroup::PlaceBuilding,
			Subgroup::PlaceReligious,
			Subgroup::PlaceOther,
			Subgroup::TransportGround,
			Subgroup::TransportWater,
			Subgroup::TransportAir,
			Subgroup::Hotel,
			Subgroup::Time,
			Subgroup::SkyWeather,
			Subgroup::Event,
			Subgroup::AwardMedal,
			Subgroup::Sport,
			Subgroup::Game,
			Subgroup::ArtsCrafts,
			Subgroup::Clothing,
			Subgroup::Sound,
			Subgroup::Music,
			Subgroup::MusicalInstrument,
			Subgroup::Phone,
			Subgroup::Computer,
			Subgroup::LightVideo,
			Subgroup::BookPaper,
			Subgroup::Money,
			Subgroup::Mail,
			Subgroup::Writing,
			Subgroup::Office,
			Subgroup::Lock,
			Subgroup::Tool,
			Subgroup::Science,
			Subgroup::Medical,
			Subgroup::Household,
			Subgroup::OtherObject,
			Subgroup::TransportSign,
			Subgroup::Warning,
			Subgroup::Arrow,
			Subgroup::Religion,
			Subgroup::Zodiac,
			Subgroup::AvSymbol,
			Subgroup::Gender,
			Subgroup::Math,
			Subgroup::Punctuation,
			Subgroup::Currency,
			Subgroup::OtherSymbol,
			Subgroup::Keycap,
			Subgroup::Alphanum,
			Subgroup::Geometric,
			Subgroup::Flag,
			Subgroup::CountryFlag,
			Subgroup::SubdivisionFlag,
		]
		.iter()
		.copied()
	}
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
pub const UNICODE_RELEASE_TIME: &'static str = "2026-02-12T04:44:17.716880+00:00";
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
pub mod component {
	pub mod hair_style;
	pub mod skin_tone;
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
pub mod activities {
	pub mod arts_crafts;
	pub mod award_medal;
	pub mod event;
	pub mod game;
	pub mod sport;
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
