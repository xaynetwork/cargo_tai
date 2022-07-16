use anymap::AnyMap;

use crate::common::opts::Options;

#[derive(Default)]
pub struct Context(AnyMap);

impl Context {
    pub fn get<T: 'static>(&self) -> &T {
        self.0
            .get()
            .expect("that should not have happened. This is a bug!")
    }

    pub fn get_mut<T: 'static>(&mut self) -> &mut T {
        self.0
            .get_mut()
            .expect("that should not have happened. This is a bug!")
    }

    pub fn insert<T: 'static>(&mut self, value: T) -> Option<T> {
        self.0.insert(value)
    }

    pub fn remove<T: 'static>(&mut self) -> T {
        self.0
            .remove()
            .expect("that should not have happened. This is a bug!")
    }
}

impl From<Options> for Context {
    fn from(opts: Options) -> Self {
        let mut map = AnyMap::new();
        map.insert(opts);
        Self(map)
    }
}
