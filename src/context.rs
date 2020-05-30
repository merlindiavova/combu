use crate::parser::{ErrorInfo, FlagArg, ParseError};
use crate::Vector;
use crate::{CalledType, Flag, FlagValue};
use std::collections::VecDeque;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct Context {
    pub raw_args: Vec<String>,
    pub args: VecDeque<String>,
    pub common_flags: Vector<Flag>,
    pub local_flags: Vector<Flag>,
    pub current_path: PathBuf,
    pub common_flags_values: Vector<(String, FlagValue)>,
    pub local_flags_values: Vector<(String, FlagValue)>,
    pub parsing_flags: Vector<FlagArg>,
    pub error_info_list: Vector<ErrorInfo>,
}

impl Context {
    pub fn new(
        raw_args: Vec<String>,
        args: VecDeque<String>,
        common_flags: Vector<Flag>,
        local_flags: Vector<Flag>,
        current_path: &str,
    ) -> Context {
        Context {
            raw_args,
            args,
            common_flags,
            local_flags,
            current_path: PathBuf::from(current_path),
            common_flags_values: Vector::default(),
            local_flags_values: Vector::default(),
            parsing_flags: Vector::default(),
            error_info_list: Vector::default(),
        }
    }
    pub fn build_new(
        raw_args: Vec<String>,
        args: VecDeque<String>,
        common_flags: Vector<Flag>,
        local_flags: Vector<Flag>,
        current_path: PathBuf,
        common_flags_values: Vector<(String, FlagValue)>,
        local_flags_values: Vector<(String, FlagValue)>,
        parsing_flags: Vector<FlagArg>,
        error_info_list: Vector<ErrorInfo>,
    ) -> Context {
        Context {
            raw_args,
            args,
            common_flags,
            local_flags,
            current_path,
            common_flags_values,
            local_flags_values,
            parsing_flags,
            error_info_list,
        }
    }

    pub fn root<'a>() -> Option<Context> {
        None
    }

    pub fn args(mut self, args: VecDeque<String>) -> Self {
        self.args = args;
        self
    }

    #[inline]
    pub fn next_arg(&mut self) -> Option<String> {
        self.args.pop_front()
    }

    pub fn current(&self) -> &Path {
        &self.current_path
    }

    pub fn change_current(mut self, path: PathBuf) {
        self.current_path = path;
    }

    pub fn find_local_long_flag(&self, name_or_alias: &str) -> (CalledType, Option<&Flag>) {
        self.local_flags.find_long_flag(name_or_alias)
    }

    pub fn find_local_short_flag(&self, short_alias: &str) -> (CalledType, Option<&Flag>) {
        self.local_flags.find_short_flag(short_alias)
    }

    pub fn find_common_long_flag(&self, name_or_alias: &str) -> (CalledType, Option<&Flag>) {
        self.common_flags.find_long_flag(name_or_alias)
    }

    pub fn find_common_short_flag(&self, short_alias: &str) -> (CalledType, Option<&Flag>) {
        self.common_flags.find_short_flag(short_alias)
    }
}

impl<'a> From<Vec<String>> for Context {
    fn from(raw_args: Vec<String>) -> Context {
        let args = VecDeque::from(raw_args.clone());
        let current_path = match &raw_args.get(0) {
            Some(str) => PathBuf::from(str),
            None => PathBuf::new(),
        };
        Context {
            raw_args,
            args,
            common_flags: Vector::default(),
            local_flags: Vector::default(),
            current_path,
            common_flags_values: Vector::default(),
            local_flags_values: Vector::default(),
            parsing_flags: Vector::default(),
            error_info_list: Vector::default(),
        }
    }
}
