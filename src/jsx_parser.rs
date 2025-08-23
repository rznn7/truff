use proc_macro2::TokenStream;
use quote::quote;

pub fn parse_element(input: &str) -> Option<TokenStream> {
    let tag_name = extract_tag_name(input)?;
    let attributes = extract_attrs(input).unwrap_or_default();

    let mut result = quote! { El::new(#tag_name) };

    for (key, value) in attributes {
        result = quote! { #result.attr(#key, #value) };
    }

    if let Some(text_content) = extract_text_content(input) {
        result = quote! { #result.text(#text_content) };
    }

    Some(result)
}

fn extract_tag_name(input: &str) -> Option<&str> {
    let start = input.find('<')? + 1;
    let remaining = &input[start..];
    let end_pos = remaining.find([' ', '>'].as_ref())?;
    let tag_name = &remaining[..end_pos];

    if tag_name.is_empty() {
        None
    } else {
        Some(tag_name)
    }
}

fn extract_attrs(input: &str) -> Option<Vec<(&str, &str)>> {
    let start = input.find(' ')? + 1;
    let end = input.find('>')?;
    let attributes_part = &input[start..end];
    let attribute_parts = attributes_part.split(' ').collect::<Vec<&str>>();

    let attrs = attribute_parts
        .iter()
        .map(|&part| {
            let equal_index = part.find('=').unwrap();
            let name = &part[..equal_index];
            let value = part[equal_index + 1..].trim_matches('"');
            (name, value)
        })
        .collect::<Vec<(&str, &str)>>();

    Some(attrs)
}

fn extract_text_content(input: &str) -> Option<&str> {
    let start = input.find('>')? + 1;
    let end = input.find("</")?;
    let text_content = &input[start..end];

    if text_content.is_empty() {
        None
    } else {
        Some(text_content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_element_wihtout_name() {
        let result = parse_element("<></>");
        assert!(result.is_none());
    }

    #[test]
    fn test_simple_element() {
        let result = parse_element("<div></div>").unwrap();
        let expected = quote! { El::new("div") };
        assert_eq!(expected.to_string(), result.to_string());
    }

    #[test]
    fn test_element_with_attr() {
        let result = parse_element(r#"<div class="container"></div>"#).unwrap();
        let expected = quote! { El::new("div").attr("class", "container") };
        assert_eq!(expected.to_string(), result.to_string());
    }

    #[test]
    fn test_element_with_attrs() {
        let result = parse_element(r#"<div id="id-container" class="container"></div>"#).unwrap();
        let expected =
            quote! { El::new("div").attr("id", "id-container").attr("class", "container") };
        assert_eq!(expected.to_string(), result.to_string());
    }

    #[test]
    fn test_element_with_text_content() {
        let result = parse_element(r#"<div>testo</div>"#).unwrap();
        let expected = quote! { El::new("div").text("testo") };
        assert_eq!(expected.to_string(), result.to_string());
    }

    #[test]
    fn test_element_with_text_content_and_attr() {
        let result = parse_element(r#"<div class="container">testo</div>"#).unwrap();
        let expected = quote! { El::new("div").attr("class", "container").text("testo") };
        assert_eq!(expected.to_string(), result.to_string());
    }
}
