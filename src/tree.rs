
pub trait Tree<T: ?Sized> {
    fn leaf(self) -> T;
    fn left(self) -> Box<dyn Tree<T>>;
    fn right(self) -> Box<dyn Tree<T>>;
}

pub struct Leaf<T>(pub T);

impl<T> Tree<T> for Leaf<T> {
    fn leaf(self) -> T { self.0 }

    fn left(self) -> Box<dyn Tree<T>> { panic!("left - Should not be called on Leaf!"); }

    fn right(self) -> Box<dyn Tree<T>> { panic!("right - Should not be called on Leaf!"); }
}

pub struct Fork<T>((Box<dyn Tree<T>>, Box<dyn Tree<T>>));

impl<T> Tree<T> for Fork<T> {
    fn leaf(self) -> T { panic!("leaf - Should not be called on Fork!") }

    fn left(self) -> Box<dyn Tree<T>> { self.0.0 }

    fn right(self) -> Box<dyn Tree<T>> { self.0.1 }
}
