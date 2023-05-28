// A trait which implements the print marker: `{:?}`.
use core::fmt::Debug;
pub use stack_trait::Stack;
use std::fmt;
use std::fmt::Display;

/// `GenericStack<T>` is a linked-list based implementation of a stack:
/// It implements the trait [`Stack`], i.e., methods [`Stack::push`], [`Stack::pop`], [`Stack::peek`], and [`Stack::peek_mut`].
///
/// Traits:
///
/// [`GenericStack<T>`]implements the following traits:
///
///  - [`Debug`] since an implementation of [`Stack`] is required to implement trait [`Debug`]
///  - [`PartialEq`] since we want to support `assert_eq` in our code examples
///  - [`Clone`] since an implementation of [`Stack`] is required to implement trait [`Clone]`
///  - [`Display`] since an implementation of [`Stack`] is required to implement trait [`Display`]
///
/// It also implements iterators with the help of some helper types.
#[derive(Debug, PartialEq, Clone)]
pub struct GenericStack<T: Debug + PartialEq + Display + Clone> {
    head: Link<T>,
}

/// [`GenericStack<T>`] implements trait [`Display`]: It prints the all
/// entries of the stack separated by '->'. We use the fact that
/// [`GenericStack<T>`] implements an iterator and that values stored
/// in the stack must implement trait [`Display`]: we iterate over all
/// entries and write them to the provided formatter `f`.
///  
/// # Example:
///
/// `stack=head->6->4->3->2.`
impl<T: Debug + PartialEq + Display + Clone> fmt::Display for GenericStack<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "head")?;
        for v in self.iter() {
            write!(f, "->{v}")?;
        }
        write!(f, ".")?;
        Ok(())
    }
}

/// `GenericStack<T>` uses a linked list to implement the stack.
/// The next pointer is of type [`Link<T>`].
///
type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug, PartialEq, Clone)]
struct Node<T: Debug> {
    element: T,
    next: Link<T>,
}

impl<T: Debug + PartialEq + Clone + Display> Stack<T> for GenericStack<T> {
    /// Create a new monomorphic stack storing elements of type `<T>`.
    /// # Example
    ///
    /// ```
    /// // We need to import this trait to use the methods of this trait.
    /// // We can import an implementation like `ll_stack`
    /// use stack_trait::Stack;
    /// use ll_stack::GenericStack;
    /// // We create a stack of u128
    /// let mut stack : GenericStack<u128> = GenericStack::new();
    /// ```
    fn new() -> Self {
        GenericStack { head: None }
    }

    /// push a new element on the top element of the stack.
    ///
    /// # Arguments
    ///  - `element` to be pushed on the stack
    ///
    /// # Example
    ///
    /// ```
    /// // We need to import this trait to use the methods of this trait.
    /// // We can import an implementation like `ll_stack`
    /// use stack_trait::Stack;
    /// use ll_stack::GenericStack;
    /// // We create a stack of u64
    /// let mut stack = GenericStack::new();
    ///
    /// // we an push an element to the stack
    /// stack.push(1u64);
    /// assert_eq!(stack.peek(), Some(&1u64));
    /// ```
    fn push(&mut self, element: T) {
        let new_node = Box::new(Node {
            element,
            next: self.head.take(),
        });

        self.head = Some(new_node);
    }

    /// Returns the top element of the stack if it exists, i.e.,
    /// the last element that was pushed on the stack and not yet
    /// removed by a preceding call to `pop`
    ///
    /// # Arguments
    ///  - `pop` does not take any arguments.
    ///
    /// # Example
    ///
    /// ```
    /// // We need to import this trait to use the methods of this trait.
    /// // We can import an implementation like `ll_stack`
    /// use stack_trait::Stack;
    /// use ll_stack::GenericStack;
    /// // We create a stack of i32
    /// let mut stack = GenericStack::new();
    ///
    /// // Initially, the stack is empty:
    /// assert_eq!(stack.pop(), None);
    /// // we an push an element to the stack
    /// stack.push(1);
    /// assert_eq!(stack.pop(), Some(1));
    /// ```
    fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.element
        })
    }

    /// borrows the top element of the stack if the stack is not empty.
    /// This will return `None` if the stack is empty.
    ///
    /// # Example
    ///
    /// ```
    /// use stack_trait::Stack;
    /// use ll_stack::GenericStack;
    /// // We create a stack of u128
    /// let mut stack : GenericStack<u128> = GenericStack::new();
    ///     println!("Top element: {:?}", stack.peek());
    /// ```
    fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.element)
    }

    /// borrows the top element of the stack as a mutable value if the stack is not empty.
    /// This will return `None` if the stack is empty.
    ///
    /// # Example
    ///
    /// ```
    /// use stack_trait::Stack;
    /// use ll_stack::GenericStack;
    /// // We create a stack of u128
    /// let mut stack : GenericStack<u128> = GenericStack::new();
    ///   stack.peek_mut().map(|value| { *value += 1; } );
    /// ```
    fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.element)
    }
}

///
/// We define trait Iterators to define a three iterators for
/// [`GenericStack`]:
///
///  - `iter`:
///  - `iter_mut`:
///  - `into_iter`:
pub trait Iterators<T: Debug + PartialEq + Clone + Display>:
    Debug + Display + Clone + PartialEq
{
    fn into_iter(self) -> IntoIter<T>;

    /// iterator for `ll_stack<T>`
    fn iter(&self) -> Iter<'_, T>;

    /// mutable iterator for `ll_stack<T>`
    fn iter_mut(&mut self) -> IterMut<'_, T>;
}

impl<T: Debug + PartialEq + Clone + Display> Iterators<T> for GenericStack<T> {
    fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    /// the iterator starts with the head element and method next()
    /// will then follow the next pointers.
    fn iter(&self) -> Iter<'_, T> {
        Iter {
            next: self.head.as_deref(),
        }
    }

    fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut {
            next: self.head.as_deref_mut(),
        }
    }
}

pub struct IntoIter<T: Debug + PartialEq + Clone + Display>(GenericStack<T>);

impl<T: Debug + PartialEq + Clone + Display> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        // access fields of a tuple struct numerically
        self.0.pop()
    }
}

pub struct Iter<'a, T: Debug> {
    next: Option<&'a Node<T>>,
}

impl<'a, T: Debug + PartialEq + Clone + Display> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.element
        })
    }
}

pub struct IterMut<'a, T: Debug> {
    next: Option<&'a mut Node<T>>,
}

impl<'a, T: Debug + PartialEq> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.element
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basics() {
        let mut stack = GenericStack::new();

        // Check empty stack behaves right
        assert_eq!(stack.pop(), None);

        // Populate stack
        stack.push(1);
        stack.push(2);
        stack.push(3);

        // Check normal removal
        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        stack.push(4);
        stack.push(5);

        // Check normal removal
        assert_eq!(stack.pop(), Some(5));
        assert_eq!(stack.pop(), Some(4));

        // Check exhaustion
        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn peek() {
        let mut stack = GenericStack::new();
        assert_eq!(stack.peek(), None);
        assert_eq!(stack.peek_mut(), None);
        stack.push(1);
        stack.push(2);
        stack.push(3);

        assert_eq!(stack.peek(), Some(&3));
        assert_eq!(stack.peek_mut(), Some(&mut 3));

        if let Some(value) = stack.peek_mut() {
            *value = 42;
        }

        assert_eq!(stack.peek(), Some(&42));
        assert_eq!(stack.pop(), Some(42));
    }

    #[test]
    fn into_iter() {
        let mut stack = GenericStack::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);

        let mut iter = stack.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter() {
        let mut stack = GenericStack::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);

        let mut iter = stack.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
    }

    #[test]
    fn iter_mut() {
        let mut stack = GenericStack::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);

        let mut iter = stack.iter_mut();
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 1));
    }

    #[test]
    fn examples() {
        use core::fmt::Debug;
        use std::fmt;
        use std::fmt::Display;

        #[derive(Debug, PartialEq, Clone, Default)]
        pub struct Du64 {
            value: u64,
        }

        impl Drop for Du64 {
            fn drop(&mut self) {
                println!("Dropped: {}", self.value);
            }
        }

        impl Display for Du64 {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{}", self.value)
            }
        }

        let mut stack = GenericStack::new();

        // Check empty stack behaves right
        assert_eq!(stack.pop(), None);

        // Populate stack
        stack.push(Du64 { value: 1 });
        stack.push(Du64 { value: 2 });
        stack.push(Du64 { value: 3 });
        stack.push(Du64 { value: 4 });
        stack.push(Du64 { value: 5 });

        let stack2 = stack.clone();
        assert_eq!(stack, stack2);

        if let Some(node) = stack.peek_mut() {
            node.value += 1;
        }
        assert_ne!(stack, stack2);

        let stack3 = stack.clone();
        assert_eq!(stack, stack3);

        for v in &mut stack.iter_mut() {
            v.value += 1;
        }
        assert_ne!(stack, stack3);

        let stack4 = stack.clone();
        assert_eq!(stack, stack4);
        for mut v in stack.into_iter() {
            v.value += 1;
            println!("Incremented Value: {v:?}");
        }
        // assert_ne!(stack, stack4);

        let mut stack = GenericStack::new();
        if let Some(value) = stack.peek_mut() {
            *value += 1;
            assert_eq!(true, false);
        }

        // Populate stack
        stack.push(1);
        stack.push(2);
        stack.push(3);
        stack.push(4);
        stack.push(5);
        stack.iter().min();
        assert_eq!(stack.iter().min(), Some(&1));
        assert_eq!(stack.iter().max(), Some(&5));
        assert_eq!(stack.iter().sum::<i32>(), 15);

        let stack5 = stack.clone();
        assert_eq!(stack, stack5);

        let mut entries = 0;
        for _e in stack.iter() {
            entries += 1;
        }
        assert_eq!(entries, stack.iter().count());
        assert_eq!(stack, stack5);

        let stack6 = stack.clone();
        assert_eq!(stack, stack6);

        // we allow unused_must_use in the following block
        #[allow(unused_must_use)]
        {
            stack.iter_mut().map(|value| *value += 1);
            // NOTE: above statement does not change the stack!
            assert_eq!(stack, stack6);
        }

        // NOTE: We need some function that consumes the iterator like `last`:
        stack.iter_mut().map(|value| *value += 1).last();
        assert_ne!(stack, stack6);

        let mut entries = 0;
        stack.iter().map(|_| entries += 1).last();
        assert_eq!(entries, stack.iter().count());
    }
}
