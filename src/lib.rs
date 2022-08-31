pub trait IntoExt {
    fn into_<T>(self) -> T
    where
        Self: Into<T>,
    {
        Into::into(self)
    }
}

impl<T> IntoExt for T {}

pub trait TryIntoExt {
    fn try_into_<T>(self) -> Result<T, <Self as TryInto<T>>::Error>
    where
        Self: TryInto<T>,
    {
        TryInto::try_into(self)
    }
}

impl<T> TryIntoExt for T {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let _ = 1i8.into_::<i16>();
        let _ = 1i8.try_into_::<u8>();
    }
}
