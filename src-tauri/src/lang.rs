use serde::Serialize;
use whatlang::{detect, Lang};

#[derive(Debug, Clone, Serialize)]
pub struct LanguageOption {
    pub code: String,
    pub name: String,
    pub native_name: String,
}

pub fn supported() -> Vec<LanguageOption> {
    vec![
        ("auto", "Auto detect", "Auto"),
        ("en", "English", "English"),
        ("ru", "Russian", "Русский"),
        ("es", "Spanish", "Español"),
        ("de", "German", "Deutsch"),
        ("fr", "French", "Français"),
        ("it", "Italian", "Italiano"),
        ("pt", "Portuguese", "Português"),
        ("nl", "Dutch", "Nederlands"),
        ("pl", "Polish", "Polski"),
        ("uk", "Ukrainian", "Українська"),
        ("tr", "Turkish", "Türkçe"),
        ("ja", "Japanese", "日本語"),
        ("zh", "Chinese (Simplified)", "中文"),
        ("ko", "Korean", "한국어"),
        ("ar", "Arabic", "العربية"),
        ("hi", "Hindi", "हिन्दी"),
        ("id", "Indonesian", "Bahasa Indonesia"),
        ("vi", "Vietnamese", "Tiếng Việt"),
        ("th", "Thai", "ไทย"),
        ("he", "Hebrew", "עברית"),
        ("cs", "Czech", "Čeština"),
        ("sv", "Swedish", "Svenska"),
        ("fi", "Finnish", "Suomi"),
        ("da", "Danish", "Dansk"),
        ("no", "Norwegian", "Norsk"),
        ("el", "Greek", "Ελληνικά"),
        ("ro", "Romanian", "Română"),
        ("hu", "Hungarian", "Magyar"),
    ]
    .into_iter()
    .map(|(c, n, nn)| LanguageOption {
        code: c.to_string(),
        name: n.to_string(),
        native_name: nn.to_string(),
    })
    .collect()
}

pub fn lookup_name(code: &str) -> String {
    supported()
        .into_iter()
        .find(|l| l.code == code)
        .map(|l| l.name)
        .unwrap_or_else(|| code.to_string())
}

pub fn detect_language(text: &str) -> Option<String> {
    detect(text).map(|info| lang_to_iso(info.lang()).to_string())
}

fn lang_to_iso(lang: Lang) -> &'static str {
    match lang {
        Lang::Eng => "en",
        Lang::Rus => "ru",
        Lang::Spa => "es",
        Lang::Deu => "de",
        Lang::Fra => "fr",
        Lang::Ita => "it",
        Lang::Por => "pt",
        Lang::Nld => "nl",
        Lang::Pol => "pl",
        Lang::Ukr => "uk",
        Lang::Tur => "tr",
        Lang::Jpn => "ja",
        Lang::Cmn => "zh",
        Lang::Kor => "ko",
        Lang::Ara => "ar",
        Lang::Hin => "hi",
        Lang::Ind => "id",
        Lang::Vie => "vi",
        Lang::Tha => "th",
        Lang::Heb => "he",
        Lang::Ces => "cs",
        Lang::Swe => "sv",
        Lang::Fin => "fi",
        Lang::Dan => "da",
        Lang::Ell => "el",
        Lang::Ron => "ro",
        Lang::Hun => "hu",
        Lang::Nob => "no",
        _ => "auto",
    }
}
