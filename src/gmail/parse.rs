use anyhow::{Context, Ok, Result};
use roxmltree::Document;

pub fn get_unread_count(xml_text: &str) -> Result<usize> {
    let doc = Document::parse(&xml_text).with_context(|| "xml parse error")?;
    let full_count_node = doc
        .descendants()
        .find(|n| n.has_tag_name("fullcount"))
        .with_context(|| "could not find <fullcount> tag")?;

    let count_str = full_count_node.text().unwrap_or("0");
    let count: usize = count_str.parse().unwrap_or(0);
    Ok(count)
}
