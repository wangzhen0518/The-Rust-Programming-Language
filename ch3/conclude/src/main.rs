use std::vec;

fn fibo(n: usize) {
    let mut a = vec![1, 1];
    a.resize(n, 0);
    for i in 2..n {
        // println!("{}", i);
        a[i] = a[i - 1] + a[i - 2];
    }
    // for i in 0..a.len() {
    //     println!("{}", a[i]);
    // }
    println!("\n");
    for e in a {
        println!("{}", e);
    }
}
fn main() {
    fibo(10);
}
