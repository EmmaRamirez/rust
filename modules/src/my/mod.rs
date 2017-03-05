mod inaccessible;
pub mod nested;

pub fn function() {
    println!("called my::function()");
}

fn private_function() {
    // can't be called outside
}

pub fn indirect_access() {
    // call private function publicly
    private_function();
}
