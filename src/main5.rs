macro_rules! generate_functions {
    ($($func_name:ident),*) => {
        $(
            fn $func_name() {
                println!("Hello from {}", stringify!($func_name));
            }
        )*
    };
}

generate_functions!(foo, bar, baz);

fn main() {
    foo();  
    bar();  
    baz();  
}
