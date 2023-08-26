#[route(GET, "/")]
fn index() {
    // --snip--
}

#[proc_macro_attribute]
pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {
    // --snip--
}

mod run {
    #[macro_export]
    macro_rules! myvec {
        ($($x:expr),*) => {
            {
                let mut temp_vec = Vec::new();
                $(
                    temp_vec.push($x);
                )*
                temp_vec
            }
        };
    }

    pub fn run1() {
        let x = myvec!(1, 2, 3);
        for i in x {
            println!("{}", i);
        }
    }
    pub fn run2() {
        use hello_macro::HelloMacro;
        use hello_macro_derive::HelloMacro;
        #[derive(HelloMacro)]
        struct Pancakes;
        Pancakes::hello_macro();
    }
}

fn main() {
    run::run2();
}
