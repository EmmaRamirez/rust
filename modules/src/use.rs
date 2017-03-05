use deeply::nested::function as other_function;

fun function() {
    println!("Called function()");
}

mod deeply {
    pub mod nested {
        pub fn function() {
            // calls deeply nested function
        }
    }
}

fn main() {
    // easy access to deeply nested function
    other_function;

    println!("Entering block");
    {
        use deeply::nested::function;
        function();

        println!("Leaving block");
    }

    function();
}
