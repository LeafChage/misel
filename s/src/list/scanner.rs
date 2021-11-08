#[warn(unused_variables)]
use super::super::error::{Result, ScannerError};
use super::super::target::{HowToHandle, Target};
use super::S;
use std::fmt;

impl<T> S<T>
where
    T: Eq + fmt::Debug + Clone,
{
    pub fn to_end<'a>(&'a self) -> (S<T>, &Self) {
        if let Some(head) = self.head() {
            let (car, cdr) = self.tail().to_end();
            (S::cons(head.clone(), car), cdr)
        } else {
            (S::Nil, self.tail())
        }
    }

    pub fn next<'a, 'b, Tg>(&'a self, target: &'b Tg) -> Result<(Option<Tg::Output>, &'a Self)>
    where
        Tg: Target<T>,
    {
        let size = target.size();
        let buffer = self.head_before(size);
        if let Some(result) = target.search(&buffer) {
            match target.handle() {
                HowToHandle::Ignore => Ok((None, self.tail_after(size))),
                HowToHandle::Leave => Ok((None, self)),
                HowToHandle::Include => Ok((Some(result), self.tail_after(size))),
            }
        } else {
            Err(ScannerError::message(""))
        }
    }

    pub fn skip<'a, Tg>(&'a self, target: &Tg) -> Result<&'a Self>
    where
        Tg: Target<T>,
    {
        if let Some(_) = self.head() {
            let size = target.size();
            let buffer = self.head_before(size);
            if let Some(_) = target.search(&buffer) {
                match target.handle() {
                    HowToHandle::Ignore => Ok(self.tail()),
                    HowToHandle::Leave => Ok(self),
                    HowToHandle::Include => panic!("unexpected"),
                }
            } else {
                Ok(self.tail().skip(target)?)
            }
        } else {
            Err(ScannerError::end())
        }
    }

    pub fn until<'a, Tg>(&'a self, target: &Tg) -> Result<(S<T>, &'a Self)>
    where
        Tg: Target<T>,
    {
        if let Some(head) = self.head() {
            let size = target.size();
            let buffer = self.head_before(size);
            if let Some(_) = target.search(&buffer) {
                Ok(match target.handle() {
                    HowToHandle::Ignore => (S::Nil, self.tail_after(size)),
                    HowToHandle::Leave => (S::Nil, self),
                    HowToHandle::Include => (S::from(buffer), self.tail_after(size)),
                })
            } else {
                let (car, cdr) = self.tail().until(target)?;
                Ok((S::cons(head.clone(), car), cdr))
            }
        } else {
            Err(ScannerError::end())
        }
    }

    pub fn until_or_end<'a, Tg>(&'a self, target: &Tg) -> Result<(S<T>, &'a Self)>
    where
        Tg: Target<T>,
    {
        if let Some(head) = self.head() {
            let size = target.size();
            let buffer = self.head_before(size);
            if let Some(_) = target.search(&buffer) {
                Ok(match target.handle() {
                    HowToHandle::Ignore => (S::Nil, self.tail_after(size)),
                    HowToHandle::Leave => (S::Nil, self),
                    HowToHandle::Include => (S::from(buffer), self.tail_after(size)),
                })
            } else {
                let (car, cdr) = self.tail().until_or_end(target)?;
                Ok((S::cons(head.clone(), car), cdr))
            }
        } else {
            Ok((S::Nil, self))
        }
    }

    fn many_counted<'a, Tg>(&'a self, target: &Tg, count: usize) -> usize
    where
        Tg: Target<T>,
    {
        if let Ok((_, _)) = self.next(target) {
            self.tail_after(target.size())
                .many_counted(target, count + 1)
        } else {
            count
        }
    }

    pub fn many<'a, Tg>(&'a self, target: &Tg) -> Result<(usize, S<T>, &'a Self)>
    where
        Tg: Target<T>,
    {
        let count = self.many_counted(target, 0);
        Ok(match target.handle() {
            HowToHandle::Ignore => (count, S::Nil, self.tail_after(target.size() * count)),
            HowToHandle::Leave => (count, self.head_before(target.size() * count), self),
            HowToHandle::Include => (
                count,
                self.head_before(target.size() * count),
                self.tail_after(target.size() * count),
            ),
        })
    }

    pub fn many1<'a, Tg>(&'a self, target: &Tg) -> Result<(usize, S<T>, &'a Self)>
    where
        Tg: Target<T>,
    {
        let (count, result, tokens) = self.many(target)?;
        if count == 0 {
            Err(ScannerError::message(""))
        } else {
            Ok((count, result, tokens))
        }
    }

    pub fn head_before(&self, to: usize) -> S<T> {
        if to > 0 {
            if let Some(head) = self.head() {
                S::cons(head.clone(), self.tail().head_before(to - 1))
            } else {
                S::Nil
            }
        } else {
            S::Nil
        }
    }
}

#[cfg(test)]
mod test {
    use super::S;
    use crate::target::{And, Mono, Or, OrAnd};
    #[test]
    fn ts_to_end() {
        assert_eq!(S::from(vec![1, 2]).to_end(), (S::from(vec![1, 2]), &S::Nil));
    }

    #[test]
    fn ts_next_is() {
        let s = S::from(vec![1, 2]);
        assert_eq!(
            s.next(&Mono::new(1).ignore()),
            Ok((None, &S::from(vec![2])))
        );
        assert_eq!(
            s.next(&Mono::new(1).leave()),
            Ok((None, &S::from(vec![1, 2])))
        );
        assert_eq!(
            s.next(&Mono::new(1).include()),
            Ok((Some(1), &S::from(vec![2])))
        );
    }

    #[test]
    fn ts_next_are() {
        let s = S::from(vec![1, 1, 2]);
        assert_eq!(
            s.next(&And::from(vec![1, 1]).ignore()),
            Ok((None, &S::from(vec![2])))
        );
        assert_eq!(
            s.next(&And::from(vec![1, 1]).leave()),
            Ok((None, &S::from(vec![1, 1, 2])))
        );
        assert_eq!(
            s.next(&And::from(vec![1, 1]).include()),
            Ok((Some(S::from(vec![1, 1])), &S::from(vec![2])))
        );
    }

    #[test]
    fn ts_to_skip() {
        assert_eq!(
            S::from(vec![1, 2, 4,]).skip(&Mono::new(2).ignore()),
            Ok(&S::from(vec![4]))
        );
        assert_eq!(
            S::from(vec![1, 2, 4,]).skip(&Mono::new(2).leave()),
            Ok(&S::from(vec![2, 4]))
        );
    }

    #[test]
    fn ts_until() {
        assert_eq!(
            S::from(vec![1, 2, 4, 2,]).until(&Mono::new(2).ignore()),
            Ok((S::from(vec![1]), &S::from(vec![4, 2,])))
        );
        assert_eq!(
            S::from(vec![1, 2, 4, 2,]).until(&Mono::new(2).leave()),
            Ok((S::from(vec![1]), &S::from(vec![2, 4, 2,])))
        );
        assert_eq!(
            S::from(vec![1, 2, 4, 2,]).until(&Mono::new(2).include()),
            Ok((S::from(vec![1, 2]), &S::from(vec![4, 2])))
        );
        assert_eq!(
            S::from(vec![1, 2, 4, 2,]).until(&And::from(vec![4, 2]).ignore()),
            Ok((S::from(vec![1, 2]), &S::from(vec![])))
        );
        assert_eq!(
            S::from(vec![1, 2, 4, 2,]).until(&And::from(vec![4, 2]).leave()),
            Ok((S::from(vec![1, 2]), &S::from(vec![4, 2])))
        );
        assert_eq!(
            S::from(vec![1, 2, 4, 2,]).until(&And::from(vec![4, 2]).include()),
            Ok((S::from(vec![1, 2, 4, 2]), &S::from(vec![])))
        );
        assert_eq!(
            S::from(vec![1, 2, 4, 2,]).until(&Or::from(vec![4, 2]).include()),
            Ok((S::from(vec![1, 2]), &S::from(vec![4, 2])))
        );
        assert_eq!(
            S::from(vec![1, 2, 4, 2,])
                .until(&OrAnd::from(vec![And::from(vec![4, 3]), And::from(vec![4, 2])]).include()),
            Ok((S::from(vec![1, 2, 4, 2]), &S::from(vec![])))
        );
    }

    #[test]
    fn ts_until_or_end() {
        assert_eq!(
            S::from(vec![1, 2, 4, 2,]).until_or_end(&Mono::new(4).include()),
            Ok((S::from(vec![1, 2, 4]), &S::from(vec![2])))
        );

        assert_eq!(
            S::from(vec![1, 2, 4, 2,]).until_or_end(&Mono::new(5).include()),
            Ok((S::from(vec![1, 2, 4, 2]), &S::from(vec![])))
        );
    }

    #[test]
    fn ts_many() {
        assert_eq!(
            S::from(vec![3, 1, 3, 1, 3, 1, 5,]).many(&And::from(vec![3, 1]).leave()),
            Ok((
                3,
                S::from(vec![3, 1, 3, 1, 3, 1]),
                &S::from(vec![3, 1, 3, 1, 3, 1, 5,])
            ))
        );
        assert_eq!(
            S::from(vec![3, 1, 3, 1, 3, 1, 5,]).many(&And::from(vec![3, 1]).ignore()),
            Ok((3, S::Nil, &S::from(vec![5])))
        );
    }

    #[test]
    fn ts_next_are_or() {
        assert_eq!(
            S::from(vec![1, 1, 2])
                .next(&OrAnd::from(vec![And::from(vec![3, 3]), And::from(vec![1, 1])]).ignore()),
            Ok((None, &S::from(vec![2])))
        );

        assert_eq!(
            S::from(vec![1, 1, 2])
                .next(&OrAnd::from(vec![And::from(vec![1]), And::from(vec![1, 1])]).ignore()),
            Ok((None, &S::from(vec![1, 2])))
        );
    }

    #[test]
    fn ts_head_before() {
        assert_eq!(
            S::from(vec![1, 2, 3, 4, 5]).head_before(2),
            (S::from(vec![1, 2]))
        )
    }
}
