#![feature(slice_concat_ext)]

use unicode_segmentation::UnicodeSegmentation;
use std::slice::SliceConcatExt;

extern crate unicode_segmentation;

pub fn normalize_tag(tag: &str) -> Result<String, &'static str> {

    if tag.is_empty() {
        Err("Tag is empty")
    } else {
        let lc = tag.to_lowercase();
        let words: Vec<&str> = lc.unicode_words().collect();
        let joined = words.join("-");
        Ok(joined)
    }
}

#[test]
fn test_tag_normalize() {
    assert!(normalize_tag("").is_err());
    assert_eq!(Ok("abc".to_string()), normalize_tag("ABC"));
    assert_eq!(Ok("abc-this-is-a-tag".to_string()), normalize_tag("ABC this is a tag\""));
    assert_eq!(Ok("abc-this-is-a-tag".to_string()), normalize_tag("   ABC this     is a tag\""));
    assert_eq!(Ok("abc-this-is-a-tag".to_string()), normalize_tag("--   ABC this     is a tag--\""));
}
