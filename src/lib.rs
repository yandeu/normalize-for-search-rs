#[doc = include_str!("../README.md")]
extern crate unicode_normalization;
use regex::Regex;
use unicode_normalization::UnicodeNormalization;

fn custom_normalize(c: char) -> String {
    match c {
        'æ' => String::from("ae"),
        'œ' => String::from("oe"),
        'ß' => String::from("ss"),
        _ => c.to_string(),
    }
}

pub fn normalize_for_search<T: Into<String>>(text: T) -> String {
    let mut t: String = text.into();

    // lowerCase
    t = t.to_lowercase();
    // trim
    t = t.trim().to_string();
    // standard normalization
    let re = Regex::new(r"[\u0300-\u036F]").unwrap();
    t = t.nfkd().collect::<String>();
    t = re.replace_all(&t, "").to_string();

    // custom normalization
    let mut normalized = String::new();
    for (_i, c) in t.chars().enumerate() {
        normalized.push_str(&custom_normalize(c))
    }

    // replace ae, oe, ue
    normalized = normalized.replace("ae", "a");
    normalized = normalized.replace("oe", "o");
    normalized = normalized.replace("ue", "u");

    // remove too many spaces
    let re2 = Regex::new(r"\s+").unwrap();
    normalized = re2.replace_all(&normalized, " ").to_string();

    normalized.to_owned()
}
