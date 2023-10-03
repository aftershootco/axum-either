#[cfg(not(feature = "nomacro"))]
mod macroimpl;
#[cfg(feature = "nomacro")]
mod nomacroimpl;

#[derive(Clone)]
pub struct EitherHandler<Either, Or> {
    either: Either,
    or: Or,
}

pub trait IntoEitherHandler<Or> {
    fn or(self, or: Or) -> EitherHandler<Self, Or>
    where
        Self: Sized,
    {
        EitherHandler { either: self, or }
    }
}

impl<T, U> IntoEitherHandler<U> for T {}
