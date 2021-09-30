#[warn(unused_variables)]
use super::error::{Result, ScannerError};
use super::S;
use std::fmt;
use std::iter::Iterator;

#[derive(Clone, Debug)]
enum HowToHandle {
    /// abcd until b return (a, cd)
    Ignore,

    /// abcd until b return (a, bcd)
    Leave,

    /// abcd until b return (ab, cd)
    Include,
}

impl<T> S<T>
where
    T: Eq + fmt::Debug,
{
    pub fn next_is_ignore<'a>(&'a self, target: &T) -> Result<(&'a T, &Self)> {
        self.next_is(target, HowToHandle::Ignore)
    }

    pub fn next_is_leave<'a>(&'a self, target: &T) -> Result<(&'a T, &Self)> {
        self.next_is(target, HowToHandle::Leave)
    }

    fn next_is<'a>(&'a self, target: &T, how_to_handle: HowToHandle) -> Result<(&'a T, &Self)> {
        if let Some(head) = self.head() {
            if &head == &target {
                match how_to_handle {
                    HowToHandle::Ignore => Ok((head, self.tail())),
                    HowToHandle::Leave => Ok((head, self)),
                    HowToHandle::Include => panic!("unexpected"),
                }
            } else {
                Err(ScannerError::unexpected(target, head))
            }
        } else {
            Err(ScannerError::end())
        }
    }

    pub fn next_are_ignore<'a>(&'a self, targets: &S<T>) -> Result<(S<&'a T>, &'a S<T>)> {
        self.next_are(targets, HowToHandle::Ignore)
    }

    pub fn next_are_leave<'a>(&'a self, targets: &S<T>) -> Result<(S<&'a T>, &'a S<T>)> {
        self.next_are(targets, HowToHandle::Leave)
    }

    fn next_are<'a>(
        &'a self,
        targets: &S<T>,
        how_to_handle: HowToHandle,
    ) -> Result<(S<&'a T>, &'a S<T>)> {
        let ok = self
            .zip_with(targets, |t, target| t == target)
            .fold(true, |a, b| a && *b);

        if ok {
            match how_to_handle {
                HowToHandle::Ignore => {
                    let length = targets.length();
                    Ok(self.slice(length))
                }
                HowToHandle::Leave => Ok((self.head_before(targets.length()), self)),
                HowToHandle::Include => panic!("unexpected"),
            }
        } else {
            Err(ScannerError::not_found(targets))
        }
    }

    pub fn to_skip_ignore(&self, target: &T) -> Result<&Self> {
        self.to_skip(target, HowToHandle::Ignore)
    }

    pub fn to_skip_leave(&self, target: &T) -> Result<&Self> {
        self.to_skip(target, HowToHandle::Leave)
    }

    fn to_skip(&self, target: &T, how_to_handle: HowToHandle) -> Result<&Self> {
        if let Some(head) = self.head() {
            Ok(if head == target {
                match how_to_handle {
                    HowToHandle::Ignore => self.tail(),
                    HowToHandle::Leave => self,
                    HowToHandle::Include => panic!("unexpected"),
                }
            } else {
                self.tail().to_skip(target, how_to_handle)?
            })
        } else {
            Err(ScannerError::end())
        }
    }

    pub fn until_ignore<'a>(&'a self, target: &T) -> Result<(S<&'a T>, &'a Self)> {
        self.until(target, HowToHandle::Ignore)
    }

    pub fn until_leave<'a>(&'a self, target: &T) -> Result<(S<&'a T>, &'a Self)> {
        self.until(target, HowToHandle::Leave)
    }

    pub fn until_include<'a>(&'a self, target: &T) -> Result<(S<&'a T>, &'a Self)> {
        self.until(target, HowToHandle::Include)
    }

    fn until<'a>(&'a self, target: &T, how_to_handle: HowToHandle) -> Result<(S<&'a T>, &'a Self)> {
        if let Some(head) = self.head() {
            Ok(if head == target {
                match how_to_handle {
                    HowToHandle::Ignore => (S::Nil, self.tail()),
                    HowToHandle::Leave => (S::Nil, self),
                    HowToHandle::Include => (S::unit(head), self.tail()),
                }
            } else {
                let (car, cdr) = self.tail().until(target, how_to_handle)?;
                (S::cons(head, car), cdr)
            })
        } else {
            Err(ScannerError::end())
        }
    }

    pub fn until_targets_ignore<'a>(&'a self, targets: &S<T>) -> Result<(S<&'a T>, &'a Self)> {
        self.until_targets(targets, HowToHandle::Ignore)
    }
    pub fn until_targets_leave<'a>(&'a self, targets: &S<T>) -> Result<(S<&'a T>, &'a Self)> {
        self.until_targets(targets, HowToHandle::Leave)
    }

    pub fn until_targets_include<'a>(&'a self, targets: &S<T>) -> Result<(S<&'a T>, &'a Self)> {
        self.until_targets(targets, HowToHandle::Include)
    }

    fn until_targets<'a>(
        &'a self,
        targets: &S<T>,
        how_to_handle: HowToHandle,
    ) -> Result<(S<&'a T>, &'a Self)> {
        let ok = self
            .zip_with(&targets, |t, target| t == target)
            .fold(true, |a, b| a && *b);

        if let Some(head) = self.head() {
            if ok {
                match how_to_handle {
                    HowToHandle::Ignore => {
                        let length = targets.length();
                        Ok((S::Nil, self.tail_after(length)))
                    }
                    HowToHandle::Leave => Ok((S::Nil, self)),
                    HowToHandle::Include => Ok(self.slice(targets.length())),
                }
            } else {
                let (car, cdr) = self.tail().until_targets(targets, how_to_handle)?;
                Ok((S::cons(head, car), cdr))
            }
        } else {
            Err(ScannerError::end())
        }
    }

    pub fn to_somewhere_ignore<'a>(&'a self, targets: &Vec<T>) -> Result<(S<&'a T>, &'a Self)> {
        self.to_somewhere(targets, HowToHandle::Ignore)
    }

    pub fn to_somewhere_leave<'a>(&'a self, targets: &Vec<T>) -> Result<(S<&'a T>, &'a Self)> {
        self.to_somewhere(targets, HowToHandle::Leave)
    }

    pub fn to_somewhere_include<'a>(&'a self, targets: &Vec<T>) -> Result<(S<&'a T>, &'a Self)> {
        self.to_somewhere(targets, HowToHandle::Include)
    }

    fn to_somewhere<'a>(
        &'a self,
        targets: &Vec<T>,
        how_to_handle: HowToHandle,
    ) -> Result<(S<&'a T>, &'a Self)> {
        if let Some(head) = self.head() {
            let include = targets.iter().fold(None, |result, current| match result {
                Some(_) => result,
                None => {
                    if current == head {
                        Some(current)
                    } else {
                        None
                    }
                }
            });

            Ok(match include {
                Some(_target) => match how_to_handle {
                    HowToHandle::Ignore => (S::Nil, self.tail()),
                    HowToHandle::Leave => (S::Nil, self),
                    HowToHandle::Include => (S::unit(head), self.tail()),
                },
                None => {
                    let (car, cdr) = self.tail().to_somewhere(targets, how_to_handle)?;
                    (S::cons(head, car), cdr)
                }
            })
        } else {
            Err(ScannerError::end())
        }
    }

    pub fn to_end(&self) -> S<&T> {
        if let Some(head) = self.head() {
            S::cons(head, self.tail().to_end())
        } else {
            S::Nil
        }
    }

    pub fn many_ignore<'a>(&'a self, targets: &S<T>) -> Result<(usize, &'a Self)> {
        self.many(targets, HowToHandle::Ignore)
    }
    pub fn many_leave<'a>(&'a self, targets: &S<T>) -> Result<(usize, &'a Self)> {
        self.many(targets, HowToHandle::Leave)
    }

    fn many<'a>(&'a self, targets: &S<T>, how_to_handle: HowToHandle) -> Result<(usize, &'a Self)> {
        let mut tail = self;
        let mut count = 0;
        while tail
            .zip_with(&targets, |t, target| t == target)
            .fold(true, |a, b| a && *b)
        {
            count += 1;
            let length = targets.length();
            tail = tail.tail_after(length);
        }

        Ok(match how_to_handle {
            HowToHandle::Ignore => (count, tail),
            HowToHandle::Leave => (count, self),
            HowToHandle::Include => unimplemented!(),
        })
    }

    pub fn many1_ignore<'a>(&'a self, targets: &S<T>) -> Result<(usize, &'a Self)> {
        self.many1(targets, HowToHandle::Ignore)
    }
    pub fn many1_leave<'a>(&'a self, targets: &S<T>) -> Result<(usize, &'a Self)> {
        self.many1(targets, HowToHandle::Leave)
    }

    fn many1<'a>(
        &'a self,
        targets: &S<T>,
        how_to_handle: HowToHandle,
    ) -> Result<(usize, &'a Self)> {
        let (count, tokens) = self.many(targets, how_to_handle)?;
        if count == 0 {
            Err(ScannerError::not_found(targets))
        } else {
            Ok((count, tokens))
        }
    }

    pub fn next_is_or_ignore<'a>(&'a self, someone: &Vec<T>) -> Result<(&'a T, &'a Self)> {
        self.next_is_or(someone, HowToHandle::Ignore)
    }

    pub fn next_is_or_leave<'a>(&'a self, someone: &Vec<T>) -> Result<(&'a T, &'a Self)> {
        self.next_is_or(someone, HowToHandle::Leave)
    }

    fn next_is_or<'a>(
        &'a self,
        someone: &Vec<T>,
        how_to_handle: HowToHandle,
    ) -> Result<(&'a T, &'a Self)> {
        for t in someone.iter() {
            let result = self.next_is(t, how_to_handle.clone());
            if let Ok(_) = result {
                return result;
            }
        }
        Err(ScannerError::message(""))
    }

    pub fn next_are_or_ignore<'a>(&'a self, someone: &Vec<S<T>>) -> Result<(S<&'a T>, &'a Self)> {
        self.next_are_or(someone, HowToHandle::Ignore)
    }

    pub fn next_are_or_leave<'a>(&'a self, someone: &Vec<S<T>>) -> Result<(S<&'a T>, &'a Self)> {
        self.next_are_or(someone, HowToHandle::Leave)
    }

    fn next_are_or<'a>(
        &'a self,
        someone: &Vec<S<T>>,
        how_to_handle: HowToHandle,
    ) -> Result<(S<&'a T>, &'a Self)> {
        for t in someone.iter() {
            let result = self.next_are(t, how_to_handle.clone());
            if let Ok(_) = result {
                return result;
            }
        }

        Err(ScannerError::message(""))
    }
}

#[test]
fn ts_next_is() {
    assert_eq!(
        S::from_vector(vec![1, 2]).next_is_ignore(&1),
        Ok((&1, &S::from_vector(vec![2])))
    );
    assert_eq!(
        S::from_vector(vec![1, 2]).next_is_leave(&1),
        Ok((&1, &S::from_vector(vec![1, 2])))
    );
}

#[test]
fn ts_next_are() {
    assert_eq!(
        S::from_vector(vec![1, 1, 2]).next_are_ignore(&S::from_vector(vec![1, 1])),
        Ok((S::from_vector(vec![&1, &1]), &S::from_vector(vec![2])))
    );
}

#[test]
fn ts_to_skip() {
    assert_eq!(
        S::from_vector(vec![1, 2, 4,]).to_skip_ignore(&2),
        Ok(&S::from_vector(vec![4]))
    );
    assert_eq!(
        S::from_vector(vec![1, 2, 4,]).to_skip_leave(&2),
        Ok(&S::from_vector(vec![2, 4]))
    );
}

#[test]
fn ts_until() {
    assert_eq!(
        S::from_vector(vec![1, 2, 4, 2,]).until_ignore(&2),
        Ok((S::from_vector(vec![&1]), &S::from_vector(vec![4, 2,])))
    );
    assert_eq!(
        S::from_vector(vec![1, 2, 4, 2,]).until_leave(&2),
        Ok((S::from_vector(vec![&1]), &S::from_vector(vec![2, 4, 2,])))
    );
    assert_eq!(
        S::from_vector(vec![1, 2, 4, 2,]).until_include(&2),
        Ok((S::from_vector(vec![&1, &2]), &S::from_vector(vec![4, 2])))
    );
}

#[test]
fn ts_until_targets() {
    assert_eq!(
        S::from_vector(vec![1, 2, 4, 2,]).until_targets_ignore(&S::from_vector(vec![4, 2])),
        Ok((S::from_vector(vec![&1, &2]), &S::from_vector(vec![])))
    );
    assert_eq!(
        S::from_vector(vec![1, 2, 4, 2,]).until_targets_leave(&S::from_vector(vec![4, 2])),
        Ok((S::from_vector(vec![&1, &2]), &S::from_vector(vec![4, 2])))
    );
    assert_eq!(
        S::from_vector(vec![1, 2, 4, 2,]).until_targets_include(&S::from_vector(vec![4, 2])),
        Ok((
            S::from_vector(vec![&1, &2, &4, &2]),
            &S::from_vector(vec![])
        ))
    );
}

#[test]
fn ts_many() {
    assert_eq!(
        S::from_vector(vec![3, 1, 3, 1, 3, 1, 5,]).many_ignore(&S::from_vector(vec![3, 1])),
        Ok((3, &S::from_vector(vec![5])))
    );
    assert_eq!(
        S::from_vector(vec![3, 1, 3, 1, 3, 1, 5,]).many_leave(&S::from_vector(vec![3, 1])),
        Ok((3, &S::from_vector(vec![3, 1, 3, 1, 3, 1, 5,])))
    );
}

#[test]
fn ts_next_are_or() {
    assert_eq!(
        S::from_vector(vec![1, 1, 2]).next_are_or_ignore(&vec![
            S::from_vector(vec![3, 3]),
            S::from_vector(vec![1, 1])
        ]),
        Ok((S::from_vector(vec![&1, &1]), &S::from_vector(vec![2])))
    );

    assert_eq!(
        S::from_vector(vec![1, 1, 2])
            .next_are_or_ignore(&vec![S::from_vector(vec![1]), S::from_vector(vec![1, 1])]),
        Ok((S::from_vector(vec![&1]), &S::from_vector(vec![1, 2])))
    );
}
