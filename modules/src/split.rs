mod my;

fun function() {
    // call function
}

fn main() {
    my::function();

    function();

    my::indirect_access();

    my::nested::function();
}
