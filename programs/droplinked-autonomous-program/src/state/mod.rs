mod shop;

pub(crate) use shop::*;

pub trait IsState<T> {
    const PREFIX: &'static str;

    fn prefix() -> &'static str {
        Self::PREFIX
    }

    fn space(params: T) -> usize;
}
