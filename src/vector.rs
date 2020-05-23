use crate::{CalledType, Flag};

#[derive(Clone, Debug)]
pub struct Vector<T>(pub Option<Vec<T>>);

impl<T> Default for Vector<T> {
    fn default() -> Self {
        Vector(None)
    }
}

impl<T> From<Vec<T>> for Vector<T> {
    fn from(vec: Vec<T>) -> Self {
        Vector(Some(vec))
    }
}

impl<T> From<Option<Vec<T>>> for Vector<T> {
    fn from(val: Option<Vec<T>>) -> Self {
        Vector(val)
    }
}

impl<T: Clone> From<&Vec<T>> for Vector<T> {
    fn from(vec: &Vec<T>) -> Self {
        Vector(Some(vec.to_vec()))
    }
}

impl<T> Vector<T> {
    pub fn new() -> Self {
        Vector::default()
    }

    pub fn init(&mut self, value: Option<Vec<T>>) {
        *self = Vector(value);
    }

    pub fn push(&mut self, push: T) {
        match self {
            Vector(None) => {
                *self = Vector(Some(vec![push]));
            }
            Vector(Some(ref mut v)) => (*v).push(push),
        }
    }

    pub fn append(&mut self, other: &mut Vec<T>) {
        match self {
            Vector(None) => {
                let mut inner = vec![];
                inner.append(other);
            }
            Vector(Some(ref mut vec)) => {
                (*vec).append(other);
            }
        }
    }

    pub fn is_none(&self) -> bool {
        match self {
            Vector(None) => true,
            _ => false,
        }
    }

    pub fn none(&mut self) {
        (*self) = Vector(None);
    }

    pub fn clear(&mut self) {
        match self {
            Vector(None) => {}
            Vector(Some(ref mut inner)) => (*inner).clear(),
        }
    }

    pub fn inner(&self) -> Option<&Vec<T>> {
        match &self {
            Vector(None) => None,
            Vector(Some(inner)) => Some(inner),
        }
    }

    pub fn inner_ref(self) -> Option<Vec<T>> {
        match self {
            Vector(None) => None,
            Vector(inner) => inner,
        }
    }
}

impl Vector<Flag> {
    pub fn find_long_flag(&self, alias_or_name: &str) -> (CalledType, Option<&Flag>) {
        match &self {
            Vector(None) => (CalledType::None, None),
            Vector(Some(flags)) => match flags.iter().find(|flag| flag.is(alias_or_name)) {
                None => match flags.iter().find(|flag| flag.any_long(alias_or_name)) {
                    None => match flags.iter().find(|flag| flag.any_short(alias_or_name)) {
                        None => (CalledType::None, None),
                        Some(f) => (CalledType::Short, Some(f)),
                    },
                    Some(f) => (CalledType::Long, Some(f)),
                },
                Some(f) => (CalledType::Name, Some(f)),
            },
        }
    }

    pub fn find_short_flag(&self, alias_or_name: &str) -> (CalledType, Option<&Flag>) {
        match &self {
            Vector(None) => (CalledType::None, None),
            Vector(Some(flags)) => match flags.iter().find(|flag| flag.any_short(alias_or_name)) {
                None => match flags.iter().find(|flag| flag.is(alias_or_name)) {
                    None => match flags.iter().find(|flag| flag.any_long(alias_or_name)) {
                        None => (CalledType::None, None),
                        Some(f) => (CalledType::Long, Some(f)),
                    },
                    Some(f) => (CalledType::Name, Some(f)),
                },
                Some(f) => (CalledType::Short, Some(f)),
            },
        }
    }
}
