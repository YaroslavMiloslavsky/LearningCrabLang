#[allow(unused)]
pub trait SanitizerRule<T: ?Sized> {
    fn sanitize(&self, data: &mut T) -> T;
}
