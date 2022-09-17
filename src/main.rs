use clap::Parser;
use reqwest::blocking::Client;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Definition {
	definition: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Meaning {
	part_of_speech: String,
	definitions: Vec<Definition>,
}

#[derive(Deserialize, Debug)]
struct Entry {
	word: String,
	phonetic: Option<String>,
	meanings: Vec<Meaning>,
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct App {
	/// Word to look up
	#[clap(about, index = 1)]
	word: String,
}

fn main() {
	let client = Client::builder()
		.user_agent("Meaning CLI")
		.build()
		.expect("Failed to build HTTP client.");
	let args = App::parse();
	let response = client
		.get(format!(
			"https://api.dictionaryapi.dev/api/v2/entries/en/{}",
			args.word
		))
		.send()
		.expect("Failed to send request.")
		.json::<Vec<Entry>>();
	match response {
		Err(error) => println!("Error: {}", error),
		Ok(entries) => {
			if entries.is_empty() {
				println!("No entries found")
			} else {
				let mut first = true;
				for entry in entries.iter() {
					for meaning in entry.meanings.iter() {
						for definition in meaning.definitions.iter() {
							if !first {
								println!();
							}
							first = false;
							println!("Word: {}", entry.word);
							println!(
								"Pronounciation: {}",
								&entry
									.phonetic
									.as_ref()
									.map(|p| p.as_str())
									.unwrap_or("None found")
							);
							println!("Gramatical category: {}", meaning.part_of_speech);
							println!("Definition: {}", definition.definition);
						}
					}
				}
			}
		}
	}
}
