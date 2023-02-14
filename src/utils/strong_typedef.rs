use std::marker::PhantomData;

pub struct StrongTypedef<T, Tag> {
    data: T,
    _phantom: PhantomData<Tag>,
}

impl<T, Tag> StrongTypedef<T, Tag> {
    pub fn new(obj: T) -> Self {
        StrongTypedef {
            data: obj,
            _phantom: PhantomData::default(),
        }
    }

    pub fn get(&self) -> &T {
        return &self.data;
    }
}