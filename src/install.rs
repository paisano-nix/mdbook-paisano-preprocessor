#[cfg(feature = "cli-install")]
// ripped from mdbook-admonish
use anyhow::{Context, Result};
use std::{
	fs::{self, File},
	io::Write,
	path::PathBuf,
};
use toml_edit::{self, Array, Document, Item, Table, Value};

const STD_REFERENCE_CSS_FILES: &[(&str, &[u8])] = &[(
	"mdbook-paisano-preprocessor.css",
	include_bytes!("../assets/mdbook-paisano-preprocessor.css"),
)];

trait ArrayExt {
	fn contains_str(
		&self,
		value: &str,
	) -> bool;
}

impl ArrayExt for Array {
	fn contains_str(
		&self,
		value: &str,
	) -> bool {
		self.iter().any(|element| match element.as_str() {
			None => false,
			Some(element_str) => element_str == value,
		})
	}
}

pub fn handle_install(
	proj_dir: PathBuf,
	css_dir: PathBuf,
) -> Result<()> {
	let config = proj_dir.join("book.toml");
	log::info!("Reading configuration file '{}'", config.display());
	let toml = fs::read_to_string(&config).with_context(|| {
		format!("can't read configuration file '{}'", config.display())
	})?;
	let mut doc = toml
		.parse::<Document>()
		.context("configuration is not valid TOML")?;

	if let Ok(preprocessor) = preprocessor(&mut doc) {
		const ASSETS_VERSION: &str = std::include_str!("../assets/VERSION");
		let value = toml_edit::value(
			toml_edit::Value::from(ASSETS_VERSION.trim()).decorated(
				" ",
				" # do not edit: managed by `mdbook-paisano-preprocessor install`",
			),
		);
		preprocessor["assets_version"] = value;
	} else {
		log::info!("Unexpected configuration, not updating prereprocessor configuration");
	};

	let mut additional_css = additional_css(&mut doc);
	for (name, content) in STD_REFERENCE_CSS_FILES {
		let filepath = proj_dir.join(css_dir.clone()).join(name);
		// Normalize path to remove no-op components
		// https://github.com/tommilligan/mdbook-admonish/issues/47
		let filepath: PathBuf = filepath.components().collect();
		let filepath_str = css_dir.join(name);
		let filepath_str: PathBuf = filepath_str.components().collect();
		let filepath_str =
			filepath_str.to_str().context("non-utf8 filepath")?;

		if let Ok(ref mut additional_css) = additional_css {
			if !additional_css.contains_str(filepath_str) {
				log::info!("Adding '{filepath_str}' to 'additional-css'");
				additional_css.push(filepath_str);
			}
		} else {
			log::warn!(
				"Unexpected configuration, not updating 'additional-css'"
			);
		}

		log::info!(
			"Copying '{name}' to '{filepath}'",
			filepath = filepath.display()
		);
		let mut file =
			File::create(&filepath).context("can't open file for writing")?;
		file.write_all(content)
			.context("can't write content to file")?;
	}

	let new_toml = doc.to_string();
	if new_toml != toml {
		log::info!("Saving changed configuration to '{}'", config.display());
		let mut file = File::create(config)
			.context("can't open configuration file for writing.")?;
		file.write_all(new_toml.as_bytes())
			.context("can't write configuration")?;
	} else {
		log::info!("Configuration '{}' already up to date", config.display());
	}

	log::info!("mdbook-paisano-preprocessor is now installed. You can start using it in your book.");
	Ok(())
}

/// Return the `additional-css` field, initializing if required.
///
/// Return `Err` if the existing configuration is unknown.
fn additional_css(doc: &mut Document) -> Result<&mut Array, ()> {
	let doc = doc.as_table_mut();

	let empty_table = Item::Table(Table::default());
	let empty_array = Item::Value(Value::Array(Array::default()));

	doc.entry("output")
		.or_insert(empty_table.clone())
		.as_table_mut()
		.and_then(|item| {
			item.entry("html")
				.or_insert(empty_table)
				.as_table_mut()?
				.entry("additional-css")
				.or_insert(empty_array)
				.as_value_mut()?
				.as_array_mut()
		})
		.ok_or(())
}

/// Return the preprocessor table for admonish, initializing if required.
///
/// Return `Err` if the existing configuration is unknown.
fn preprocessor(doc: &mut Document) -> Result<&mut Item, ()> {
	let doc = doc.as_table_mut();

	let empty_table = Item::Table(Table::default());
	let item = doc.entry("preprocessor").or_insert(empty_table.clone());
	let item = item
		.as_table_mut()
		.ok_or(())?
		.entry("paisano-preprocessor")
		.or_insert(empty_table);
	item["command"] = toml_edit::value("mdbook-paisano-preprocessor");
	Ok(item)
}
