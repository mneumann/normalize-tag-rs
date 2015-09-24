extern crate unicode_segmentation;

pub fn normalize_tag(tag: &str) -> Result<String, &'static str> {
    if tag.is_empty() {
        Err("Tag is empty")
    } else {
        Ok(tag.to_lowercase())
    }
}

#[test]
fn test_tag_normalize() {
    assert!(normalize_tag("").is_err());
    assert_eq!(Ok("abc".to_string()), normalize_tag("ABC"));
}
