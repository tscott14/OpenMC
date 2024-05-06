
pub struct Variable<T> (T);

impl<T> Variable<T> {
    pub fn new(value: T) -> Self {
        Self(value)
    }

    pub fn value(&self) -> T {
        self.0
    }
}