#![allow(dead_code)]

pub struct Node<T> {
    first_child: Option<std::boxed::Box<Node<T>>>,
    next_sibling: Option<std::boxed::Box<Node<T>>>,
    pub value: T,
}

impl<T> Node<T> {
    pub fn new(value: T, first_child: Option<std::boxed::Box<Node<T>>>, next_sibling: Option<std::boxed::Box<Node<T>>>) -> Node<T> {
        return Node {
            value: value,
            first_child: first_child,
            next_sibling: next_sibling,
        };
    }
}

pub struct Tree<T> {
    pub root: Option<std::boxed::Box<Node<T>>>,
}

impl<T> Tree<T> {
    pub fn new(root_value: T, first_child: Option<std::boxed::Box<Node<T>>>) -> Tree<T> {
        return Tree {
            root: Some(
                      std::boxed::Box::new(
                          Node::new(root_value, first_child, None)),
                  ),
        };
    }

    pub fn iter(&self) -> PreorderIter<T> {
        return PreorderIter::new(self.root.as_ref());
    }
}

pub struct PreorderIter<'a, T> {
    stack: Vec<&'a Node<T>>,
}

impl<'a, T> PreorderIter<'a, T> {
    pub fn new(root: Option<&'a std::boxed::Box<Node<T>>>) -> PreorderIter<'a, T> {
        match root {
            Some(n) => return PreorderIter {
                stack: vec![n],
            },
            None => return PreorderIter {
                stack: vec![]
            },
        };
    }
}

impl<'a, T> Iterator for PreorderIter<'a, T> {
    type Item = &'a Node<T>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.stack.pop() {
            Some(n) => {
                if let Some(s) = &n.next_sibling {
                    self.stack.push(s);
                }

                if let Some(c) = &n.first_child {
                    self.stack.push(c);
                }

                return Some(n);
            },
            None => return None,
        };
    }
}

#[cfg(test)]
mod tests {}
