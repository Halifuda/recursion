extern crate recursion;

use recursion::CopyRecursion;

struct Fibonacci(CopyRecursion<(usize, usize)>);

impl Fibonacci {
    fn new() -> Self {
        Self(CopyRecursion::<(usize, usize)>::new(
            Some((0, 1)),
            // use `y` as the expected next value (F_{n+1}),
            // and `x` as the current value (F_n).
            |(x, y)| Some((*y, x + y)),
        ))
    }

    fn set(&mut self, data: (usize, usize)) -> Option<(usize, usize)> {
        self.0.set(data)
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
    println!("Set the initialize elems to (F_0={}, F_1={})", 1, 2);
    fib.set((1, 2));
    println!("The first {} altered fibonacci numbers are:", N);
    for (i, n) in fib.into_iter().enumerate() {
        print!("{} ", n);
        if i + 1 >= N {
            println!("");
            break;
        }
    }
}
