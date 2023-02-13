//! An mdbook preprocessor that lets you render Standard Cell Reference Documentation into your book.
//!
//! # Setup
//!
//! First install this preprocessor with `cargo install mdbook-paisano-preprocessor`.
//!
//! Then add the preprocessor to your `book.toml`:
//!
//! ```toml
//! [book]
//! authors = ["You"]
//! language = "en"
//! multilingual = false
//! src = "src"
//! title = "example"
//!
//! [preprocessor.std-reference]
//! flakeroot = ./.
//! placeholder = "Standard Reference"
//! flakefragment = "__std.docs"
//!//! ```
//!
//! # Usage
//!
//! There are two ways to use Kroki in your book. First is a fenced code block:
//!
//! ``````markdown
//! ```kroki-mermaid
//! graph TD
//!   A[ Anyone ] -->|Can help | B( Go to github.com/yuzutech/kroki )
//!   B --> C{ How to contribute? }
//!   C --> D[ Reporting bugs ]
//!   C --> E[ Sharing ideas ]
//!   C --> F[ Advocating ]
//! ```
//! ``````
//!
//! The code block's language has to be `kroki-<diagram type>`.
//!
//! The other method is to use an image tag, for diagrams contents that are too big to put inline
//! in the markdown (such as for excalidraw):
//!
//! ```markdown
//! ![Excalidraw example](kroki-excalidraw:example.excalidraw)
//! ```
//!
//! The title field can be anything, but the source field needs to start with `kroki-<diagram type>:`.
//! Both relative and absolute paths are supported. Relative paths are relative to the current markdown
//! source file, *not* the root of the mdbook.
//!
//! The preprocessor will collect all Kroki diagrams of both types, send requests out in parallel
//! to the appropriate Kroki API endpoint, and replace their SVG contents back into the markdown.
//!
//! # Other
//!
//! This preprocessor only supports HTML rendering.

#[cfg(feature = "cli-install")]
mod install;

use anyhow::Result;
use clap::{Parser, Subcommand};
use mdbook::{
	errors::Error,
	preprocess::{CmdPreprocessor, Preprocessor},
};
use mdbook_paisano_preprocessor::StdReference;
#[cfg(feature = "cli-install")]
use std::path::PathBuf;
use std::{io, process};

/// mdbook preprocessor to add support for std-reference
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
	#[command(subcommand)]
	command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
	/// Check whether a renderer is supported by this preprocessor
	Supports { renderer: String },

	#[cfg(feature = "cli-install")]
	/// Install the required assset files and include it in the config
	Install {
		/// Root directory for the book, should contain the configuration file (`book.toml`)
		///
		/// If not set, defaults to the current directory.
		dir: Option<PathBuf>,

		/// Relative directory for the css assets, from the book directory root
		///
		/// If not set, defaults to the current directory.
		#[arg(long)]
		css_dir: Option<PathBuf>,
	},
}

fn main() {
	env_logger::init_from_env(
		env_logger::Env::default().default_filter_or("info"),
	);

	let cli = Cli::parse();
	if let Err(error) = run(cli) {
		log::error!("Fatal error: {}", error);
		for error in error.chain() {
			log::error!("  - {}", error);
		}
		process::exit(1);
	}
}

fn run(cli: Cli) -> Result<()> {
	match cli.command {
		None => handle_preprocessing(),
		Some(Commands::Supports { renderer }) => {
			handle_supports(renderer);
		}
		#[cfg(feature = "cli-install")]
		Some(Commands::Install { dir, css_dir }) => install::handle_install(
			dir.unwrap_or_else(|| PathBuf::from(".")),
			css_dir.unwrap_or_else(|| PathBuf::from(".")),
		),
	}
}

fn handle_preprocessing() -> Result<(), Error> {
	let (ctx, book) = CmdPreprocessor::parse_input(io::stdin())?;

	if ctx.mdbook_version != mdbook::MDBOOK_VERSION {
		eprintln!(
            "Warning: The mdbook-paisano-preprocessor preprocessor was built against version \
             {} of mdbook, but we're being called from version {}",
            mdbook::MDBOOK_VERSION,
            ctx.mdbook_version
        );
	}

	let processed_book = StdReference.run(&ctx, book)?;
	serde_json::to_writer(io::stdout(), &processed_book)?;

	Ok(())
}

fn handle_supports(renderer: String) -> ! {
	let supported = StdReference.supports_renderer(&renderer);

	// Signal whether the renderer is supported by exiting with 1 or 0.
	if supported {
		process::exit(0);
	} else {
		process::exit(1);
	}
}
