use clap::{App, Arg};
mod wrap;

fn main() {
	let matches = App::new("open-attestation")
		.version("1.0")
		.subcommand(
			App::new("wrap")
				.about("wrap documeent")
				.version("0.1")
				.author("nebulis <nebounet@gmail.com>")
				.arg(
					Arg::with_name("input")
						.index(1)
						.required(true)
						.help("input bla"),
				)
				.arg(
					Arg::with_name("output")
						.index(2)
						.required(true)
						.help("input bla"),
				),
		)
		.get_matches();

	match matches.subcommand() {
		("wrap", Some(wrap_matches)) => {
			wrap::wrap(wrap_matches.value_of("input").unwrap(), wrap_matches.value_of("output").unwrap())
		}
		("", None) => println!("No subcommand was used"), // If no subcommand was usd it'll match the tuple ("", None)
		_ => unreachable!(), // If all subcommands are defined above, anything else is unreachable!()
	}
}
