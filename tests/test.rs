use normalize_for_search::normalize_for_search;

#[test]
fn french_characters() {
    // la cédille
    assert_eq!(normalize_for_search("ç"), "c");
    // l'accent aigu
    assert_eq!(normalize_for_search("é"), "e");
    // l'accent circonflexe
    assert_eq!(normalize_for_search("â/ê/î/ô/û"), "a/e/i/o/u");
    // l'accent grave
    assert_eq!(normalize_for_search("à/è/ì/ò/ù"), "a/e/i/o/u");
    // l'accent tréma
    assert_eq!(normalize_for_search("ë/ï/ü"), "e/i/u");
    // o-e entrelacé
    assert_eq!(normalize_for_search("œ"), "o");
    assert_eq!(normalize_for_search("oe"), "o");
}

#[test]
fn german_characters() {
    // umlaut
    assert_eq!(normalize_for_search("ä/ö/ü"), "a/o/u");
    assert_eq!(normalize_for_search("ae/oe/ue"), "a/o/u");
    // eszett
    assert_eq!(normalize_for_search("ß"), "ss");
    assert_eq!(normalize_for_search("ss"), "ss");
}

#[test]
fn simple_test() {
    let mut result;

    // empty string
    result = String::from("");
    assert_eq!(normalize_for_search(""), result);
    assert_eq!(normalize_for_search(" "), result);

    // spaces
    result = String::from("celine diaz");
    assert_eq!(normalize_for_search(" Céline   Díaz"), result);

    // umlaut
    result = String::from("zmorgele");
    assert_eq!(normalize_for_search("Zmörgele"), result);
    assert_eq!(normalize_for_search("Zmorgele"), result);
    assert_eq!(normalize_for_search("Zmoergele"), result);

    // accents
    result = String::from("jerome");
    assert_eq!(normalize_for_search("Jérôme"), result);
    assert_eq!(normalize_for_search("jeroeme"), result);
    assert_eq!(normalize_for_search("Jèröme"), result);
    assert_eq!(normalize_for_search("Jerome"), result);
}
