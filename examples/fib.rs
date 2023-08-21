extern crate recursion;

use recursion::CopyRecursion;

struct Fibonacci(CopyRecursion<(usize, usize)>);

impl Fibonacci {
    fn new() -> Self {
        Self(CopyRecursion::<(usize, usize)>::new(
            Some((0, 1)),
            |(x, y)| Some((*y, x + y)),
        ))
    }
}

impl Iterator for Fibonacci {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|(x, _y)| x)
    }
}

fn main() {
    let fib = &mut Fibonacci::new();
    const N: usize = 10;
    println!("The first {} fibonacci numbers are:", N);
    for (i, n) in fib.into_iter().enumerate() {
        print!("{} ", n);
        if i + 1 >= N {
            println!("");
            break;
        }
    }
    println!("Set the initialize elems to (a0={}, a1={})", 1, 2);
    fib.0.set((1, 2));
    println!("The first {} altered fibonacci numbers are:", N);
    for (i, n) in fib.into_iter().enumerate() {
        print!("{} ", n);
        if i + 1 >= N {
            println!("");
            break;
        }
    }
}
