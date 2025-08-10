enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}

impl<T> Default for List<T> {
    fn default() -> List<T> {
        List::Nil
    }
}

#[allow(dead_code)]
impl<T> List<T> {
    fn new() -> Self {
        List::Nil
    }

    fn retrieve(&self, index: usize) -> Option<&T> {
        match self {
            List::Nil => None,
            List::Cons(x, _) if index == 0 => Some(x),
            List::Cons(_, tail) => tail.retrieve(index - 1),
        }
    }

    fn append(&mut self, x: T) {
        match self {
            List::Nil => {
                *self = List::Cons(x, Box::new(List::Nil));
            },
            List::Cons(_, tail) => tail.append(x),
        }
    }

    fn remove(&mut self, index: usize) -> Option<T> {
        match self {
            List::Nil => None,
            List::Cons(_, _) if index == 0 => {
                if let List::Cons(x, tail) = std::mem::take(self) {
                    *self = *tail;
                    Some(x)
                } else { None }
            },
            List::Cons(_, tail) => tail.remove(index - 1),
        }
    }

    fn prepend(&mut self, x: T) {
        *self = List::Cons(x, Box::new(std::mem::take(self)));
    }

    fn size(&self) -> usize {
        match self {
            List::Nil => 0,
            List::Cons(_, tail) => 1 + tail.size(),
        }
    }
}

/* owned iteration (for x in xs) */

impl<T> Iterator for List<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match std::mem::take(self) {
            List::Cons(x, tail) => {
                *self = *tail;
                Some(x)
            },
            List::Nil => None,
        }
    }
}

/* borrowed iteration (for x in &xs) */

struct ListIter<'a, T> {
    current: Option<&'a List<T>>,
}

impl<'a, T> Iterator for ListIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.current {
            Some(List::Cons(x, tail)) => {
                self.current = Some(tail);
                Some(x)
            },
            _ => None,
        }
    }
}

impl<'a, T> IntoIterator for &'a List<T> {
    type Item = &'a T;
    type IntoIter = ListIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        ListIter { current: Some(self) }
    }
}
