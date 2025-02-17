fn main() {
    let n: u32 = 5;
    println!("⚙️ ~ fnmain ~ n: {}", n);
    println!("fib({n}): {}" , fib(n));

    println!("{n}! = {}", factorial(n));
    //printing a todo!()
    // fizzbuzz(n);
    
    println!("Lenght: {}", collatz_lenght(11)) ;
}

fn fib(n: u32) -> u32 {
    if n < 2{
        n
    } else { 
        fib(n-1) + fib(n-2)
    }
}

fn factorial(n: u32) -> u32 {
    let mut product = 1;
    for i in 1..=n { 
        product *= dbg!(i);
    }
    product
}

fn fizzbuzz (_n: u32) {
    todo!();
}


//Determine the length of the collatz sequence beginning at 'n'
fn collatz_lenght(mut n: i32) -> i32 {
    let mut iter = 1;
    while n > 1 {
        n = if n%2 == 0 { n / 2 } else { 3 * n + 1};
        iter += 1;
    }
    iter
}

#[test]
fn test_collatz_lenght() {
    assert_eq!(collatz_lenght(11), 15);
}

