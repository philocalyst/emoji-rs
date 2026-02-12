#[doc = "🌹"]
pub const ROSE: crate::Emoji = crate::Emoji {
	glyph:                "🌹",
	codepoint:            &[127801u32],
	status:               crate::Status::FullyQualified,
	introduction_version: crate::Version { major: 0u8, minor: 6u8, patch: 0u8 },
	name:                 "rose",
	group:                crate::Group::AnimalsNature,
	subgroup:             crate::Subgroup::PlantFlower,
	is_variant:           false,
	variants:             &[],
	skin_tones:           None,
	gender_variants:      None,
	annotations:          &[
		#[cfg(feature = "bn")]
		crate::Annotation {
			lang:     "bn",
			tts:      Some("গোল\u{9be}প"),
			keywords: &["", "গ\u{9be}ছ", "ফ\u{9c1}ল"],
		},
		#[cfg(feature = "da")]
		crate::Annotation {
			lang:     "da",
			tts:      Some("rose"),
			keywords: &["", "blomst", "kærlighed"],
		},
		#[cfg(feature = "de")]
		crate::Annotation {
			lang:     "de",
			tts:      Some("Rose"),
			keywords: &["", "blume", "blüte", "pflanze", "rose"],
		},
		#[cfg(feature = "en")]
		crate::Annotation {
			lang:     "en",
			tts:      Some("rose"),
			keywords: &["", "beauty", "elegant", "flower", "love", "plant", "red", "valentine"],
		},
		#[cfg(feature = "en-gb")]
		crate::Annotation {
			lang:     "en-gb",
			tts:      Some("rose"),
			keywords: &["", "beauty", "elegant", "flower", "love", "plant", "red", "valentine"],
		},
		#[cfg(feature = "es")]
		crate::Annotation { lang: "es", tts: Some("rosa"), keywords: &["", "flor"] },
		#[cfg(feature = "es-mx")]
		crate::Annotation {
			lang:     "es-mx",
			tts:      Some("rosa"),
			keywords: &["", "amor", "flor", "planta"],
		},
		#[cfg(feature = "et")]
		crate::Annotation {
			lang:     "et",
			tts:      Some("roos"),
			keywords: &["", "armastus", "elegantne", "ilu", "lill", "punane", "taim", "valentin", "õis"],
		},
		#[cfg(feature = "fi")]
		crate::Annotation {
			lang:     "fi",
			tts:      Some("ruusu"),
			keywords: &["", "kasvi", "kukka"],
		},
		#[cfg(feature = "fr")]
		crate::Annotation {
			lang:     "fr",
			tts:      Some("rose"),
			keywords: &["", "amour", "beauté", "fleur", "plante", "rouge", "saint-valentin"],
		},
		#[cfg(feature = "hi")]
		crate::Annotation {
			lang:     "hi",
			tts:      Some("ग\u{941}लाब"),
			keywords: &["", "फ\u{942}ल"],
		},
		#[cfg(feature = "hu")]
		crate::Annotation {
			lang:     "hu",
			tts:      Some("rózsa"),
			keywords: &[
				"",
				"növény",
				"romantika",
				"szenvedély",
				"szerelem",
				"szerelmes",
				"virág",
				"vörös rózsa",
			],
		},
		#[cfg(feature = "it")]
		crate::Annotation {
			lang:     "it",
			tts:      Some("rosa"),
			keywords: &[
				"",
				"amore",
				"bellezza",
				"fiore",
				"giardino",
				"natura",
				"pianta",
				"regalo",
				"rosa rossa",
				"rosso",
			],
		},
		#[cfg(feature = "ja")]
		crate::Annotation {
			lang:     "ja",
			tts:      Some("バラ"),
			keywords: &["", "ローズ", "植物", "花", "薔薇", "赤いバラ"],
		},
		#[cfg(feature = "ko")]
		crate::Annotation {
			lang:     "ko",
			tts:      Some("장미꽃"),
			keywords: &["", "꽃", "발레타인", "사랑", "식물", "아름다움", "우아한"],
		},
		#[cfg(feature = "lt")]
		crate::Annotation {
			lang:     "lt",
			tts:      Some("rožė"),
			keywords: &["", "augalas", "gėlė", "raudona", "valentino diena"],
		},
		#[cfg(feature = "ms")]
		crate::Annotation {
			lang:     "ms",
			tts:      Some("bunga mawar"),
			keywords: &["", "bunga", "merah", "tumbuhan", "valentine"],
		},
		#[cfg(feature = "nb")]
		crate::Annotation { lang: "nb", tts: Some("rose"), keywords: &["", "blomst"] },
		#[cfg(feature = "nl")]
		crate::Annotation {
			lang:     "nl",
			tts:      Some("roos"),
			keywords: &["", "bloem", "elegant", "liefde", "plant", "rood", "schoonheid", "valentijn"],
		},
		#[cfg(feature = "pl")]
		crate::Annotation {
			lang:     "pl",
			tts:      Some("róża"),
			keywords: &["", "czerwony", "kwiat", "miłość", "roślina"],
		},
		#[cfg(feature = "pt")]
		crate::Annotation { lang: "pt", tts: Some("rosa"), keywords: &["", "flor"] },
		#[cfg(feature = "ru")]
		crate::Annotation {
			lang:     "ru",
			tts:      Some("роза"),
			keywords: &[
				"",
				"бутон",
				"валентинка",
				"красная",
				"красота",
				"любовь",
				"растение",
				"цветок",
				"элегантность",
			],
		},
		#[cfg(feature = "sv")]
		crate::Annotation { lang: "sv", tts: Some("ros"), keywords: &["", "blomma"] },
		#[cfg(feature = "th")]
		crate::Annotation {
			lang:     "th",
			tts:      Some("ดอกก\u{e38}หลาบ"),
			keywords: &["", "ก\u{e38}หลาบแดง", "ดอกไม\u{e49}"],
		},
		#[cfg(feature = "uk")]
		crate::Annotation {
			lang:     "uk",
			tts:      Some("троянда"),
			keywords: &["", "квітка", "рослина", "червона троянда"],
		},
		#[cfg(feature = "vi")]
		crate::Annotation {
			lang:     "vi",
			tts:      Some("hoa hồng"),
			keywords: &["", "cây", "hoa", "màu đỏ", "sang trọng", "tình yêu", "valentine", "vẻ đẹp"],
		},
		#[cfg(feature = "zh")]
		crate::Annotation {
			lang:     "zh",
			tts:      Some("玫瑰"),
			keywords: &["", "优雅", "红玫瑰", "花"],
		},
		#[cfg(feature = "zh-hant")]
		crate::Annotation {
			lang:     "zh-hant",
			tts:      Some("玫瑰"),
			keywords: &["", "紅玫瑰", "花"],
		},
	],
};
#[doc = "🌺"]
pub const HIBISCUS: crate::Emoji = crate::Emoji {
	glyph:                "🌺",
	codepoint:            &[127802u32],
	status:               crate::Status::FullyQualified,
	introduction_version: crate::Version { major: 0u8, minor: 6u8, patch: 0u8 },
	name:                 "hibiscus",
	group:                crate::Group::AnimalsNature,
	subgroup:             crate::Subgroup::PlantFlower,
	is_variant:           false,
	variants:             &[],
	skin_tones:           None,
	gender_variants:      None,
	annotations:          &[
		#[cfg(feature = "bn")]
		crate::Annotation {
			lang:     "bn",
			tts:      Some("জব\u{9be}"),
			keywords: &["", "গ\u{9be}ছ", "ফ\u{9c1}ল"],
		},
		#[cfg(feature = "da")]
		crate::Annotation {
			lang:     "da",
			tts:      Some("hibiscus"),
			keywords: &["", "blomst", "hawaiiblomst"],
		},
		#[cfg(feature = "de")]
		crate::Annotation {
			lang:     "de",
			tts:      Some("Hibiskus"),
			keywords: &["", "blume", "blüte", "hibiskus", "pflanze", "rosa"],
		},
		#[cfg(feature = "en")]
		crate::Annotation {
			lang:     "en",
			tts:      Some("hibiscus"),
			keywords: &["", "flower", "plant"],
		},
		#[cfg(feature = "en-gb")]
		crate::Annotation {
			lang:     "en-gb",
			tts:      Some("hibiscus"),
			keywords: &["", "flower", "plant"],
		},
		#[cfg(feature = "es")]
		crate::Annotation {
			lang:     "es",
			tts:      Some("flor de hibisco"),
			keywords: &["", "flor", "hibisco"],
		},
		#[cfg(feature = "es-mx")]
		crate::Annotation {
			lang:     "es-mx",
			tts:      Some("flor de hibisco"),
			keywords: &["", "flor", "hibisco"],
		},
		#[cfg(feature = "et")]
		crate::Annotation { lang: "et", tts: Some("hibisk"), keywords: &["", "lill", "taim"] },
		#[cfg(feature = "fi")]
		crate::Annotation {
			lang:     "fi",
			tts:      Some("kiinanruusu"),
			keywords: &["", "kasvi", "kukka"],
		},
		#[cfg(feature = "fr")]
		crate::Annotation {
			lang:     "fr",
			tts:      Some("hibiscus"),
			keywords: &["", "fleur", "plante"],
		},
		#[cfg(feature = "hi")]
		crate::Annotation {
			lang:     "hi",
			tts:      Some("ग\u{941}ड\u{93c}हल, हिबिस\u{94d}कस"),
			keywords: &[
				"",
				"अड\u{93c}ह\u{941}ल",
				"ग\u{941}ड\u{93c}हल",
				"जपाप\u{941}ष\u{94d}प",
				"फ\u{942}ल",
				"हिबिस\u{94d}कस",
			],
		},
		#[cfg(feature = "hu")]
		crate::Annotation {
			lang:     "hu",
			tts:      Some("hibiszkusz"),
			keywords: &["", "növény", "virág"],
		},
		#[cfg(feature = "it")]
		crate::Annotation {
			lang:     "it",
			tts:      Some("ibisco"),
			keywords: &["", "fiore", "natura", "pianta", "rosso"],
		},
		#[cfg(feature = "ja")]
		crate::Annotation {
			lang:     "ja",
			tts:      Some("ハイビスカス"),
			keywords: &["", "植物", "花"],
		},
		#[cfg(feature = "ko")]
		crate::Annotation {
			lang:     "ko",
			tts:      Some("무궁화"),
			keywords: &["", "꽃", "식물", "히비스커스"],
		},
		#[cfg(feature = "lt")]
		crate::Annotation {
			lang:     "lt",
			tts:      Some("kinrožė"),
			keywords: &["", "augalas", "gėlė"],
		},
		#[cfg(feature = "ms")]
		crate::Annotation {
			lang:     "ms",
			tts:      Some("bunga raya"),
			keywords: &["", "bunga", "tumbuhan"],
		},
		#[cfg(feature = "nb")]
		crate::Annotation { lang: "nb", tts: Some("hibiskus"), keywords: &["", "blomst"] },
		#[cfg(feature = "nl")]
		crate::Annotation {
			lang:     "nl",
			tts:      Some("hibiscus"),
			keywords: &["", "bloem", "plant"],
		},
		#[cfg(feature = "pl")]
		crate::Annotation {
			lang:     "pl",
			tts:      Some("kwiat hibiskusa"),
			keywords: &["", "hibiskus", "hibiskusa", "kwiat", "roślina"],
		},
		#[cfg(feature = "pt")]
		crate::Annotation {
			lang:     "pt",
			tts:      Some("hibisco"),
			keywords: &["", "flor", "planta", "primavera"],
		},
		#[cfg(feature = "ru")]
		crate::Annotation {
			lang:     "ru",
			tts:      Some("гибискус"),
			keywords: &["", "китайская роза", "цветок"],
		},
		#[cfg(feature = "sv")]
		crate::Annotation {
			lang:     "sv",
			tts:      Some("hibiskus"),
			keywords: &["", "blomma", "hibiskusblomma"],
		},
		#[cfg(feature = "th")]
		crate::Annotation {
			lang:     "th",
			tts:      Some("ดอกชบา"),
			keywords: &["", "ชบา", "ดอกไม\u{e49}"],
		},
		#[cfg(feature = "uk")]
		crate::Annotation {
			lang:     "uk",
			tts:      Some("гібіскус"),
			keywords: &["", "квітка", "рослина"],
		},
		#[cfg(feature = "vi")]
		crate::Annotation {
			lang:     "vi",
			tts:      Some("hoa dâm bụt"),
			keywords: &["", "hoa", "thực vật"],
		},
		#[cfg(feature = "zh")]
		crate::Annotation {
			lang:     "zh",
			tts:      Some("芙蓉"),
			keywords: &["", "木槿", "植物", "花"],
		},
		#[cfg(feature = "zh-hant")]
		crate::Annotation { lang: "zh-hant", tts: Some("芙蓉"), keywords: &["", "花"] },
	],
};
#[doc = "🌷"]
pub const TULIP: crate::Emoji = crate::Emoji {
	glyph:                "🌷",
	codepoint:            &[127799u32],
	status:               crate::Status::FullyQualified,
	introduction_version: crate::Version { major: 0u8, minor: 6u8, patch: 0u8 },
	name:                 "tulip",
	group:                crate::Group::AnimalsNature,
	subgroup:             crate::Subgroup::PlantFlower,
	is_variant:           false,
	variants:             &[],
	skin_tones:           None,
	gender_variants:      None,
	annotations:          &[
		#[cfg(feature = "bn")]
		crate::Annotation {
			lang:     "bn",
			tts:      Some("টিউলিপ"),
			keywords: &["", "গ\u{9be}ছ", "ফ\u{9c1}ল"],
		},
		#[cfg(feature = "da")]
		crate::Annotation { lang: "da", tts: Some("tulipan"), keywords: &["", "blomst"] },
		#[cfg(feature = "de")]
		crate::Annotation {
			lang:     "de",
			tts:      Some("Tulpe"),
			keywords: &["", "blume", "blüte", "pflanze", "tulpe"],
		},
		#[cfg(feature = "en")]
		crate::Annotation {
			lang:     "en",
			tts:      Some("tulip"),
			keywords: &["", "blossom", "flower", "growth", "plant"],
		},
		#[cfg(feature = "en-gb")]
		crate::Annotation {
			lang:     "en-gb",
			tts:      Some("tulip"),
			keywords: &["", "blossom", "flower", "growth", "plant"],
		},
		#[cfg(feature = "es")]
		crate::Annotation { lang: "es", tts: Some("tulipán"), keywords: &["", "flor"] },
		#[cfg(feature = "es-mx")]
		crate::Annotation { lang: "es-mx", tts: Some("tulipán"), keywords: &["", "flor"] },
		#[cfg(feature = "et")]
		crate::Annotation {
			lang:     "et",
			tts:      Some("tulp"),
			keywords: &["", "kasv", "lill", "taim", "õis"],
		},
		#[cfg(feature = "fi")]
		crate::Annotation {
			lang:     "fi",
			tts:      Some("tulppaani"),
			keywords: &["", "kasvi", "kukka"],
		},
		#[cfg(feature = "fr")]
		crate::Annotation {
			lang:     "fr",
			tts:      Some("tulipe"),
			keywords: &["", "bourgeon", "fleur", "plante", "printemps"],
		},
		#[cfg(feature = "hi")]
		crate::Annotation {
			lang:     "hi",
			tts:      Some("ट\u{94d}य\u{942}लिप"),
			keywords: &["", "फ\u{942}ल"],
		},
		#[cfg(feature = "hu")]
		crate::Annotation {
			lang:     "hu",
			tts:      Some("tulipán"),
			keywords: &["", "növény", "virág"],
		},
		#[cfg(feature = "it")]
		crate::Annotation {
			lang:     "it",
			tts:      Some("tulipano"),
			keywords: &["", "fiore", "giardino", "natura", "pianta", "rosa"],
		},
		#[cfg(feature = "ja")]
		crate::Annotation {
			lang:     "ja",
			tts:      Some("チューリップ"),
			keywords: &["", "植物", "花", "開花"],
		},
		#[cfg(feature = "ko")]
		crate::Annotation {
			lang:     "ko",
			tts:      Some("튤립"),
			keywords: &["", "꽃", "꽃이 피는", "식물"],
		},
		#[cfg(feature = "lt")]
		crate::Annotation {
			lang:     "lt",
			tts:      Some("tulpė"),
			keywords: &["", "augalas", "gėlė"],
		},
		#[cfg(feature = "ms")]
		crate::Annotation {
			lang:     "ms",
			tts:      Some("bunga tulip"),
			keywords: &["", "bunga", "pertumbuhan", "tulip", "tumbuhan"],
		},
		#[cfg(feature = "nb")]
		crate::Annotation { lang: "nb", tts: Some("tulipan"), keywords: &["", "blomst"] },
		#[cfg(feature = "nl")]
		crate::Annotation {
			lang:     "nl",
			tts:      Some("tulp"),
			keywords: &["", "bloem", "bloesem", "groei", "plant"],
		},
		#[cfg(feature = "pl")]
		crate::Annotation {
			lang:     "pl",
			tts:      Some("tulipan"),
			keywords: &["", "kwiat", "pąk", "roślina"],
		},
		#[cfg(feature = "pt")]
		crate::Annotation { lang: "pt", tts: Some("tulipa"), keywords: &["", "flor"] },
		#[cfg(feature = "ru")]
		crate::Annotation {
			lang:     "ru",
			tts:      Some("тюльпан"),
			keywords: &["", "цвести", "цветок"],
		},
		#[cfg(feature = "sv")]
		crate::Annotation { lang: "sv", tts: Some("tulpan"), keywords: &["", "blomma"] },
		#[cfg(feature = "th")]
		crate::Annotation {
			lang:     "th",
			tts:      Some("ท\u{e34}วล\u{e34}ป"),
			keywords: &["", "ดอกไม\u{e49}"],
		},
		#[cfg(feature = "uk")]
		crate::Annotation {
			lang:     "uk",
			tts:      Some("тюльпан"),
			keywords: &["", "квітка", "рожевий тюльпан", "рослина"],
		},
		#[cfg(feature = "vi")]
		crate::Annotation {
			lang:     "vi",
			tts:      Some("hoa tulip"),
			keywords: &["", "hoa", "thực vật"],
		},
		#[cfg(feature = "zh")]
		crate::Annotation { lang: "zh", tts: Some("郁金香"), keywords: &["", "开花", "花"] },
		#[cfg(feature = "zh-hant")]
		crate::Annotation { lang: "zh-hant", tts: Some("鬱金香"), keywords: &["", "花"] },
	],
};
#[doc = "🥀"]
pub const WILTED_FLOWER: crate::Emoji = crate::Emoji {
	glyph:                "🥀",
	codepoint:            &[129344u32],
	status:               crate::Status::FullyQualified,
	introduction_version: crate::Version { major: 3u8, minor: 0u8, patch: 0u8 },
	name:                 "wilted flower",
	group:                crate::Group::AnimalsNature,
	subgroup:             crate::Subgroup::PlantFlower,
	is_variant:           false,
	variants:             &[],
	skin_tones:           None,
	gender_variants:      None,
	annotations:          &[
		#[cfg(feature = "bn")]
		crate::Annotation {
			lang:     "bn",
			tts:      Some("নেত\u{9be}নো ফ\u{9c1}ল"),
			keywords: &["", "নেত\u{9be}নো", "ফ\u{9c1}ল"],
		},
		#[cfg(feature = "da")]
		crate::Annotation {
			lang:     "da",
			tts:      Some("vissen blomst"),
			keywords: &["", "afblomstret", "blomst", "visnet", "vissen"],
		},
		#[cfg(feature = "de")]
		crate::Annotation {
			lang:     "de",
			tts:      Some("welke Blume"),
			keywords: &[
				"",
				"am sterben",
				"blume",
				"sterbend",
				"verwelkt",
				"welk",
				"welke blume",
				"welkende blume",
			],
		},
		#[cfg(feature = "en")]
		crate::Annotation {
			lang:     "en",
			tts:      Some("wilted flower"),
			keywords: &["", "dying", "flower", "wilted"],
		},
		#[cfg(feature = "en-gb")]
		crate::Annotation {
			lang:     "en-gb",
			tts:      Some("wilted flower"),
			keywords: &["", "dying", "flower", "wilted"],
		},
		#[cfg(feature = "es")]
		crate::Annotation {
			lang:     "es",
			tts:      Some("flor marchita"),
			keywords: &["", "flor", "marchita", "marchitada", "marchitarse"],
		},
		#[cfg(feature = "es-mx")]
		crate::Annotation {
			lang:     "es-mx",
			tts:      Some("flor marchita"),
			keywords: &["", "flor", "marchita", "marchitada", "marchitarse"],
		},
		#[cfg(feature = "et")]
		crate::Annotation {
			lang:     "et",
			tts:      Some("närbunud lill"),
			keywords: &["", "lill", "närbunud", "suremas"],
		},
		#[cfg(feature = "fi")]
		crate::Annotation {
			lang:     "fi",
			tts:      Some("nuutunut kukka"),
			keywords: &["", "kukka", "nuutunut"],
		},
		#[cfg(feature = "fr")]
		crate::Annotation {
			lang:     "fr",
			tts:      Some("fleur fanée"),
			keywords: &["", "fanée", "fleur", "mort", "rose"],
		},
		#[cfg(feature = "hi")]
		crate::Annotation {
			lang:     "hi",
			tts:      Some("म\u{941}रझाया ह\u{941}आ फ\u{942}ल"),
			keywords: &["", "फ\u{942}ल", "म\u{941}रझाया"],
		},
		#[cfg(feature = "hu")]
		crate::Annotation {
			lang:     "hu",
			tts:      Some("hervadt virág"),
			keywords: &["", "hervadt", "rózsa", "virág"],
		},
		#[cfg(feature = "it")]
		crate::Annotation {
			lang:     "it",
			tts:      Some("fiore appassito"),
			keywords: &[
				"",
				"appassito",
				"fiore",
				"natura",
				"pianta",
				"rosa",
				"rosa appassita",
				"sfiorire",
				"sfiorita",
			],
		},
		#[cfg(feature = "ja")]
		crate::Annotation {
			lang:     "ja",
			tts:      Some("しおれた花"),
			keywords: &["", "しおれた", "花"],
		},
		#[cfg(feature = "ko")]
		crate::Annotation {
			lang:     "ko",
			tts:      Some("시든 꽃"),
			keywords: &["", "꺾인 고개", "꽃", "시든", "시들어 가는", "시듦"],
		},
		#[cfg(feature = "lt")]
		crate::Annotation {
			lang:     "lt",
			tts:      Some("gležna gėlė"),
			keywords: &["", "augalas", "gležna", "gėlė"],
		},
		#[cfg(feature = "ms")]
		crate::Annotation {
			lang:     "ms",
			tts:      Some("bunga layu"),
			keywords: &["", "bunga", "layu"],
		},
		#[cfg(feature = "nb")]
		crate::Annotation {
			lang:     "nb",
			tts:      Some("vissen blomst"),
			keywords: &["", "blomst", "tørket", "vissen"],
		},
		#[cfg(feature = "nl")]
		crate::Annotation {
			lang:     "nl",
			tts:      Some("verwelkte bloem"),
			keywords: &["", "bloem", "sterven", "verwelkt"],
		},
		#[cfg(feature = "pl")]
		crate::Annotation {
			lang:     "pl",
			tts:      Some("zwiędły kwiat"),
			keywords: &[
				"",
				"kwiat",
				"płatki",
				"roślina",
				"róża",
				"smutek",
				"smutny",
				"więdnąć",
				"zwiędły",
			],
		},
		#[cfg(feature = "pt")]
		crate::Annotation {
			lang:     "pt",
			tts:      Some("flor murcha"),
			keywords: &["", "flor", "morrendo", "murcha", "murchando"],
		},
		#[cfg(feature = "ru")]
		crate::Annotation {
			lang:     "ru",
			tts:      Some("увядший цветок"),
			keywords: &["", "завяла", "лепесток", "роза", "увядший", "цветок"],
		},
		#[cfg(feature = "sv")]
		crate::Annotation {
			lang:     "sv",
			tts:      Some("vissnad blomma"),
			keywords: &["", "blomma", "torkad", "vissen"],
		},
		#[cfg(feature = "th")]
		crate::Annotation {
			lang:     "th",
			tts:      Some("ดอกไม\u{e49}เห\u{e35}\u{e48}ยว"),
			keywords: &["", "ก\u{e38}หลาบ", "ดอกไม\u{e49}", "เฉา", "เห\u{e35}\u{e48}ยว"],
		},
		#[cfg(feature = "uk")]
		crate::Annotation {
			lang:     "uk",
			tts:      Some("привʼяла троянда"),
			keywords: &["", "завʼяла троянда", "зів’яла", "зів’яла троянда", "квітка"],
		},
		#[cfg(feature = "vi")]
		crate::Annotation { lang: "vi", tts: Some("hoa héo"), keywords: &["", "hoa", "héo"] },
		#[cfg(feature = "zh")]
		crate::Annotation {
			lang:     "zh",
			tts:      Some("枯萎的花"),
			keywords: &["", "凋谢", "枯萎", "花"],
		},
		#[cfg(feature = "zh-hant")]
		crate::Annotation {
			lang:     "zh-hant",
			tts:      Some("枯萎花朵"),
			keywords: &["", "凋零", "枯萎", "花"],
		},
	],
};
#[doc = "🏵\u{fe0f}"]
pub const ROSETTE: crate::Emoji = crate::Emoji {
	glyph:                "🏵\u{fe0f}",
	codepoint:            &[127989u32],
	status:               crate::Status::FullyQualified,
	introduction_version: crate::Version { major: 0u8, minor: 7u8, patch: 0u8 },
	name:                 "rosette",
	group:                crate::Group::AnimalsNature,
	subgroup:             crate::Subgroup::PlantFlower,
	is_variant:           false,
	variants:             &[],
	skin_tones:           None,
	gender_variants:      None,
	annotations:          &[
		#[cfg(feature = "bn")]
		crate::Annotation {
			lang:     "bn",
			tts:      Some("ফিতে দিয\u{9bc}ে তৈরি গোল\u{9be}পের ব\u{9cd}য\u{9be}জ"),
			keywords: &["", "গ\u{9be}ছ", "ফ\u{9c1}ল"],
		},
		#[cfg(feature = "da")]
		crate::Annotation { lang: "da", tts: Some("roset"), keywords: &["", "plante"] },
		#[cfg(feature = "de")]
		crate::Annotation {
			lang:     "de",
			tts:      Some("Rosette"),
			keywords: &["", "pflanze", "rosette"],
		},
		#[cfg(feature = "en")]
		crate::Annotation { lang: "en", tts: Some("rosette"), keywords: &["", "plant"] },
		#[cfg(feature = "en-gb")]
		crate::Annotation { lang: "en-gb", tts: Some("rosette"), keywords: &["", "plant"] },
		#[cfg(feature = "es")]
		crate::Annotation {
			lang:     "es",
			tts:      Some("roseta"),
			keywords: &["", "flor", "planta"],
		},
		#[cfg(feature = "es-mx")]
		crate::Annotation {
			lang:     "es-mx",
			tts:      Some("roseta"),
			keywords: &["", "flor", "planta"],
		},
		#[cfg(feature = "et")]
		crate::Annotation { lang: "et", tts: Some("rosett"), keywords: &["", "lill", "taim"] },
		#[cfg(feature = "fi")]
		crate::Annotation { lang: "fi", tts: Some("ruusuke"), keywords: &["", "kasvi"] },
		#[cfg(feature = "fr")]
		crate::Annotation { lang: "fr", tts: Some("rosette"), keywords: &["", "plante"] },
		#[cfg(feature = "hi")]
		crate::Annotation {
			lang:     "hi",
			tts:      Some("प\u{941}ष\u{94d}प, रिबन का प\u{941}ष\u{94d}प"),
			keywords: &[
				"",
				"ग\u{941}लाबवत\u{94d}",
				"प\u{941}ष\u{94d}प",
				"पौध\u{947}",
				"रिबन का प\u{941}ष\u{94d}प",
			],
		},
		#[cfg(feature = "hu")]
		crate::Annotation {
			lang:     "hu",
			tts:      Some("rozetta"),
			keywords: &["", "növény", "virág"],
		},
		#[cfg(feature = "it")]
		crate::Annotation {
			lang:     "it",
			tts:      Some("rosetta"),
			keywords: &["", "coccarda", "fiore", "natura", "pianta"],
		},
		#[cfg(feature = "ja")]
		crate::Annotation { lang: "ja", tts: Some("花飾り"), keywords: &["", "植物", "花"] },
		#[cfg(feature = "ko")]
		crate::Annotation {
			lang:     "ko",
			tts:      Some("장미"),
			keywords: &["", "꽃", "로제트", "식물", "장미 모양"],
		},
		#[cfg(feature = "lt")]
		crate::Annotation {
			lang:     "lt",
			tts:      Some("rozetė"),
			keywords: &["", "augalas", "gėlė", "žiedas"],
		},
		#[cfg(feature = "ms")]
		crate::Annotation { lang: "ms", tts: Some("roset"), keywords: &["", "tumbuhan"] },
		#[cfg(feature = "nb")]
		crate::Annotation { lang: "nb", tts: Some("rosett"), keywords: &["", "plante"] },
		#[cfg(feature = "nl")]
		crate::Annotation { lang: "nl", tts: Some("rozet"), keywords: &["", "plant"] },
		#[cfg(feature = "pl")]
		crate::Annotation {
			lang:     "pl",
			tts:      Some("rozeta"),
			keywords: &["", "kwiat", "roślina"],
		},
		#[cfg(feature = "pt")]
		crate::Annotation {
			lang:     "pt",
			tts:      Some("roseta"),
			keywords: &["", "flor", "flor amarela", "planta", "primavera"],
		},
		#[cfg(feature = "ru")]
		crate::Annotation {
			lang:     "ru",
			tts:      Some("розетка"),
			keywords: &["", "растение", "розочка", "цветок"],
		},
		#[cfg(feature = "sv")]
		crate::Annotation { lang: "sv", tts: Some("bandros"), keywords: &["", "blomma"] },
		#[cfg(feature = "th")]
		crate::Annotation {
			lang:     "th",
			tts:      Some("ลายดอกก\u{e38}หลาบ"),
			keywords: &["", "ดอกไม\u{e49}"],
		},
		#[cfg(feature = "uk")]
		crate::Annotation {
			lang:     "uk",
			tts:      Some("розета"),
			keywords: &["", "бутон", "квітка", "розетка", "рослина", "чорнобривець"],
		},
		#[cfg(feature = "vi")]
		crate::Annotation {
			lang:     "vi",
			tts:      Some("nơ hoa hồng"),
			keywords: &["", "thực vật"],
		},
		#[cfg(feature = "zh")]
		crate::Annotation {
			lang:     "zh",
			tts:      Some("圆形花饰"),
			keywords: &["", "光荣花", "植物", "花", "花圈"],
		},
		#[cfg(feature = "zh-hant")]
		crate::Annotation {
			lang:     "zh-hant",
			tts:      Some("花朵"),
			keywords: &["", "玫瑰花圖案", "花"],
		},
	],
};
#[doc = "🌻"]
pub const SUNFLOWER: crate::Emoji = crate::Emoji {
	glyph:                "🌻",
	codepoint:            &[127803u32],
	status:               crate::Status::FullyQualified,
	introduction_version: crate::Version { major: 0u8, minor: 6u8, patch: 0u8 },
	name:                 "sunflower",
	group:                crate::Group::AnimalsNature,
	subgroup:             crate::Subgroup::PlantFlower,
	is_variant:           false,
	variants:             &[],
	skin_tones:           None,
	gender_variants:      None,
	annotations:          &[
		#[cfg(feature = "bn")]
		crate::Annotation {
			lang:     "bn",
			tts:      Some("স\u{9c2}র\u{9cd}যম\u{9c1}খি"),
			keywords: &["", "গ\u{9be}ছ", "ফ\u{9c1}ল", "স\u{9c2}র\u{9cd}য"],
		},
		#[cfg(feature = "da")]
		crate::Annotation { lang: "da", tts: Some("solsikke"), keywords: &["", "blomst"] },
		#[cfg(feature = "de")]
		crate::Annotation {
			lang:     "de",
			tts:      Some("Sonnenblume"),
			keywords: &["", "blume", "blüte", "pflanze", "sonne", "sonnenblume"],
		},
		#[cfg(feature = "en")]
		crate::Annotation {
			lang:     "en",
			tts:      Some("sunflower"),
			keywords: &["", "flower", "outdoors", "plant", "sun"],
		},
		#[cfg(feature = "en-gb")]
		crate::Annotation {
			lang:     "en-gb",
			tts:      Some("sunflower"),
			keywords: &["", "flower", "outdoors", "plant", "sun"],
		},
		#[cfg(feature = "es")]
		crate::Annotation { lang: "es", tts: Some("girasol"), keywords: &["", "flor", "sol"] },
		#[cfg(feature = "es-mx")]
		crate::Annotation {
			lang:     "es-mx",
			tts:      Some("girasol"),
			keywords: &["", "flor", "planta"],
		},
		#[cfg(feature = "et")]
		crate::Annotation {
			lang:     "et",
			tts:      Some("päevalill"),
			keywords: &["", "lill", "päike", "taim"],
		},
		#[cfg(feature = "fi")]
		crate::Annotation {
			lang:     "fi",
			tts:      Some("auringonkukka"),
			keywords: &["", "aurinko", "kasvi", "kukka"],
		},
		#[cfg(feature = "fr")]
		crate::Annotation {
			lang:     "fr",
			tts:      Some("tournesol"),
			keywords: &["", "champs", "fleur", "plante", "soleil"],
		},
		#[cfg(feature = "hi")]
		crate::Annotation {
			lang:     "hi",
			tts:      Some("स\u{942}र\u{94d}यम\u{941}खी"),
			keywords: &["", "फ\u{942}ल", "सनफ\u{93c}\u{94d}लॉवर"],
		},
		#[cfg(feature = "hu")]
		crate::Annotation {
			lang:     "hu",
			tts:      Some("napraforgó"),
			keywords: &["", "nap", "növény", "virág"],
		},
		#[cfg(feature = "it")]
		crate::Annotation {
			lang:     "it",
			tts:      Some("girasole"),
			keywords: &["", "fiore", "giallo", "natura", "pianta"],
		},
		#[cfg(feature = "ja")]
		crate::Annotation {
			lang:     "ja",
			tts:      Some("ヒマワリ"),
			keywords: &["", "ひまわり", "花"],
		},
		#[cfg(feature = "ko")]
		crate::Annotation {
			lang:     "ko",
			tts:      Some("해바라기"),
			keywords: &["", "꽃", "식물", "야외", "해"],
		},
		#[cfg(feature = "lt")]
		crate::Annotation {
			lang:     "lt",
			tts:      Some("saulėgrąža"),
			keywords: &["", "augalas", "gėlė", "saulė"],
		},
		#[cfg(feature = "ms")]
		crate::Annotation {
			lang:     "ms",
			tts:      Some("bunga matahari"),
			keywords: &["", "bunga", "matahari", "tumbuhan"],
		},
		#[cfg(feature = "nb")]
		crate::Annotation {
			lang:     "nb",
			tts:      Some("solsikke"),
			keywords: &["", "blomst", "sol"],
		},
		#[cfg(feature = "nl")]
		crate::Annotation {
			lang:     "nl",
			tts:      Some("zonnebloem"),
			keywords: &["", "bloem", "buiten", "plant", "zon"],
		},
		#[cfg(feature = "pl")]
		crate::Annotation {
			lang:     "pl",
			tts:      Some("słonecznik"),
			keywords: &["", "kwiat", "roślina", "słońce"],
		},
		#[cfg(feature = "pt")]
		crate::Annotation {
			lang:     "pt",
			tts:      Some("girassol"),
			keywords: &["", "flor", "planta"],
		},
		#[cfg(feature = "ru")]
		crate::Annotation {
			lang:     "ru",
			tts:      Some("подсолнух"),
			keywords: &["", "солнце", "цветок"],
		},
		#[cfg(feature = "sv")]
		crate::Annotation { lang: "sv", tts: Some("solros"), keywords: &["", "blomma"] },
		#[cfg(feature = "th")]
		crate::Annotation {
			lang:     "th",
			tts:      Some("ดอกทานตะว\u{e31}น"),
			keywords: &["", "ดอกไม\u{e49}", "ทานตะว\u{e31}น"],
		},
		#[cfg(feature = "uk")]
		crate::Annotation {
			lang:     "uk",
			tts:      Some("соняшник"),
			keywords: &["", "квітка", "рослина", "сонце", "сонях"],
		},
		#[cfg(feature = "vi")]
		crate::Annotation {
			lang:     "vi",
			tts:      Some("hoa hướng dương"),
			keywords: &["", "hoa", "mặt trời", "thực vật"],
		},
		#[cfg(feature = "zh")]
		crate::Annotation {
			lang:     "zh",
			tts:      Some("向日葵"),
			keywords: &["", "太阳", "太阳花", "花"],
		},
		#[cfg(feature = "zh-hant")]
		crate::Annotation { lang: "zh-hant", tts: Some("向日葵"), keywords: &["", "花"] },
	],
};
#[doc = "💐"]
pub const BOUQUET: crate::Emoji = crate::Emoji {
	glyph:                "💐",
	codepoint:            &[128144u32],
	status:               crate::Status::FullyQualified,
	introduction_version: crate::Version { major: 0u8, minor: 6u8, patch: 0u8 },
	name:                 "bouquet",
	group:                crate::Group::AnimalsNature,
	subgroup:             crate::Subgroup::PlantFlower,
	is_variant:           false,
	variants:             &[],
	skin_tones:           None,
	gender_variants:      None,
	annotations:          &[
		#[cfg(feature = "bn")]
		crate::Annotation {
			lang:     "bn",
			tts:      Some("ফ\u{9c1}লের তোড\u{9bc}\u{9be}"),
			keywords: &[
				"",
				"অ\u{9cd}য\u{9be}নিভ\u{9be}র\u{9cd}স\u{9be}রি",
				"গ\u{9be}ছ",
				"প\u{9cd}রেম",
				"প\u{9cd}রেমে পড\u{9bc}ে ফ\u{9c1}ল",
				"ফ\u{9c1}ল",
				"ব\u{9be}র\u{9cd}ষিকী",
				"রোম\u{9be}ন\u{9cd}স",
			],
		},
		#[cfg(feature = "da")]
		crate::Annotation {
			lang:     "da",
			tts:      Some("blomsterbuket"),
			keywords: &["", "blomster", "buket"],
		},
		#[cfg(feature = "de")]
		crate::Annotation {
			lang:     "de",
			tts:      Some("Blumenstrauß"),
			keywords: &["", "blumen", "blumenstrauß", "bouquet"],
		},
		#[cfg(feature = "en")]
		crate::Annotation {
			lang:     "en",
			tts:      Some("bouquet"),
			keywords: &["", "anniversary", "birthday", "date", "flower", "love", "plant", "romance"],
		},
		#[cfg(feature = "en-gb")]
		crate::Annotation {
			lang:     "en-gb",
			tts:      Some("bouquet"),
			keywords: &["", "anniversary", "birthday", "date", "flower", "love", "plant", "romance"],
		},
		#[cfg(feature = "es")]
		crate::Annotation {
			lang:     "es",
			tts:      Some("ramo de flores"),
			keywords: &["", "bouquet", "flores", "ramo"],
		},
		#[cfg(feature = "es-mx")]
		crate::Annotation {
			lang:     "es-mx",
			tts:      Some("ramo de flores"),
			keywords: &["", "bouquet", "flores", "ramo"],
		},
		#[cfg(feature = "et")]
		crate::Annotation {
			lang:     "et",
			tts:      Some("kimp"),
			keywords: &["", "lill", "romantika", "taim"],
		},
		#[cfg(feature = "fi")]
		crate::Annotation {
			lang:     "fi",
			tts:      Some("kukkakimppu"),
			keywords: &["", "kasvi", "kimppu", "kukka", "romantiikka", "treffit"],
		},
		#[cfg(feature = "fr")]
		crate::Annotation {
			lang:     "fr",
			tts:      Some("bouquet"),
			keywords: &[
				"",
				"amoureux",
				"anniversaire",
				"fleur",
				"fleurs",
				"plante",
				"rendez-vous",
				"roses",
			],
		},
		#[cfg(feature = "hi")]
		crate::Annotation {
			lang:     "hi",
			tts:      Some("फ\u{942}ल, फ\u{942}लग\u{941}च\u{94d}छ"),
			keywords: &["", "प\u{94d}यार", "फ\u{942}ल", "फ\u{942}लग\u{941}च\u{94d}छ", "ब\u{941}क\u{947}"],
		},
		#[cfg(feature = "hu")]
		crate::Annotation {
			lang:     "hu",
			tts:      Some("csokor"),
			keywords: &[
				"",
				"növény",
				"randevú",
				"randi",
				"romantika",
				"születésnap",
				"virág",
				"virágcsokor",
				"évforduló",
			],
		},
		#[cfg(feature = "it")]
		crate::Annotation {
			lang:     "it",
			tts:      Some("mazzo di fiori"),
			keywords: &[
				"",
				"anniversario",
				"appuntamento",
				"bouquet",
				"compleanno",
				"fioraio",
				"fiore",
				"fiori",
				"natura",
				"regalo",
				"romanticismo",
				"romantico",
			],
		},
		#[cfg(feature = "ja")]
		crate::Annotation {
			lang:     "ja",
			tts:      Some("花束"),
			keywords: &["", "ブーケ", "植物", "花"],
		},
		#[cfg(feature = "ko")]
		crate::Annotation {
			lang:     "ko",
			tts:      Some("꽃다발"),
			keywords: &["", "기념일", "꽃", "데이트", "로맨스", "부케", "사랑", "생일", "식물", "연애"],
		},
		#[cfg(feature = "lt")]
		crate::Annotation {
			lang:     "lt",
			tts:      Some("puokštė"),
			keywords: &["", "gimtadienis", "gėlė", "meilė", "metinės"],
		},
		#[cfg(feature = "ms")]
		crate::Annotation {
			lang:     "ms",
			tts:      Some("sejambak bunga"),
			keywords: &[
				"",
				"bunga",
				"cinta",
				"hari jadi",
				"kasih",
				"percintaan",
				"tumbuhan",
				"ulang tahun",
			],
		},
		#[cfg(feature = "nb")]
		crate::Annotation {
			lang:     "nb",
			tts:      Some("bukett"),
			keywords: &["", "blomst", "blomster", "romantikk"],
		},
		#[cfg(feature = "nl")]
		crate::Annotation {
			lang:     "nl",
			tts:      Some("boeket"),
			keywords: &[
				"",
				"afspraakje",
				"bedankt",
				"beterschap",
				"bloem",
				"date",
				"jubileum",
				"romantiek",
				"verjaardag",
			],
		},
		#[cfg(feature = "pl")]
		crate::Annotation {
			lang:     "pl",
			tts:      Some("bukiet kwiatów"),
			keywords: &["", "bukiet", "kwiaty", "miłość", "roślina"],
		},
		#[cfg(feature = "pt")]
		crate::Annotation {
			lang:     "pt",
			tts:      Some("buquê"),
			keywords: &["", "aniversário", "flor", "planta", "romance"],
		},
		#[cfg(feature = "ru")]
		crate::Annotation {
			lang:     "ru",
			tts:      Some("букет"),
			keywords: &[
				"",
				"любовь",
				"подарок",
				"поздравление",
				"праздник",
				"розы",
				"романтика",
				"свидание",
				"тюльпаны",
				"цветы",
			],
		},
		#[cfg(feature = "sv")]
		crate::Annotation {
			lang:     "sv",
			tts:      Some("blombukett"),
			keywords: &["", "blommor", "kärlek"],
		},
		#[cfg(feature = "th")]
		crate::Annotation {
			lang:     "th",
			tts:      Some("ช\u{e48}อดอกไม\u{e49}"),
			keywords: &["", "ดอกไม\u{e49}"],
		},
		#[cfg(feature = "uk")]
		crate::Annotation {
			lang:     "uk",
			tts:      Some("букет"),
			keywords: &["", "квіти", "квітка", "кохання", "рози", "рослина"],
		},
		#[cfg(feature = "vi")]
		crate::Annotation {
			lang:     "vi",
			tts:      Some("bó hoa"),
			keywords: &["", "cây", "hoa", "hẹn hò", "kỷ niệm", "lãng mạn", "sinh nhật", "tình yêu"],
		},
		#[cfg(feature = "zh")]
		crate::Annotation {
			lang:     "zh",
			tts:      Some("花束"),
			keywords: &["", "周年纪念", "生日", "罗曼史", "鲜花"],
		},
		#[cfg(feature = "zh-hant")]
		crate::Annotation {
			lang:     "zh-hant",
			tts:      Some("花束"),
			keywords: &["", "浪漫", "週年", "鮮花"],
		},
	],
};
#[doc = "💮"]
pub const WHITE_FLOWER: crate::Emoji = crate::Emoji {
	glyph:                "💮",
	codepoint:            &[128174u32],
	status:               crate::Status::FullyQualified,
	introduction_version: crate::Version { major: 0u8, minor: 6u8, patch: 0u8 },
	name:                 "white flower",
	group:                crate::Group::AnimalsNature,
	subgroup:             crate::Subgroup::PlantFlower,
	is_variant:           false,
	variants:             &[],
	skin_tones:           None,
	gender_variants:      None,
	annotations:          &[
		#[cfg(feature = "bn")]
		crate::Annotation {
			lang:     "bn",
			tts:      Some("স\u{9be}দ\u{9be} ফ\u{9c1}ল"),
			keywords: &["", "ফ\u{9c1}ল"],
		},
		#[cfg(feature = "da")]
		crate::Annotation { lang: "da", tts: Some("hvid blomst"), keywords: &["", "blomst"] },
		#[cfg(feature = "de")]
		crate::Annotation {
			lang:     "de",
			tts:      Some("Blumenstempel"),
			keywords: &["", "blume", "blumenstempel"],
		},
		#[cfg(feature = "en")]
		crate::Annotation {
			lang:     "en",
			tts:      Some("white flower"),
			keywords: &["", "flower", "white"],
		},
		#[cfg(feature = "en-gb")]
		crate::Annotation {
			lang:     "en-gb",
			tts:      Some("white flower"),
			keywords: &["", "flower", "white"],
		},
		#[cfg(feature = "es")]
		crate::Annotation {
			lang:     "es",
			tts:      Some("flor blanca"),
			keywords: &["", "blanca", "flor"],
		},
		#[cfg(feature = "es-mx")]
		crate::Annotation {
			lang:     "es-mx",
			tts:      Some("flor blanca"),
			keywords: &["", "blanca", "flor"],
		},
		#[cfg(feature = "et")]
		crate::Annotation {
			lang:     "et",
			tts:      Some("valge lill"),
			keywords: &["", "lill", "valge", "õis"],
		},
		#[cfg(feature = "fi")]
		crate::Annotation {
			lang:     "fi",
			tts:      Some("valkoinen kukka"),
			keywords: &["", "kukka"],
		},
		#[cfg(feature = "fr")]
		crate::Annotation { lang: "fr", tts: Some("fleur blanche"), keywords: &["", "fleur"] },
		#[cfg(feature = "hi")]
		crate::Annotation {
			lang:     "hi",
			tts:      Some("फ\u{942}ल की छाप"),
			keywords: &[
				"",
				"छाप",
				"फ\u{942}ल",
				"बह\u{941}त अच\u{94d}छ\u{947} काम की छाप",
				"सफ\u{93c}\u{947}द फ\u{942}ल",
			],
		},
		#[cfg(feature = "hu")]
		crate::Annotation {
			lang:     "hu",
			tts:      Some("fehér virág"),
			keywords: &["", "növény", "virág"],
		},
		#[cfg(feature = "it")]
		crate::Annotation {
			lang:     "it",
			tts:      Some("fiore bianco"),
			keywords: &["", "fiore", "fiorellino", "simbolo ben fatto"],
		},
		#[cfg(feature = "ja")]
		crate::Annotation {
			lang:     "ja",
			tts:      Some("大変よくできました"),
			keywords: &["", "はなまる", "よくできました", "花"],
		},
		#[cfg(feature = "ko")]
		crate::Annotation {
			lang:     "ko",
			tts:      Some("흰 꽃"),
			keywords: &["", "꽃", "꽃 도장", "꽃 표시", "화이트 플라워"],
		},
		#[cfg(feature = "lt")]
		crate::Annotation { lang: "lt", tts: Some("balta gėlė"), keywords: &["", "gėlė"] },
		#[cfg(feature = "ms")]
		crate::Annotation { lang: "ms", tts: Some("bunga putih"), keywords: &["", "bunga"] },
		#[cfg(feature = "nb")]
		crate::Annotation {
			lang:     "nb",
			tts:      Some("hvit blomst"),
			keywords: &["", "blomst", "blomsterstempel", "stempel"],
		},
		#[cfg(feature = "nl")]
		crate::Annotation {
			lang:     "nl",
			tts:      Some("witte bloem"),
			keywords: &["", "bloem", "wit"],
		},
		#[cfg(feature = "pl")]
		crate::Annotation {
			lang:     "pl",
			tts:      Some("biały kwiat"),
			keywords: &["", "kontury", "kwiatek", "pieczątka"],
		},
		#[cfg(feature = "pt")]
		crate::Annotation {
			lang:     "pt",
			tts:      Some("flor branca"),
			keywords: &["", "carimbo", "carimbo de flor", "carimbo de parabéns", "flor"],
		},
		#[cfg(feature = "ru")]
		crate::Annotation {
			lang:     "ru",
			tts:      Some("белый цветок"),
			keywords: &["", "белый", "цветок"],
		},
		#[cfg(feature = "sv")]
		crate::Annotation { lang: "sv", tts: Some("vit blomma"), keywords: &["", "blomma"] },
		#[cfg(feature = "th")]
		crate::Annotation {
			lang:     "th",
			tts:      Some("ตราดอกไม\u{e49}"),
			keywords: &["", "ดอกไม\u{e49}"],
		},
		#[cfg(feature = "uk")]
		crate::Annotation {
			lang: "uk", tts: Some("біла квітка"), keywords: &["", "квітка"]
		},
		#[cfg(feature = "vi")]
		crate::Annotation { lang: "vi", tts: Some("hoa trắng"), keywords: &["", "hoa"] },
		#[cfg(feature = "zh")]
		crate::Annotation { lang: "zh", tts: Some("白花"), keywords: &["", "花"] },
		#[cfg(feature = "zh-hant")]
		crate::Annotation { lang: "zh-hant", tts: Some("白花"), keywords: &["", "花"] },
	],
};
#[doc = "🪷"]
pub const LOTUS: crate::Emoji = crate::Emoji {
	glyph:                "🪷",
	codepoint:            &[129719u32],
	status:               crate::Status::FullyQualified,
	introduction_version: crate::Version { major: 14u8, minor: 0u8, patch: 0u8 },
	name:                 "lotus",
	group:                crate::Group::AnimalsNature,
	subgroup:             crate::Subgroup::PlantFlower,
	is_variant:           false,
	variants:             &[],
	skin_tones:           None,
	gender_variants:      None,
	annotations:          &[
		#[cfg(feature = "bn")]
		crate::Annotation {
			lang:     "bn",
			tts:      Some("পদ\u{9cd}ম"),
			keywords: &[
				"",
				"পবিত\u{9cd}রত\u{9be}",
				"ফ\u{9c1}ল",
				"বৌদ\u{9cd}ধধর\u{9cd}ম",
				"সৌন\u{9cd}দর\u{9cd}য",
				"হিন\u{9cd}দ\u{9c1}ধর\u{9cd}ম",
			],
		},
		#[cfg(feature = "da")]
		crate::Annotation {
			lang:     "da",
			tts:      Some("lotus"),
			keywords: &[
				"",
				"blomst",
				"buddhisme",
				"fred",
				"hinduisme",
				"indien",
				"renhed",
				"ro",
				"sindsro",
				"skønhed",
				"vietnam",
			],
		},
		#[cfg(feature = "de")]
		crate::Annotation {
			lang:     "de",
			tts:      Some("Lotusblüte"),
			keywords: &[
				"",
				"blume",
				"buddhismus",
				"erleuchtung",
				"gleichmut",
				"hinduismus",
				"liebe",
				"lotosblume",
				"lotusblüte",
				"pflanze",
				"reinheit",
				"ruhe",
				"schönheit",
			],
		},
		#[cfg(feature = "en")]
		crate::Annotation {
			lang:     "en",
			tts:      Some("lotus"),
			keywords: &[
				"", "beauty", "buddhism", "calm", "flower", "hinduism", "peace", "purity", "serenity",
			],
		},
		#[cfg(feature = "en-gb")]
		crate::Annotation {
			lang:     "en-gb",
			tts:      Some("lotus"),
			keywords: &[
				"", "beauty", "buddhism", "calm", "flower", "hinduism", "peace", "purity", "serenity",
			],
		},
		#[cfg(feature = "es")]
		crate::Annotation {
			lang:     "es",
			tts:      Some("loto"),
			keywords: &["", "budismo", "flor", "hinduismo", "pureza"],
		},
		#[cfg(feature = "es-mx")]
		crate::Annotation {
			lang:     "es-mx",
			tts:      Some("loto"),
			keywords: &["", "budismo", "flor", "hinduismo", "lotus", "pureza"],
		},
		#[cfg(feature = "et")]
		crate::Annotation {
			lang:     "et",
			tts:      Some("lootos"),
			keywords: &["", "budism", "hinduism", "ilu", "lill", "puhtus", "rahu"],
		},
		#[cfg(feature = "fi")]
		crate::Annotation {
			lang:     "fi",
			tts:      Some("lootus"),
			keywords: &[
				"",
				"buddhalaisuus",
				"hindulaisuus",
				"intia",
				"kauneus",
				"kukka",
				"puhtaus",
				"rauhallinen",
				"viattomuus",
			],
		},
		#[cfg(feature = "fr")]
		crate::Annotation {
			lang:     "fr",
			tts:      Some("lotus"),
			keywords: &["", "beauté", "bouddhisme", "calme", "fleur", "hindouisme", "pureté", "sérénité"],
		},
		#[cfg(feature = "hi")]
		crate::Annotation {
			lang:     "hi",
			tts:      Some("कमल"),
			keywords: &[
				"",
				"कमल का फ\u{942}ल",
				"पवित\u{94d}रता",
				"फ\u{942}ल",
				"बौद\u{94d}ध धर\u{94d}म",
				"ब\u{94d}य\u{942}टी",
				"मन की शा\u{902}ति",
				"शा\u{902}ति",
				"स\u{941}\u{902}दरता",
				"हि\u{902}द\u{942} धर\u{94d}म",
			],
		},
		#[cfg(feature = "hu")]
		crate::Annotation {
			lang:     "hu",
			tts:      Some("lótusz"),
			keywords: &[
				"",
				"buddhizmus",
				"hinduizmus",
				"india",
				"lótuszvirág",
				"tisztaság",
				"vietnam",
				"virág",
			],
		},
		#[cfg(feature = "it")]
		crate::Annotation {
			lang:     "it",
			tts:      Some("loto"),
			keywords: &[
				"",
				"bellezza",
				"buddhismo",
				"fior di loto",
				"fiore",
				"india",
				"induismo",
				"purezza",
				"serenità",
				"vietnam",
				"yoga",
			],
		},
		#[cfg(feature = "ja")]
		crate::Annotation {
			lang:     "ja",
			tts:      Some("ハスの花"),
			keywords: &["", "ヒンドゥー教", "仏教", "極楽", "清浄", "花"],
		},
		#[cfg(feature = "ko")]
		crate::Annotation {
			lang:     "ko",
			tts:      Some("연꽃"),
			keywords: &[
				"",
				"꽃",
				"베트남",
				"불교",
				"뷰티",
				"순수",
				"아름다움",
				"인도",
				"평온",
				"평화",
				"플라워",
				"힌두교",
			],
		},
		#[cfg(feature = "lt")]
		crate::Annotation {
			lang:     "lt",
			tts:      Some("lotusas"),
			keywords: &[
				"",
				"budizmas",
				"grožis",
				"gėlė",
				"indija",
				"induizmas",
				"lotosas",
				"ramybė",
				"taika",
				"tyrumas",
				"vietnamas",
			],
		},
		#[cfg(feature = "ms")]
		crate::Annotation {
			lang:     "ms",
			tts:      Some("teratai"),
			keywords: &["", "budisme", "bunga", "hinduisme", "india", "kesucian", "vietnam"],
		},
		#[cfg(feature = "nb")]
		crate::Annotation {
			lang:     "nb",
			tts:      Some("lotusblomst"),
			keywords: &[
				"",
				"blomst",
				"buddhisme",
				"fred",
				"hinduisme",
				"lotus",
				"renhet",
				"ro",
				"sinnsro",
				"skjønnhet",
			],
		},
		#[cfg(feature = "nl")]
		crate::Annotation {
			lang:     "nl",
			tts:      Some("lotus"),
			keywords: &[
				"",
				"bloem",
				"boeddhisme",
				"hindoeïsme",
				"puurheid",
				"rust",
				"schoonheid",
				"sereen",
				"vrede",
			],
		},
		#[cfg(feature = "pl")]
		crate::Annotation {
			lang:     "pl",
			tts:      Some("lotos"),
			keywords: &["", "buddyzm", "czystość", "hinduizm", "kwiat", "lotosu", "piękno", "spokój"],
		},
		#[cfg(feature = "pt")]
		crate::Annotation {
			lang:     "pt",
			tts:      Some("lótus"),
			keywords: &[
				"",
				"beleza",
				"budismo",
				"calma",
				"flor",
				"flor de lótus",
				"hinduísmo",
				"paz",
				"pureza",
				"serenidade",
				"vietnã",
				"índia",
			],
		},
		#[cfg(feature = "ru")]
		crate::Annotation {
			lang:     "ru",
			tts:      Some("лотос"),
			keywords: &[
				"",
				"буддизм",
				"индуизм",
				"красота",
				"медитация",
				"спокойствие",
				"умиротворение",
				"цветок",
				"чистота",
			],
		},
		#[cfg(feature = "sv")]
		crate::Annotation {
			lang:     "sv",
			tts:      Some("lotus"),
			keywords: &["", "blomma", "buddhism", "frid", "hinduism", "renhet", "stillhet"],
		},
		#[cfg(feature = "th")]
		crate::Annotation {
			lang:     "th",
			tts:      Some("ดอกบ\u{e31}ว"),
			keywords: &[
				"",
				"ความงาม",
				"ความบร\u{e34}ส\u{e38}ทธ\u{e34}\u{e4c}",
				"ความสงบ",
				"ดอกไม\u{e49}",
				"ว\u{e31}นพระ",
				"ศาสนาพ\u{e38}ทธ",
				"ศาสนาฮ\u{e34}นด\u{e39}",
				"อ\u{e34}นเด\u{e35}ย",
				"ไหว\u{e49}พระ",
			],
		},
		#[cfg(feature = "uk")]
		crate::Annotation {
			lang:     "uk",
			tts:      Some("лотос"),
			keywords: &[
				"",
				"безтурботність",
				"буддизм",
				"врівноваженість",
				"гармонія",
				"квітка",
				"краса",
				"лад",
				"мир",
				"символ чистоти",
				"спокій",
				"індуїзм",
			],
		},
		#[cfg(feature = "vi")]
		crate::Annotation {
			lang:     "vi",
			tts:      Some("hoa sen"),
			keywords: &["", "hoa", "phật giáo", "sự tinh khiết", "việt nam", "ấn độ", "ấn độ giáo"],
		},
		#[cfg(feature = "zh")]
		crate::Annotation {
			lang:     "zh",
			tts:      Some("莲花"),
			keywords: &["", "佛教", "印度教", "幽静", "恬静", "纯洁", "花", "花朵"],
		},
		#[cfg(feature = "zh-hant")]
		crate::Annotation {
			lang:     "zh-hant",
			tts:      Some("蓮花"),
			keywords: &["", "佛教", "印度", "印度教", "寧靜", "平和", "平靜", "純潔", "花", "越南"],
		},
	],
};
#[doc = "🌸"]
pub const CHERRY_BLOSSOM: crate::Emoji = crate::Emoji {
	glyph:                "🌸",
	codepoint:            &[127800u32],
	status:               crate::Status::FullyQualified,
	introduction_version: crate::Version { major: 0u8, minor: 6u8, patch: 0u8 },
	name:                 "cherry blossom",
	group:                crate::Group::AnimalsNature,
	subgroup:             crate::Subgroup::PlantFlower,
	is_variant:           false,
	variants:             &[],
	skin_tones:           None,
	gender_variants:      None,
	annotations:          &[
		#[cfg(feature = "bn")]
		crate::Annotation {
			lang:     "bn",
			tts:      Some("চেরি ব\u{9cd}লজম"),
			keywords: &[
				"",
				"গ\u{9be}ছ",
				"চেরি",
				"চেরি ব\u{9cd}লসম",
				"ফ\u{9c1}ল",
				"বিকশিত হওয\u{9bc}\u{9be}",
			],
		},
		#[cfg(feature = "da")]
		crate::Annotation {
			lang:     "da",
			tts:      Some("kirsebærblomst"),
			keywords: &["", "blomst", "blomstrende kirsebær", "kirsebær"],
		},
		#[cfg(feature = "de")]
		crate::Annotation {
			lang:     "de",
			tts:      Some("Kirschblüte"),
			keywords: &["", "blume", "blüte", "kirschblüte", "kirsche", "pflanze"],
		},
		#[cfg(feature = "en")]
		crate::Annotation {
			lang:     "en",
			tts:      Some("cherry blossom"),
			keywords: &["", "blossom", "cherry", "flower", "plant", "spring", "springtime"],
		},
		#[cfg(feature = "en-gb")]
		crate::Annotation {
			lang:     "en-gb",
			tts:      Some("cherry blossom"),
			keywords: &["", "blossom", "cherry", "flower", "plant", "spring", "springtime"],
		},
		#[cfg(feature = "es")]
		crate::Annotation {
			lang:     "es",
			tts:      Some("flor de cerezo"),
			keywords: &["", "cerezo", "flor"],
		},
		#[cfg(feature = "es-mx")]
		crate::Annotation {
			lang:     "es-mx",
			tts:      Some("flor de cerezo"),
			keywords: &["", "cerezo", "flor"],
		},
		#[cfg(feature = "et")]
		crate::Annotation {
			lang:     "et",
			tts:      Some("kirsiõis"),
			keywords: &["", "kirss", "lill", "taim", "õis"],
		},
		#[cfg(feature = "fi")]
		crate::Annotation {
			lang:     "fi",
			tts:      Some("kirsikankukka"),
			keywords: &["", "kasvi", "kirsikka", "kukka"],
		},
		#[cfg(feature = "fr")]
		crate::Annotation {
			lang:     "fr",
			tts:      Some("fleur de cerisier"),
			keywords: &["", "bourgeon", "fleur", "plante", "printemps"],
		},
		#[cfg(feature = "hi")]
		crate::Annotation {
			lang:     "hi",
			tts:      Some("च\u{948}री ब\u{94d}लॉसम"),
			keywords: &["", "च\u{948}री", "फ\u{942}ल", "फ\u{942}ल द\u{947}खना", "वस\u{902}त"],
		},
		#[cfg(feature = "hu")]
		crate::Annotation {
			lang:     "hu",
			tts:      Some("cseresznyevirág"),
			keywords: &["", "cseresznye", "növény", "virág"],
		},
		#[cfg(feature = "it")]
		crate::Annotation {
			lang:     "it",
			tts:      Some("fiore di ciliegio"),
			keywords: &[
				"",
				"ciliegio",
				"fiore",
				"fiorito",
				"natura",
				"pianta",
				"primavera",
				"primula",
				"rosa",
			],
		},
		#[cfg(feature = "ja")]
		crate::Annotation {
			lang:     "ja",
			tts:      Some("桜"),
			keywords: &["", "サクラ", "植物", "花", "開花"],
		},
		#[cfg(feature = "ko")]
		crate::Annotation {
			lang:     "ko",
			tts:      Some("벚꽃"),
			keywords: &["", "꽃", "꽃송이", "꽃이 피는", "벚꽃놀이", "봄", "식물"],
		},
		#[cfg(feature = "lt")]
		crate::Annotation {
			lang:     "lt",
			tts:      Some("vyšnios žiedas"),
			keywords: &["", "augalas", "gėlė", "vyšnia", "žydėti"],
		},
		#[cfg(feature = "ms")]
		crate::Annotation {
			lang:     "ms",
			tts:      Some("bunga sakura"),
			keywords: &["", "bunga", "mekar", "sakura", "tumbuhan"],
		},
		#[cfg(feature = "nb")]
		crate::Annotation {
			lang:     "nb",
			tts:      Some("kirsebærblomst"),
			keywords: &["", "blomst", "kirsebær"],
		},
		#[cfg(feature = "nl")]
		crate::Annotation {
			lang:     "nl",
			tts:      Some("kersenbloesem"),
			keywords: &["", "bloem", "bloesem", "kers", "lente", "plant", "voorjaar"],
		},
		#[cfg(feature = "pl")]
		crate::Annotation {
			lang:     "pl",
			tts:      Some("kwiat wiśni"),
			keywords: &["", "kwiat", "kwiatek", "kwitnąca wiśnia", "sakura", "wiśnia"],
		},
		#[cfg(feature = "pt")]
		crate::Annotation {
			lang:     "pt",
			tts:      Some("flor de cerejeira"),
			keywords: &["", "cereja", "cerejeira", "flor", "planta", "primavera"],
		},
		#[cfg(feature = "ru")]
		crate::Annotation {
			lang:     "ru",
			tts:      Some("цветок вишни"),
			keywords: &["", "мальва", "сакура", "цвести", "цветок"],
		},
		#[cfg(feature = "sv")]
		crate::Annotation {
			lang:     "sv",
			tts:      Some("körsbärsblomma"),
			keywords: &["", "blomma", "körsbär"],
		},
		#[cfg(feature = "th")]
		crate::Annotation {
			lang:     "th",
			tts:      Some("ดอกซาก\u{e38}ระ"),
			keywords: &["", "ดอกไม\u{e49}"],
		},
		#[cfg(feature = "uk")]
		crate::Annotation {
			lang:     "uk",
			tts:      Some("вишневий цвіт"),
			keywords: &["", "вишня", "квітка", "рослина", "цвіт", "цвіт вишні"],
		},
		#[cfg(feature = "vi")]
		crate::Annotation {
			lang:     "vi",
			tts:      Some("hoa anh đào"),
			keywords: &["", "hoa", "thực vật"],
		},
		#[cfg(feature = "zh")]
		crate::Annotation { lang: "zh", tts: Some("樱花"), keywords: &["", "花"] },
		#[cfg(feature = "zh-hant")]
		crate::Annotation { lang: "zh-hant", tts: Some("櫻花"), keywords: &["", "花"] },
	],
};
#[doc = "🪻"]
pub const HYACINTH: crate::Emoji = crate::Emoji {
	glyph:                "🪻",
	codepoint:            &[129723u32],
	status:               crate::Status::FullyQualified,
	introduction_version: crate::Version { major: 15u8, minor: 0u8, patch: 0u8 },
	name:                 "hyacinth",
	group:                crate::Group::AnimalsNature,
	subgroup:             crate::Subgroup::PlantFlower,
	is_variant:           false,
	variants:             &[],
	skin_tones:           None,
	gender_variants:      None,
	annotations:          &[
		#[cfg(feature = "bn")]
		crate::Annotation {
			lang:     "bn",
			tts:      Some("হ\u{9be}ইসিন\u{9cd}থ"),
			keywords: &[
				"",
				"কচ\u{9c1}রিপ\u{9be}ন\u{9be}র ফ\u{9c1}ল",
				"ফ\u{9c1}ল",
				"বসন\u{9cd}ত",
				"বেগ\u{9c1}নি",
				"ব\u{9cd}ল\u{9c1}বোনেট",
				"ল\u{9c1}পিন",
				"ল\u{9cd}য\u{9be}ভেন\u{9cd}ড\u{9be}র",
				"স\u{9cd}ন\u{9cd}য\u{9be}পড\u{9cd}র\u{9be}গন",
			],
		},
		#[cfg(feature = "da")]
		crate::Annotation {
			lang:     "da",
			tts:      Some("hyacint"),
			keywords: &[
				"",
				"blomst",
				"blomstre",
				"blå lupin",
				"blålilla",
				"forår",
				"lavendel",
				"lilla",
				"lupin",
				"løvemund",
				"plante",
			],
		},
		#[cfg(feature = "de")]
		crate::Annotation {
			lang:     "de",
			tts:      Some("Hyazinthe"),
			keywords: &[
				"",
				"blaue wiesenlupine",
				"blume",
				"blühen",
				"blüte",
				"frühling",
				"hyazinthe",
				"lavendel",
				"lila",
				"lupine",
				"löwenmaul",
				"pflanze",
				"violett",
			],
		},
		#[cfg(feature = "en")]
		crate::Annotation {
			lang:     "en",
			tts:      Some("hyacinth"),
			keywords: &[
				"",
				"bloom",
				"bluebonnet",
				"flower",
				"indigo",
				"lavender",
				"lilac",
				"lupine",
				"plant",
				"purple",
				"shrub",
				"snapdragon",
				"spring",
				"violet",
			],
		},
		#[cfg(feature = "en-gb")]
		crate::Annotation {
			lang:     "en-gb",
			tts:      Some("hyacinth"),
			keywords: &[
				"",
				"bloom",
				"bluebonnet",
				"flower",
				"indigo",
				"lavender",
				"lilac",
				"lupine",
				"plant",
				"purple",
				"shrub",
				"snapdragon",
				"spring",
				"violet",
			],
		},
		#[cfg(feature = "es")]
		crate::Annotation {
			lang:     "es",
			tts:      Some("campanilla"),
			keywords: &["", "boca de dragón", "flor", "jacinto", "lavanda", "lila", "lupino"],
		},
		#[cfg(feature = "es-mx")]
		crate::Annotation {
			lang:     "es-mx",
			tts:      Some("campanilla"),
			keywords: &["", "boca de dragón", "flor", "jacinto", "lavanda", "lila", "lupino"],
		},
		#[cfg(feature = "et")]
		crate::Annotation {
			lang:     "et",
			tts:      Some("hüatsint"),
			keywords: &["", "indigo", "lavendel", "lill", "lilla", "lupiin", "lõvilõug", "taim"],
		},
		#[cfg(feature = "fi")]
		crate::Annotation {
			lang:     "fi",
			tts:      Some("hyasintti"),
			keywords: &[
				"",
				"kasvi",
				"kukka",
				"laventeli",
				"leijonankita",
				"luonto",
				"lupiini",
				"sinilupiini",
				"syreeni",
				"violetti",
			],
		},
		#[cfg(feature = "fr")]
		crate::Annotation {
			lang:     "fr",
			tts:      Some("jacinthe"),
			keywords: &[
				"",
				"fleur",
				"fleurir",
				"indigo",
				"lavande",
				"lilas",
				"lupin",
				"mauve",
				"muflier",
				"plante",
				"printemps",
				"violet",
			],
		},
		#[cfg(feature = "hi")]
		crate::Annotation {
			lang:     "hi",
			tts:      Some("ह\u{94d}यची\u{902}थ"),
			keywords: &[
				"",
				"इ\u{902}डिगो",
				"झाड\u{93c}ी",
				"पौधा",
				"फ\u{942}ल",
				"ब\u{948}\u{902}गनी",
				"ब\u{94d}ल\u{942}बोन\u{947}ट",
				"ल\u{948}व\u{947}\u{902}डर",
				"ल\u{94d}य\u{942}पाइन",
				"स\u{94d}न\u{948}पड\u{94d}र\u{948}गन",
			],
		},
		#[cfg(feature = "hu")]
		crate::Annotation {
			lang:     "hu",
			tts:      Some("jácint"),
			keywords: &["", "búzavirág", "csillagfürt", "levendula", "tátika", "virág"],
		},
		#[cfg(feature = "it")]
		crate::Annotation {
			lang:     "it",
			tts:      Some("giacinto"),
			keywords: &["", "bocca di leone", "fiore", "lavanda", "lupino", "primavera"],
		},
		#[cfg(feature = "ja")]
		crate::Annotation {
			lang:     "ja",
			tts:      Some("ヒヤシンス"),
			keywords: &[
				"",
				"スミレ色",
				"ブルーボネット",
				"ライラック",
				"ラベンダー",
				"植物",
				"紫色",
				"花",
				"青紫",
			],
		},
		#[cfg(feature = "ko")]
		crate::Annotation {
			lang:     "ko",
			tts:      Some("히아신스"),
			keywords: &[
				"",
				"꽃",
				"라벤더",
				"라일락",
				"루핀",
				"보라색",
				"봄",
				"블루 보닛",
				"스냅드래곤",
				"식물",
				"인디고",
				"제비꽃",
			],
		},
		#[cfg(feature = "lt")]
		crate::Annotation {
			lang:     "lt",
			tts:      Some("hiacintas"),
			keywords: &[
				"",
				"augalas",
				"gėlė",
				"indigo",
				"levandos",
				"lubinai",
				"pavasaris",
				"violetinis",
				"žeidas",
				"želdinys",
				"žydėjimas",
			],
		},
		#[cfg(feature = "ms")]
		crate::Annotation {
			lang:     "ms",
			tts:      Some("keladi bunting"),
			keywords: &[
				"",
				"bluebonnet",
				"bunga",
				"bunga keladi bunting",
				"lavender",
				"lembayung",
				"lupine",
				"snapdragon",
			],
		},
		#[cfg(feature = "nb")]
		crate::Annotation {
			lang:     "nb",
			tts:      Some("hyasint"),
			keywords: &[
				"",
				"blomst",
				"klokkeblåstjerne",
				"lavendel",
				"lilla",
				"lupin",
				"plante",
				"svibel",
				"vår",
			],
		},
		#[cfg(feature = "nl")]
		crate::Annotation {
			lang:     "nl",
			tts:      Some("hyacint"),
			keywords: &["", "bloem", "lavendel", "leeuwenbek", "lupine", "vlinderbloemige"],
		},
		#[cfg(feature = "pl")]
		crate::Annotation {
			lang:     "pl",
			tts:      Some("hiacynt"),
			keywords: &[
				"",
				"bławatek",
				"chaber",
				"fioletowy",
				"indygo",
				"krzew",
				"kwiat",
				"kwitnienie",
				"lawenda",
				"lilak",
				"lwia paszcza",
				"purpurowy",
				"wiosna",
				"łubin",
			],
		},
		#[cfg(feature = "pt")]
		crate::Annotation {
			lang:     "pt",
			tts:      Some("jacinto"),
			keywords: &[
				"",
				"arbusto",
				"bluebonnet",
				"flor",
				"flor-crânio-do-dragão",
				"lavanda",
				"lilás",
				"lupinus",
				"planta",
				"primavera",
				"roxo",
				"violeta",
			],
		},
		#[cfg(feature = "ru")]
		crate::Annotation {
			lang:     "ru",
			tts:      Some("гиацинт"),
			keywords: &["", "лаванда", "львиный зев", "люпин", "цветок"],
		},
		#[cfg(feature = "sv")]
		crate::Annotation {
			lang:     "sv",
			tts:      Some("hyacint"),
			keywords: &[
				"",
				"blomma",
				"bluebonnet",
				"lavendel",
				"lila",
				"luktärter",
				"lupin",
				"purpur",
				"violblå",
				"violett",
			],
		},
		#[cfg(feature = "th")]
		crate::Annotation {
			lang:     "th",
			tts:      Some("ไฮยาซ\u{e34}นธ\u{e4c}"),
			keywords: &[
				"",
				"ดอกไม\u{e49}",
				"ดอกไม\u{e49}บาน",
				"บล\u{e39}บอนเนต",
				"ลาเวนเดอร\u{e4c}",
				"ล\u{e34}\u{e49}นม\u{e31}งกร",
				"ล\u{e39}พ\u{e34}น",
				"ส\u{e35}ม\u{e48}วง",
			],
		},
		#[cfg(feature = "uk")]
		crate::Annotation {
			lang:     "uk",
			tts:      Some("гіацинт"),
			keywords: &["", "весна", "квітка", "рослина", "фіолетовий"],
		},
		#[cfg(feature = "vi")]
		crate::Annotation {
			lang:     "vi",
			tts:      Some("lục bình"),
			keywords: &[
				"",
				"bluebonnet",
				"bụi hoa",
				"cây",
				"hoa",
				"hoa dạ lan hương",
				"hoa len mũ xanh",
				"hoa mõm sói",
				"hoa oải hương",
				"hoa tiên ông",
				"hoa tử đinh hương",
				"màu chàm",
				"mùa xuân",
				"oải hương",
				"tím",
				"đậu lupin",
			],
		},
		#[cfg(feature = "zh")]
		crate::Annotation {
			lang:     "zh",
			tts:      Some("风信子"),
			keywords: &[
				"",
				"春天",
				"植物",
				"灌木",
				"矢车菊",
				"紫丁香",
				"紫罗兰",
				"紫色",
				"羽扇豆",
				"花",
				"花朵",
				"蓝帽花",
				"薰衣草",
				"金鱼草",
				"靛蓝",
			],
		},
		#[cfg(feature = "zh-hant")]
		crate::Annotation {
			lang:     "zh-hant",
			tts:      Some("風信子"),
			keywords: &[
				"",
				"矢車菊",
				"羽扇豆",
				"花",
				"薰衣草",
				"金魚藻",
				"風信子，紫色，花苞，春天，紫羅蘭，靛藍，紫丁香，薰衣草，植物，花，羽扇豆",
				"魯冰花",
			],
		},
	],
};
#[doc = "🌼"]
pub const BLOSSOM: crate::Emoji = crate::Emoji {
	glyph:                "🌼",
	codepoint:            &[127804u32],
	status:               crate::Status::FullyQualified,
	introduction_version: crate::Version { major: 0u8, minor: 6u8, patch: 0u8 },
	name:                 "blossom",
	group:                crate::Group::AnimalsNature,
	subgroup:             crate::Subgroup::PlantFlower,
	is_variant:           false,
	variants:             &[],
	skin_tones:           None,
	gender_variants:      None,
	annotations:          &[
		#[cfg(feature = "bn")]
		crate::Annotation { lang: "bn", tts: Some("ফ\u{9c1}ল"), keywords: &["", "গ\u{9be}ছ"] },
		#[cfg(feature = "da")]
		crate::Annotation { lang: "da", tts: Some("blomst"), keywords: &["", "gul blomst"] },
		#[cfg(feature = "de")]
		crate::Annotation {
			lang:     "de",
			tts:      Some("gelbe Blüte"),
			keywords: &["", "blume", "blüte", "gelbe blüte", "pflanze"],
		},
		#[cfg(feature = "en")]
		crate::Annotation {
			lang:     "en",
			tts:      Some("blossom"),
			keywords: &["", "buttercup", "dandelion", "flower", "plant"],
		},
		#[cfg(feature = "en-gb")]
		crate::Annotation {
			lang:     "en-gb",
			tts:      Some("blossom"),
			keywords: &["", "buttercup", "dandelion", "flower", "plant"],
		},
		#[cfg(feature = "es")]
		crate::Annotation { lang: "es", tts: Some("flor"), keywords: &["", "flor"] },
		#[cfg(feature = "es-mx")]
		crate::Annotation { lang: "es-mx", tts: Some("flor"), keywords: &["", "florecer"] },
		#[cfg(feature = "et")]
		crate::Annotation { lang: "et", tts: Some("õis"), keywords: &["", "lill", "taim"] },
		#[cfg(feature = "fi")]
		crate::Annotation {
			lang:     "fi",
			tts:      Some("kukinta"),
			keywords: &["", "kasvi", "kukka"],
		},
		#[cfg(feature = "fr")]
		crate::Annotation {
			lang:     "fr",
			tts:      Some("bourgeon"),
			keywords: &["", "bouton d’or", "dent-de-lion", "fleur", "pissenlit", "plante", "printemps"],
		},
		#[cfg(feature = "hi")]
		crate::Annotation {
			lang:     "hi",
			tts:      Some("फ\u{942}ल खिलना"),
			keywords: &["", "प\u{941}ष\u{94d}प प\u{941}\u{902}ज", "फ\u{942}ल", "सि\u{902}हपर\u{94d}णी"],
		},
		#[cfg(feature = "hu")]
		crate::Annotation {
			lang:     "hu",
			tts:      Some("virágzás"),
			keywords: &["", "növény", "virág"],
		},
		#[cfg(feature = "it")]
		crate::Annotation {
			lang:     "it",
			tts:      Some("fiore"),
			keywords: &["", "bocciolo", "fiorellino", "giallo", "margherita", "natura", "pianta"],
		},
		#[cfg(feature = "ja")]
		crate::Annotation {
			lang:     "ja",
			tts:      Some("開花"),
			keywords: &["", "キンポウゲ", "咲いた花", "植物", "花"],
		},
		#[cfg(feature = "ko")]
		crate::Annotation {
			lang:     "ko",
			tts:      Some("꽃송이"),
			keywords: &["", "꽃", "꽃이 피다", "민들레", "버터컵", "식물"],
		},
		#[cfg(feature = "lt")]
		crate::Annotation {
			lang:     "lt",
			tts:      Some("gėlės žiedas"),
			keywords: &["", "augalas", "gėlė", "kiaulpienė"],
		},
		#[cfg(feature = "ms")]
		crate::Annotation {
			lang:     "ms",
			tts:      Some("bunga mekar"),
			keywords: &["", "bunga", "dandelion", "kuning muda", "mekar", "tumbuhan"],
		},
		#[cfg(feature = "nb")]
		crate::Annotation { lang: "nb", tts: Some("blomst"), keywords: &["", "plante"] },
		#[cfg(feature = "nl")]
		crate::Annotation {
			lang:     "nl",
			tts:      Some("bloesem"),
			keywords: &["", "bloem", "boterbloem", "paardenbloem", "plant"],
		},
		#[cfg(feature = "pl")]
		crate::Annotation {
			lang:     "pl",
			tts:      Some("kwiat"),
			keywords: &["", "kwiatek", "kwitnie", "roślina"],
		},
		#[cfg(feature = "pt")]
		crate::Annotation {
			lang:     "pt",
			tts:      Some("flor"),
			keywords: &["", "florescer", "planta"],
		},
		#[cfg(feature = "ru")]
		crate::Annotation {
			lang: "ru", tts: Some("цветок"), keywords: &["", "растение"]
		},
		#[cfg(feature = "sv")]
		crate::Annotation {
			lang:     "sv",
			tts:      Some("blomma"),
			keywords: &["", "blomster", "växt"],
		},
		#[cfg(feature = "th")]
		crate::Annotation {
			lang:     "th",
			tts:      Some("ดอกไม\u{e49}บาน"),
			keywords: &["", "ดอกไม\u{e49}"],
		},
		#[cfg(feature = "uk")]
		crate::Annotation {
			lang:     "uk",
			tts:      Some("жовта квітка"),
			keywords: &["", "квітка", "ромашка", "рослина"],
		},
		#[cfg(feature = "vi")]
		crate::Annotation {
			lang:     "vi",
			tts:      Some("hoa"),
			keywords: &["", "bồ công anh", "thực vật"],
		},
		#[cfg(feature = "zh")]
		crate::Annotation { lang: "zh", tts: Some("开花"), keywords: &["", "花", "蒲公英"] },
		#[cfg(feature = "zh-hant")]
		crate::Annotation {
			lang:     "zh-hant",
			tts:      Some("開花"),
			keywords: &["", "花", "蒲公英"],
		},
	],
};
