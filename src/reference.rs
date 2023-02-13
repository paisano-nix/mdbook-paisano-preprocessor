use askama::Template;
use serde::Deserialize;

#[derive(Deserialize, Template)]
#[template(
	source = "{% for cell in self.0.iter() -%} {{ cell|safe }} {%- endfor %}",
	ext = "html"
)]
pub struct StdDocSchema<'a>(#[serde(borrow)] pub Vec<StdCell<'a>>);

#[derive(Debug, Deserialize, Template)]
#[template(path = "cell.md")]
pub struct StdCell<'a> {
	#[serde(rename(deserialize = "cell"))]
	pub name: &'a str,
	readme: Option<&'a str>,
	#[serde(borrow)]
	#[serde(rename(deserialize = "cellBlocks"))]
	blocks: Vec<StdBlock<'a>>,
}

#[derive(Debug, Deserialize, Template)]
#[template(path = "block.md")]
#[serde(rename_all = "camelCase")]
struct StdBlock<'a> {
	#[allow(dead_code)]
	#[serde(rename(deserialize = "blockType"))]
	r#type: &'a str,
	#[serde(rename(deserialize = "cellBlock"))]
	name: &'a str,
	readme: Option<&'a str>,
	#[serde(borrow)]
	targets: Vec<StdTarget<'a>>,
}

#[derive(Debug, Deserialize, Template)]
#[template(path = "target.md", escape = "none")]
struct StdTarget<'a> {
	name: &'a str,
	description: Option<&'a str>,
	readme: Option<&'a str>,
	#[serde(borrow)]
	#[allow(dead_code)]
	actions: Vec<StdAction<'a>>,
}

#[derive(Debug, Deserialize, Template)]
#[template(path = "action.md", escape = "none")]
struct StdAction<'a> {
	#[allow(dead_code)]
	name: &'a str,
	#[allow(dead_code)]
	description: &'a str,
}

// Any filter defined in the module `filters` is accessible in your template.
mod filters {
	use pulldown_cmark::{Event, HeadingLevel, Options, Parser, Tag};
	use pulldown_cmark_to_cmark::cmark;

	// This filter does not have extra arguments
	pub fn read_file<
		T: std::fmt::Display + std::convert::AsRef<std::path::Path>,
	>(
		s: T
	) -> ::askama::Result<String> {
		std::fs::read_to_string(s)
			.or_else(|_| Err(askama::Error::Fmt(std::fmt::Error)))
	}

	// This filter requires a `usize` input when called in templates
	pub fn offset_headers<T: std::fmt::Display>(
		s: T,
		offset: usize,
	) -> ::askama::Result<String> {
		let markdown = s.to_string();
		let mut buffer = String::with_capacity(markdown.len());

		let events = Parser::new_ext(&markdown, Options::all())
			.map(|mut e| match e {
				Event::Start(Tag::Heading(ref mut heading_level, _, _)) => {
					let modified =
						std::cmp::min(*heading_level as usize + offset, 6);
					*heading_level = HeadingLevel::try_from(modified).unwrap();
					e
				}
				Event::End(Tag::Heading(ref mut heading_level, _, _)) => {
					let modified =
						std::cmp::min(*heading_level as usize + offset, 6);
					*heading_level = HeadingLevel::try_from(modified).unwrap();
					e
				}
				e => e,
			})
			.collect::<Vec<Event>>();

		cmark(events.iter(), &mut buffer)
			.or_else(|_| Err(askama::Error::Fmt(std::fmt::Error)))?;

		Ok(buffer)
	}
}
