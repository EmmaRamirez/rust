fn function() {

}

mod cool {
    pub fn function() {

    }
}

mod my {
    fn function() {

    }

    mod cool {
        pub fn function() {

        }
    }

    pub fn indirect_call() {
        // calls the current function in scope
        self::function();
        function();

        // calls the cool mod function inside 'my'
        self::cool::function();

        // calls the parent scope (outside the my module)
        super::function();

        // Binds the cool functioon in the *crate* scope
        // In this case the crate scope is the outermost scope.
        {
            use cool::function as root_function;
            root_function;
        }
    }
}


fn main() {
    my::indirect_call();
}
