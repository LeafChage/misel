use std::fmt;
use std::iter::Iterator;

#[derive(Eq, PartialEq)]
pub enum S<T>
where
    T: Eq + fmt::Debug,
{
    Cons(T, Box<S<T>>),
    Nil,
}

impl<T> S<T>
where
    T: Eq + fmt::Debug,
{
    pub fn unit(head: T) -> Self {
        S::Cons(head, Box::new(S::Nil))
    }

    pub fn cons(head: T, tail: S<T>) -> Self {
        S::Cons(head, Box::new(tail))
    }

    pub fn from_vector(v: Vec<T>) -> Self {
        v.into_iter().rev().fold(S::Nil, |l, head| S::cons(head, l))
    }

    pub fn fold<Acc, F>(&self, init: Acc, f: F) -> Acc
    where
        F: Fn(Acc, &T) -> Acc,
    {
        let car = self.head();
        let cdr = self.tail();
        let cadr = self.tail().head();

        match (car, cadr) {
            (Some(car), None) => f(init, car),
            (Some(car), Some(_)) => cdr.fold(f(init, car), f),
            (None, _) => init,
        }
    }

    pub fn head(&self) -> Option<&T> {
        match &*self {
            S::Cons(head, _) => Some(head),
            S::Nil => None,
        }
    }

    pub fn tail(&self) -> &S<T> {
        match self {
            &S::Cons(_, ref tail) => &(*tail),
            &S::Nil => &S::Nil,
        }
    }
}

impl<T> fmt::Debug for S<T>
where
    T: Eq + fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            S::Cons(a, b) => write!(f, "({:?} {:?})", a, b),
            S::Nil => write!(f, "nil"),
        }
    }
}

// ///////////////////////////////
//
// pub struct Chain<T>
// where
//     T: Eq + fmt::Debug,
// {
//     pub head: T,
//     pub tail: Option<Box<Chain<T>>>,
// }
//
// impl<T> Chain<T>
// where
//     T: Eq + fmt::Debug,
// {
//     pub fn new(v: T, v2: Option<Box<Chain<T>>>) -> Self {
//         Chain::<T> { head: v, tail: v2 }
//     }
//     pub fn some(v: T, child: Chain<T>) -> Self {
//         Chain::<T> {
//             head: v,
//             tail: Some(Box::new(child)),
//         }
//     }
//     pub fn none(v: T) -> Self {
//         Chain::<T> {
//             head: v,
//             tail: None,
//         }
//     }
// }
//
// impl<T> PartialEq for Chain<T>
// where
//     T: Eq + fmt::Debug,
// {
//     fn eq(&self, other: &Self) -> bool {
//         self.head == other.head && self.tail == other.tail
//     }
// }
//
// impl<T> fmt::Debug for Chain<T>
// where
//     T: Eq + fmt::Debug,
// {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         if let Some(tail) = &self.tail {
//             write!(f, "\n{:?}{:?}", &self.head, &tail)
//         } else {
//             write!(f, "\n{:?}", &self.head)
//         }
//     }
// }
