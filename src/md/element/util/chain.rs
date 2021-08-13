use std::fmt;

#[derive(Eq)]
pub struct Chain<T>
where
    T: Eq + fmt::Debug,
{
    pub head: T,
    pub tail: Option<Box<Chain<T>>>,
}

impl<T> Chain<T>
where
    T: Eq + fmt::Debug,
{
    pub fn new(v: T, v2: Option<Box<Chain<T>>>) -> Self {
        Chain::<T> { head: v, tail: v2 }
    }
    pub fn some(v: T, child: Chain<T>) -> Self {
        Chain::<T> {
            head: v,
            tail: Some(Box::new(child)),
        }
    }
    pub fn none(v: T) -> Self {
        Chain::<T> {
            head: v,
            tail: None,
        }
    }
}

impl<T> PartialEq for Chain<T>
where
    T: Eq + fmt::Debug,
{
    fn eq(&self, other: &Self) -> bool {
        self.head == other.head && self.tail == other.tail
    }
}

impl<T> fmt::Debug for Chain<T>
where
    T: Eq + fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(tail) = &self.tail {
            write!(f, "\n{:?}{:?}", &self.head, &tail)
        } else {
            write!(f, "\n{:?}", &self.head)
        }
    }
}
