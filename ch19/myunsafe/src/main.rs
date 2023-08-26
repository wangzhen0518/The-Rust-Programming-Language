extern "C" {
    fn abs(input: i32) -> i32;
}

#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}

unsafe trait Foo {
    // methods go here
}

unsafe impl Foo for i32 {}

mod run {

    pub fn run1() {
        let mut num = 5;
        let r1 = &num as *const i32;
        let r2 = &mut num as *mut i32;
        unsafe {
            println!("r1 is: {:?}", r1);
            println!("r2 is: {:?}", r2);
        }
    }
    pub fn run2() {
        let mut v = vec![1, 2, 3, 4, 5];
        let r = &mut v[..];
        let (a, b) = r.split_at_mut(3);
    }
    pub fn run3() {
        use crate::abs;
        unsafe {
            println!("Absolute value of -3 according to C: {}", abs(-3));
        }
    }

    pub fn run4() {
        static HELLO_WORLD: &str = "Hello, world!";
        static mut COUNTER: u32 = 0;
        unsafe {
            COUNTER += 1;
            println!("COUNTER: {}", COUNTER);
        }
    }
}

fn main() {
    run::run4();
}
