// Copyright 2015-2017 Parity Technologies (UK) Ltd.
// This file is part of Parity.

// Parity is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity.  If not, see <http://www.gnu.org/licenses/>.

//! Parity Brain Wallet Generator.

#![warn(missing_docs)]

#[macro_use]
extern crate lazy_static;

extern crate itertools;

#[cfg(target_arch = "wasm32")]
extern crate wasm_bindgen;
//#[cfg(all(test, target_arch = "wasm32"))]
//extern crate wasm_bindgen_test;

use std::fmt;
use std::collections::HashSet;

pub use random_phrase::random_phrase;

/// The list of dictionary words.
// the wordlist JSON also happens to be valid Rust syntax for an array constant.
pub const WORDS: &'static [&'static str] = &include!("../res/wordlist.json");

#[cfg(not(target_arch = "wasm32"))]
mod random_phrase {
	extern crate rand;
	use self::rand::{Rng, OsRng};
	use itertools::Itertools;
	use ::WORDS;
	/// Generate a string which is a random phrase of a number of lowercase words.
	///
	/// `words` is the number of words, chosen from a dictionary of 7,530. An value of
	/// 12 gives 155 bits of entropy (almost saturating address space); 20 gives 258 bits
	/// which is enough to saturate 32-byte key space
	pub fn random_phrase(no_of_words: usize) -> String {
		let mut rng = OsRng::new().expect("Not able to operate without random source.");
		(0..no_of_words).map(|_| rng.choose(WORDS).unwrap()).join(" ")
	}
}

#[cfg(all(target_arch = "wasm32", feature = "browser-wasm"))]
mod random_phrase {
	extern crate web_sys;
	use self::web_sys::{Crypto};
	use ::WORDS;
	use wasm_bindgen::prelude::*;
	use itertools::Itertools;

	#[wasm_bindgen]
	/// Generate a string which is a random phrase of a number of lowercase words.
	///
	/// `words` is the number of words, chosen from a dictionary of 7,530. An value of
	/// 12 gives 155 bits of entropy (almost saturating address space); 20 gives 258 bits
	/// which is enough to saturate 32-byte key space
	pub fn random_phrase(no_of_words: usize) -> String {
		let nb_words = WORDS.len();
		use std::mem;
		let mut buf = [0u8; mem::size_of::<u16>()];
		let crypto: Crypto = self::web_sys::window().unwrap().crypto().unwrap();
		let mut choose = || {
			crypto.get_random_values_with_u8_array(&mut buf[..]).expect("Not able to operate without random source.");
			// unsafe transmute more efficient but not worth this usecase, and not
			// adding an additional deps for it.
			let rand_val = ((buf[0] as usize) + (buf[1] as usize) * 2^8) % nb_words;
			WORDS[rand_val as usize]
		};
		(0..no_of_words).map(|_| choose()).join(" ")
	}

}

#[cfg(all(target_arch = "wasm32", not(feature = "browser-wasm")))]
mod random_phrase {
	use wasm_bindgen::prelude::*;

	#[wasm_bindgen]
	pub fn random_phrase(no_of_words: usize) -> String {
    // TODO thereis a crate to use non crypto rand on node : we could use it for test purpose only
    // TODO substrate/pwasm extrinsec with other name if needed
    unimplemented!()
  }
}

#[cfg(target_arch = "wasm32")]
/// simple function reexport with bindgen
mod wasm_reexport {
	use wasm_bindgen::prelude::*;
  use wasm_bindgen::JsValue;

	#[wasm_bindgen]
  pub fn validate_phrase(phrase: &str, expected_no_of_words: usize) -> Result<(), JsValue> {
    ::validate_phrase(phrase, expected_no_of_words).map_err(|e|format!("{:?}",&e).into())
  }
}

/// Phrase Validation Error
#[derive(Debug, Clone, PartialEq)]
pub enum Error {
	/// Phrase is shorter than it was expected.
	PhraseTooShort(usize),
	/// Phrase contains a word that doesn't come from our dictionary.
	WordNotFromDictionary(String),
}

impl fmt::Display for Error {
	fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			Error::PhraseTooShort(len) => writeln!(fmt, "The phrase is too short ({})", len),
			Error::WordNotFromDictionary(ref word) => writeln!(fmt, "The word '{}' does not come from the dictionary.", word),
		}
	}
}

/// Validates given phrase and checks if:
/// 1. All the words are coming from the dictionary.
/// 2. There are at least `expected_no_of_words` in the phrase.
pub fn validate_phrase(phrase: &str, expected_no_of_words: usize) -> Result<(), Error> {
	lazy_static! {
		static ref WORD_SET: HashSet<&'static str> = WORDS.iter().cloned().collect();
	}

	let mut len = 0;
	for word in phrase.split_whitespace() {
		len += 1;
		if !WORD_SET.contains(word) {
			return Err(Error::WordNotFromDictionary(word.into()));
		}
	}

	if len < expected_no_of_words {
		return Err(Error::PhraseTooShort(len));
	}

	return Ok(());
}

#[cfg(test)]
pub mod tests {

	#[cfg(all(test, target_arch = "wasm32"))]
	use wasm_bindgen_test::*;

	use super::{validate_phrase, Error};
	use super::random_phrase::random_phrase;

	//#[wasm_bindgen_test]
	#[test]
	fn should_produce_right_number_of_words() {
		let p = random_phrase(10);
		assert_eq!(p.split(" ").count(), 10);
	}

	//#[wasm_bindgen_test]
	#[test]
	fn should_not_include_carriage_return() {
		let p = random_phrase(10);
		assert!(!p.contains('\r'), "Carriage return should be trimmed.");
	}

	#[test]
	fn should_validate_the_phrase() {
		let p = random_phrase(10);

		assert_eq!(validate_phrase(&p, 10), Ok(()));
		assert_eq!(validate_phrase(&p, 12), Err(Error::PhraseTooShort(10)));
		assert_eq!(validate_phrase("xxx", 0), Err(Error::WordNotFromDictionary("xxx".into())));
	}
}

