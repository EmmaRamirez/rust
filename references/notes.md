# References

> Libraries cannot provide new inabilities

All the pointers we've seen so far are owning pointers. Rust also has nonowning pointer types called references which have no effect on their referent's lifetime.

In fact, it's the opposite: references must never outlive their referents. Rust refers to creating a reference to some value as 'borrowing' the value, what you have borrowed you must eventually return to its owner.

- A `shared reference` lets you read but not modify its referent
    - Can have as many refs to a particular value as you like
    - &e yields a shared reference to e's value
    - If e has type type T, then &e has type &T, "ref T"
    - Shared references are `Copy`
- A `mutable reference` to a value, may both read and modify value
    - However, you may not have any other references active
    - The expression `&mut e` yeilds a mutable reference to e's value
    - &mut T, "ref mute T"
    - Mutable references are **not** `Copy`

