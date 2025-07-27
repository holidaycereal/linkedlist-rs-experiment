pub enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}

// so we don't have to write `std::mem::replace(my_list, List::Nil)`
// and instead can write `std::mem::take(my_list)`
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

    fn get_root(&self) -> Option<&T> {
        self.get(0)
    }

    fn get(&self, index: usize) -> Option<&T> {
        match self {
            List::Nil => None,
            List::Cons(x, _) if index == 0 => Some(x),
            List::Cons(_, tail) => tail.get(index - 1),
        }
    }

    fn len(&self) -> usize {
        match self {
            List::Nil => 0,
            List::Cons(_, tail) => 1 + tail.len(),
        }
    }

    fn prepend(&mut self, x: T) {
        *self = List::Cons(x, Box::new(std::mem::take(self)));
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
            List::Cons(_, _) if index == 0 => if let List::Cons(x, tail) = std::mem::take(self) {
                *self = *tail;
                Some(x)
            } else {
                None
            },
            List::Cons(_, tail) => tail.remove(index - 1),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construct_list() {
        let list: List<i32> = List::new();
        match list {
            List::Nil => assert!(true),
            _ => panic!("expected empty list"),
        }
    }

    #[test]
    fn test_root_is_none() {
        let list: List<i32> = List::new();
        let root_obj = list.get_root();
        assert!(root_obj.is_none());
    }

    #[test]
    fn test_append_to_root() {
        let mut list: List<i32> = List::new();
        list.append(1);
        let root_obj = list.get_root();
        assert!(root_obj.is_some());
    }

    #[test]
    fn test_append_and_get_0() {
        let mut list: List<i32> = List::new();
        list.append(1);
        let root_obj = list.get_root();
        assert!(root_obj.is_some());
        let obj_retrieved = list.get(0);
        assert_eq!(obj_retrieved, Some(&1));
    }

    #[test]
    fn test_multiple_append_and_get_1() {
        let mut list: List<i32> = List::new();
        list.append(10);
        list.append(20);
        let root_obj = list.get_root();
        assert!(root_obj.is_some());
        let obj_retrieved = list.get(1);
        assert_eq!(obj_retrieved, Some(&20));
    }

    #[test]
    fn test_multiple_append_and_get_3() {
        let mut list: List<i32> = List::new();
        list.append(10);
        list.append(20);
        list.append(30);
        list.append(40);
        list.append(50);
        let root_obj = list.get_root();
        assert!(root_obj.is_some());
        let obj_retrieved = list.get(3);
        assert_eq!(obj_retrieved, Some(&40));
    }

    #[test]
    fn test_append_once_and_remove_0() {
        let mut list: List<i32> = List::new();
        list.append(10);
        let obj = list.remove(0);
        assert_eq!(obj, Some(10));
    }

    #[test]
    fn test_multiple_append_and_remove_1() {
        let mut list: List<i32> = List::new();
        list.append(10);
        list.append(20);
        list.append(30);
        list.append(40);
        list.append(50);
        let obj = list.remove(1);
        assert_eq!(obj, Some(20));
    }

    #[test]
    fn test_multiple_append_and_remove_3() {
        let mut list: List<i32> = List::new();
        list.append(10);
        list.append(20);
        list.append(30);
        list.append(40);
        list.append(50);
        let obj = list.remove(3);
        assert_eq!(obj, Some(40));
    }

    #[test]
    fn test_length_is_0() {
        let list: List<i32> = List::new();
        assert_eq!(list.len(), 0);
    }
}
