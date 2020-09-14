use combu::{ActionError, ActionResult, Command, Context, Flag, FlagValue};

fn main() {
	root_command().run_from_args(std::env::args().collect())
}

fn root_command() -> Command {
	Command::with_name("multi")
		.common_flag(Flag::new_bool("help").short_alias('h'))
		.common_flag(Flag::new_bool("reverse").short_alias('r'))
		.local_flag(Flag::new_bool("by-char").short_alias('c'))
		.action(print_args)
		.sub_command(add_command())
		.sub_command(sub_command())
}
fn call_help(c: Context) -> Result<ActionResult, ActionError> {
	Ok(ActionResult::ShowHelpRequest(c))
}
fn print_args(context: Context) -> Result<ActionResult, ActionError> {
	if called_help(&context) {
		return call_help(context);
	}
	let r: bool = context.get_flag_value_of("reverse") == Some(FlagValue::Bool(true));
	let c: bool = context.get_flag_value_of("by-char") == Some(FlagValue::Bool(true));
	let str = {
		let str = if r && !c {
			context
				.args
				.iter()
				.rev()
				.fold(String::new(), |c, arg| c + arg)
		} else {
			context.args.iter().fold(String::new(), |c, arg| c + arg)
		};
		if c {
			str.chars().rev().collect::<String>()
		} else {
			str
		}
	};

	println!("{}", str);

	Ok(ActionResult::Done)
}

fn called_help(c: &Context) -> bool {
	Some(FlagValue::Bool(true)) == c.get_flag_value_of("help")
}

fn add_command() -> Command {
	Command::new()
		.name("add")
		.alias("a")
		.action(add_action)
		.local_flag(Flag::new_bool("detail").short_alias('d'))
}

fn add_action(c: Context) -> Result<ActionResult, ActionError> {
	if called_help(&c) {
		return call_help(c);
	}
	let f = |(str, sum), num: f64| (format!("{} {} +", str, num), sum + num);
	let (mut str, sum): (String, f64) =
		if c.get_flag_value_of("reverse") == Some(FlagValue::Bool(true)) {
			c.args
				.iter()
				.rev()
				.filter_map(|arg| arg.parse().ok())
				.fold((String::new(), 0.0), f)
		} else {
			c.args
				.iter()
				.filter_map(|arg| arg.parse().ok())
				.fold((String::new(), 0.0), f)
		};
	str.pop();
	str.pop();

	if c.get_flag_value_of("detail").unwrap().is_bool_true() {
		println!("{} = {}", str, sum);
	} else {
		println!("{}", sum);
	}
	Ok(ActionResult::Done)
}

fn sub_command() -> Command {
	Command::new()
		.name("sub")
		.alias("s")
		.action(sub_action)
		.local_flag(Flag::new_bool("sort").short_alias('s'))
}

fn sub_action(c: Context) -> Result<ActionResult, ActionError> {
	if called_help(&c) {
		return call_help(c);
	}
	let f = |(str, sum), (index, num): (usize, f64)| {
		(
			format!("{} {} -", str, num),
			if index < 1 { num } else { sum - num },
		)
	};
	let filter_map_f = |arg: &String| arg.parse().ok();
	let (mut str, result): (String, f64) =
		if c.get_flag_value_of("reverse") == Some(FlagValue::Bool(true)) {
			c.args
				.iter()
				.rev()
				.filter_map(filter_map_f)
				.enumerate()
				.fold((String::new(), 0.0), f)
		} else if c.get_flag_value_of("sort").unwrap().is_bool_true() {
			let mut fvec = c.args.iter().filter_map(filter_map_f).collect::<Vec<f64>>();
			fvec.sort_by(|a, b| a.partial_cmp(b).unwrap());
			fvec
				.iter_mut()
				.enumerate()
				.fold((String::new(), 0.0), |s, (index, fl)| f(s, (index, *fl)))
		} else {
			c.args
				.iter()
				.filter_map(filter_map_f)
				.enumerate()
				.fold((String::new(), 0.0), f)
		};
	str.pop();
	str.pop();

	println!("{} = {}", str, result);

	Ok(ActionResult::Done)
}
