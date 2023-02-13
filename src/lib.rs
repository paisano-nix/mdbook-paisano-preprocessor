mod reference;

use crate::reference::StdDocSchema;
use anyhow::{anyhow, bail, Context, Result};
use askama::Template;
use mdbook::book::{Book, BookItem};
use mdbook::preprocess::{Preprocessor, PreprocessorContext};
use std::process::{Command, Stdio};

pub struct StdReference;

struct BookConfig {
	multi: Vec<ChapterConfig>,
}

struct ChapterConfig {
	chapter: String,
	registry: Option<String>,
	cell: Option<String>,
}

impl TryFrom<&toml::Value> for ChapterConfig {
	type Error = anyhow::Error;
	fn try_from(val: &toml::Value) -> Result<Self, Self::Error> {
		let registry = val
			.get("registry")
			.and_then(|v| v.to_owned().try_into().ok()?);
		let chapter = val
			.get("chapter")
			.ok_or(anyhow!("Chapter:\n\n{}must contatin 'chapter'", val))?
			.to_owned()
			.try_into()?;
		let cell =
			val.get("cell").and_then(|v| v.to_owned().try_into().ok()?);
		Ok(ChapterConfig {
			registry,
			chapter,
			cell,
		})
	}
}

impl Preprocessor for StdReference {
	fn name(&self) -> &'static str {
		"paisano-preprocessor"
	}

	fn run(
		&self,
		ctx: &PreprocessorContext,
		mut book: Book,
	) -> Result<Book> {
		let root = &ctx.root;

		let mut config = BookConfig { multi: vec![] };

		if let Some(cfg) = ctx.config.get_preprocessor(self.name()) {
			match (cfg.get("chapter"), cfg.get("registry"), cfg.get("multi")) {
				(Some(chap), Some(reg), None) => match (chap, reg) {
					(
						toml::Value::String(chapter),
						toml::Value::String(registry),
					) => {
						let chapter = chapter.to_string();
						let registry = Some(registry.to_string());
						let cell = None;
						config.multi.push(ChapterConfig {
							chapter,
							registry,
							cell,
						})
					}
					_ => {
						bail!("Both options, 'chapter' and 'registry' must be strings")
					}
				},
				(None, Some(reg), Some(multi)) => match (reg, multi) {
					(
						toml::Value::String(registry),
						toml::Value::Array(values),
					) => {
						for value in values.iter() {
							let mut conf: ChapterConfig = value.try_into()?;
							if conf.registry == None {
								conf.registry = Some(registry.to_owned());
							}
							config.multi.push(conf);
						}
					}
					_ => {
						bail!("registry must be a string and multi an array")
					}
				},
				(None, None, Some(multi)) => match multi {
					toml::Value::Array(values) => {
						for value in values.iter() {
							let conf: ChapterConfig = value.try_into()?;
							config.multi.push(conf);
						}
					}
					_ => {
						bail!("multi must be an array")
					}
				},
				(Some(_), _, _) => {
					bail!("top level chapter can only be combined with top level registry")
				}
				(None, Some(_), None) => {
					bail!("top level registry cannot be configured alone")
				}
				(None, None, None) => {
					bail!("Both options, 'chapter' and 'registry' must be defined and must be strings")
				}
			}
		};
		for conf in config.multi.iter_mut() {
			let mut flakepath = root.to_path_buf();
			if let Some(split) = conf
				.registry
				.as_ref()
				.expect("registry set")
				.split_once('#')
			{
				flakepath.push(split.0);
				conf.registry = Some(format!(
					"{}#{}",
					flakepath.canonicalize().unwrap().display(),
					split.1.to_owned()
				));
			} else {
				bail!("registry must be a flake url and contain a '#'")
			}
		}
		for conf in config.multi.iter() {
			log::info!(
				"Chapter '{}' in {}/SUMMARY.md",
				conf.chapter,
				root.display(),
			);

			log::info!(
				"Flake URL: '{}'",
				conf.registry.as_ref().expect("registry set")
			);

			if conf.cell.is_some() {
				log::info!("Cell Name: '{}'", conf.cell.as_ref().unwrap());
			}

			let stdout =
				eval(conf.registry.as_ref().expect("registry set").to_owned())?;
			let mut docs: StdDocSchema = serde_json::from_str(&stdout)?;

			if conf.cell.is_some() {
				docs.0
					.retain(|cell| cell.name == conf.cell.as_ref().unwrap());
			}
			let docstr = docs.render()?;

			book.for_each_mut(|section: &mut BookItem| {
				if let BookItem::Chapter(ref mut chapter) = *section {
					if chapter.name == conf.chapter {
						chapter.content.push_str("\n\n");
						chapter.content.push_str(&docstr);
					}
				}
			});
		}

		Ok(book)
	}

	fn supports_renderer(
		&self,
		renderer: &str,
	) -> bool {
		renderer == "html"
	}
}

fn eval(flakeurl: String) -> Result<String> {
	let cmd = "nix";

	// TODO: check schema
	let mut sysevl = Command::new(&cmd);
	sysevl
		.arg("eval")
		.arg("--raw")
		.arg("--impure")
		.arg("--expr")
		.arg("builtins.currentSystem")
		.stdout(Stdio::piped())
		.stderr(Stdio::null());

	let sysout = sysevl
		.output()
		.context("failed to obatin current nix system")?;
	let system = String::from_utf8(sysout.stdout)?;

	let mut eval = Command::new(&cmd);
	eval.arg("eval")
		.arg("--json")
		.arg(format!("{}.{}", flakeurl, system))
		.stdout(Stdio::piped())
		.stderr(Stdio::null());

	let out = eval
		.output()
		.context(format!("failed to evaluate nix to json from {}", flakeurl))?;
	let stdout = String::from_utf8(out.stdout)?;

	Ok(stdout)
}
