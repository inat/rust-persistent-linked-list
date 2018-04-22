#![feature(box_syntax, box_patterns)]

#[derive(Debug)]
enum List<'a, T: 'a> {
    Cons(T, Box<&'a List<'a, T>>),
    Nil,
}

impl<'a, T> List<'a, T> {
    pub fn head(&self) -> &T {
        match self {
            List::Cons(head, _) => head,
            _ => panic!("err"),
        }
    }

    pub fn tail(&self) -> &List<'a, T> {
        match self {
            List::Cons(_, box tail) => tail,
            _ => panic!("err"),
        }
    }

    pub fn len(&self) -> i32 {
        match self {
            List::Cons(_, box tail) => 1 + tail.len(),
            _ => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use List;

    #[test]
    fn it_works() {
        let a: List<i64> = List::Cons(1, box &List::Nil);
        let b = List::Cons(2, box &a);
        let c = b.tail();
        assert_eq!(a.len(), 1);
        assert_eq!(b.len(), 2);
        assert_eq!(c.len(), 1);
        assert_eq!(a.head(), &1);
        assert_eq!(b.head(), &2);
        assert_eq!(c.head(), &1);
    }
}
