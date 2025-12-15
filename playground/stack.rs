
// Using a fixed size constant for the array, or const generics.
// To keep it simple and safe without unsafe code or Vec, we use Option<T> and Copy.
// NOTE: Generic arrays in Rust are tricky to initialize without Copy or unsafe.
// We require T: Copy to allow [None; N] initialization for simplicity.

#[derive(Debug)]
pub struct Stack<T: Copy, const N: usize> {
    data: [Option<T>; N],
    top: usize,
}

impl<T: Copy, const N: usize> Stack<T, N> {
    pub fn new() -> Self {
        Stack {
            data: [None; N],
            top: 0,
        }
    }

    pub fn push(&mut self, item: T) -> Result<(), &'static str> {
        if self.top >= N {
            return Err("Stack overflow");
        }
        self.data[self.top] = Some(item);
        self.top += 1;
        Ok(())
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.top == 0 {
            return None;
        }
        self.top -= 1;
        self.data[self.top].take()
    }

    pub fn peek(&self) -> Option<&T> {
        if self.top == 0 {
            return None;
        }
        self.data[self.top - 1].as_ref()
    }

    pub fn is_empty(&self) -> bool {
        self.top == 0
    }

    pub fn size(&self) -> usize {
        self.top
    }

    pub fn clear(&mut self) {
        for i in 0..self.top {
            self.data[i] = None;
        }
        self.top = 0;
    }
}

// Added main function to allow compilation as a binary
fn main() {
    let mut s: Stack<i32, 10> = Stack::new();
    
    println!("Testing Rust Array-Stack...");
    
    // Test isEmpty and size on empty stack
    assert!(s.is_empty());
    assert_eq!(s.size(), 0);

    // Test push
    s.push(10).unwrap();
    s.push(20).unwrap();
    s.push(30).unwrap();
    
    assert!(!s.is_empty());
    assert_eq!(s.size(), 3);
    assert_eq!(s.peek(), Some(&30));

    // Test pop
    assert_eq!(s.pop(), Some(30));
    assert_eq!(s.peek(), Some(&20));
    assert_eq!(s.size(), 2);

    // Test clear
    s.clear();
    assert!(s.is_empty());
    assert_eq!(s.size(), 0);

    // Test generic
    let mut sd: Stack<f64, 5> = Stack::new();
    sd.push(1.1).unwrap();
    assert_eq!(sd.peek(), Some(&1.1));

    println!("All Rust Array-Stack tests passed!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stack_basic() {
        let mut s: Stack<i32, 10> = Stack::new();
        
        assert!(s.is_empty());
        assert_eq!(s.size(), 0);

        s.push(1).unwrap();
        s.push(2).unwrap();
        s.push(3).unwrap();

        assert_eq!(s.size(), 3);
        assert_eq!(s.peek(), Some(&3));

        assert_eq!(s.pop(), Some(3));
        assert_eq!(s.pop(), Some(2));
        assert_eq!(s.pop(), Some(1));
        assert_eq!(s.pop(), None);
    }

    #[test]
    fn test_overflow() {
        let mut s: Stack<i32, 2> = Stack::new();
        s.push(10).unwrap();
        s.push(20).unwrap();
        assert!(s.push(30).is_err());
    }

    #[test]
    fn test_clear() {
        let mut s: Stack<i32, 5> = Stack::new();
        s.push(100).unwrap();
        s.clear();
        assert!(s.is_empty());
        assert_eq!(s.pop(), None);
    }
}
