// Copyright ⓒ 2015-2018 Kevin B. Knapp
//
// `clap_complete` is distributed under the terms of both the MIT license and the Apache License
// (Version 2.0).
// See the [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT) files in this repository
// for more information.

#![doc(html_logo_url = "https://raw.githubusercontent.com/clap-rs/clap/master/assets/clap.png")]
#![doc = include_str!("../README.md")]
#![warn(missing_docs, trivial_casts, unused_allocation, trivial_numeric_casts)]
#![forbid(unsafe_code)]
#![allow(clippy::needless_doctest_main)]

//! ## Quick Start
//!
//! - For generating at compile-time, see [`generate_to`]
//! - For generating at runtime, see [`generate`]
//!
//! [`Shell`] is a convenience `enum` for an argument value type that implements `Generator`
//! for each natively-supported shell type.
//!
//! ## Example
//!
//! ```rust,no_run
//! use clap::{App, AppSettings, Arg, ValueHint};
//! use clap_complete::{generate, Generator, Shell};
//! use std::io;
//!
//! fn build_cli() -> App<'static> {
//!     App::new("example")
//!          .arg(Arg::new("file")
//!              .help("some input file")
//!                 .value_hint(ValueHint::AnyPath),
//!         )
//!        .arg(
//!            Arg::new("generator")
//!                .long("generate")
//!                .possible_values(Shell::possible_values()),
//!        )
//! }
//!
//! fn print_completions<G: Generator>(gen: G, app: &mut App) {
//!     generate(gen, app, app.get_name().to_string(), &mut io::stdout());
//! }
//!
//! fn main() {
//!     let matches = build_cli().get_matches();
//!
//!     if let Ok(generator) = matches.value_of_t::<Shell>("generator") {
//!         let mut app = build_cli();
//!         eprintln!("Generating completion file for {}...", generator);
//!         print_completions(generator, &mut app);
//!     }
//! }
//! ```

const INTERNAL_ERROR_MSG: &str = "Fatal internal error. Please consider filing a bug \
                                  report at https://github.com/clap-rs/clap/issues";

#[macro_use]
#[allow(missing_docs)]
mod macros;

pub mod generator;
pub mod shells;

pub use generator::generate;
pub use generator::generate_to;
pub use generator::Generator;
pub use shells::Shell;
