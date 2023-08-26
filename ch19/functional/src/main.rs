mod run {
    use std::vec;

    pub fn run1() {
        enum Status {
            Value(u32),
            Stop,
        }

        let list_of_statuses: Vec<Status> = (0..20u32).map(Status::Value).collect();
    }
    pub fn run2() {
        fn add_one(x: i32) -> i32 {
            x + 1
        }
        fn do_twice(f: fn(i32) -> i32, x: i32) -> i32 {
            f(x) + f(x)
        }
        let add_one_closure = |x: i32| x + 1;
        let answer = do_twice(add_one, 5);
        let answer_closure = do_twice(add_one_closure, 5);
        println!("{}\n{}", answer, answer_closure);
    }
    pub fn run3() {
        let list_of_numbers = vec![1, 2, 3, 4, 5];
        let list_of_strings: Vec<String> = list_of_numbers.iter().map(|i| i.to_string()).collect();
    }
    pub fn run4() {
        fn register1(cb: fn(i32) -> ()) {}
        let add_one_closure = |x: i32| {
            x + 1;
        };
        register1(add_one_closure);
    }
}

fn is_equal<T>(t1: &T, t2: &T) -> bool
where
    T: Eq + ?Sized,
{
    t1 == t2
}
fn main() {
    run::run2();
}
