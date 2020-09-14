use combu::{ActionResult, Command, Flag, FlagValue};
use std::env;

fn main() {
	Command::new()
		.name(env!("CARGO_PKG_NAME"))
		.authors(env!("CARGO_PKG_AUTHORS"))
		.version(env!("CARGO_PKG_VERSION"))
		.usage(env!("CARGO_PKG_NAME").to_string() + " [args]")
		.common_flag(Flag::new_bool("help").short_alias('h'))
		.action(|c| {
			if Some(FlagValue::Bool(true)) == c.get_flag_value_of("help") {
				return Ok(ActionResult::ShowHelpRequest(c));
			}
			println!("Hello, combu - {:?}", c.args);

			Ok(ActionResult::Done)
		})
		.run_from_args(env::args().collect())
}
