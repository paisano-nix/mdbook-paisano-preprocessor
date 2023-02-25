//! An mdbook preprocessor that lets you render Paisano Cell Reference Documentation into your book.
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
//! [preprocessor.paisano-preprocessor]
//! # The chapter in SUMMARY.md to which
//! # the render will be appended
//! chapter = "My Cell Reference"
//! registry = "..#__std.init"
//! ```
//!
//! # Set Up Selected Cells
//! ```toml
//! [book]
//! authors = ["You"]
//! language = "en"
//! multilingual = false
//! src = "src"
//! title = "example"
//!
//! [preprocessor.paisano-preprocessor]
//! registry = "..#__std.init"
//! [[multi]]
//! chapter = "Foo Cell"
//! cell = "foo"
//! [[multi]]
//! chapter = "Bar Cell"
//! cell = "bar"
//! ```
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
