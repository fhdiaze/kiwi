pub struct Page<T> {
    items: Vec<T>,
    page: usize,
    size: usize,
    total: usize,
}

impl<T> Page<T> {
    pub fn new(items: Vec<T>, page: usize, total: usize) -> Self {
        Page { items, page, total }
    }
}
