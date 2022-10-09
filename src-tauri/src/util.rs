pub trait ErrMapString {
    type Out;
    fn err_to_str(self) -> Self::Out;
}

impl<T, E: ToString> ErrMapString for Result<T, E> {
    type Out = Result<T, String>;

    fn err_to_str(self) -> Self::Out {
        self.map_err(|e| e.to_string())
    }
}
