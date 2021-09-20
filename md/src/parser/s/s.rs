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

    pub fn to_vector(&self) -> Vec<&T> {
        let mut v = Vec::new();
        while let Some(h) = self.head() {
            v.push(h)
        }
        v
    }

    pub fn length(&self) -> usize {
        let mut i = 0;
        let mut tail = self;
        while let Some(_) = tail.head() {
            i += 1;
            tail = tail.tail();
        }
        i
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

    pub fn zip_with<'a, F, J>(&'a self, others: &'a S<T>, f: F) -> S<J>
    where
        F: Fn(&'a T, &'a T) -> J,
        J: Eq + fmt::Debug,
    {
        let car = self.head();
        let cadr = self.tail().head();

        let ocar = others.head();
        let ocadr = others.tail().head();

        match ((car, cadr), (ocar, ocadr)) {
            ((Some(car), Some(_)), (Some(ocar), Some(_))) => {
                S::cons(f(car, ocar), self.tail().zip_with(others.tail(), f))
            }
            ((Some(car), None), (Some(ocar), _)) | ((Some(car), _), (Some(ocar), None)) => {
                S::unit(f(car, ocar))
            }
            ((None, _), _) | (_, (None, _)) => S::Nil,
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

    /// assert_eq!(
    ///     S::from_vector(vec![1, 2, 3, 4, 5])
    ///                             .tail_after(2),
    ///     &S::from_vector(vec![3, 4, 5]))
    pub fn tail_after(&self, nth: usize) -> &S<T> {
        if nth > 0 {
            self.tail().tail_after(nth - 1)
        } else {
            self
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

#[test]
fn ts_length() {
    assert_eq!(S::from_vector(vec![1, 2, 3]).length(), 3,)
}

#[test]
fn ts_fold() {
    assert_eq!(S::from_vector(vec![1, 2, 3]).fold(0, |a, b| a + b), 6,)
}

#[test]
fn ts_zipwith() {
    assert_eq!(
        S::from_vector(vec![1, 2, 3]).zip_with(&S::from_vector(vec![4, 5, 6]), |a, b| a + b),
        S::from_vector(vec![5, 7, 9])
    )
}

#[test]
fn ts_tail_after() {
    assert_eq!(
        S::from_vector(vec![1, 2, 3, 4, 5]).tail_after(2),
        &S::from_vector(vec![3, 4, 5])
    )
}
