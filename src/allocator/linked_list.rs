
pub struct Freelist {
    size: usize,
    next: Option<&'static mut Freelist>,
}
impl Freelist {
    pub const fn new() -> Self {
        Freelist {
            size: 0,
            next: None,
        }
    }
}