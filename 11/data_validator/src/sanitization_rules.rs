pub trait SanitizerRule<T> {
    fn sanitize(&self, data: T) -> T;
}
