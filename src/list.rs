
#[derive(Debug, Clone)]
pub enum List<T> {
    Nil,
    Cons(T, Box<List<T>>),
}

impl<T> List<T> {
    pub fn hd(&self) -> Option<&T> {
        match self {
            List::Nil => None,
            List::Cons(head, _) => Some(head)
        }
    }

    pub fn empty<TT>() -> Box<List<TT>> {
        Box::new(List::<TT>::Nil)
    }

    pub fn unfold<S, F: Fn(S) -> Option<(S, T)>> (init: S, f: F) -> Box<List<T>> {
        fn iter<S, T, F: Fn(S) -> Option<(S, T)>> (state: S, f: &F) -> Box<List<T>> {
            match f(state) {
                None => Box::new(List::Nil),
                Some((s, y)) => Box::new(List::Cons(y, iter(s, f)))
            }
        }
        iter(init, &f)
    }

    pub fn from_vec(v : Vec<T>)-> Box<List<T>> where T: Clone {
        let len = v.len();
        Self::unfold(0, move |i| {
            if i < len {
                v.get(i).map(|x| (i + 1, x.clone()))
            } else {
                None
            }
         })
    }

    pub fn from_array(array: &[T])-> Box<List<T>> where T: Clone {
        Self::from_vec(array.iter().cloned().collect())
    }

    pub fn map<U, F: Fn(&T) -> U>(&self, f: F) -> Box<List<U>> {
        match self {
            List::Nil => Box::new(List::Nil),
            List::Cons(x, xs) => {
                let new_head = f(x);
                let new_tail = xs.map(f);
                Box::new(List::Cons(new_head, new_tail))
            }
        }
    }
}





