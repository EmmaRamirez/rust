fn main() {
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    // `iter()` for vecs yields &i32. Destructure to `i32`
    println!("2 in vec1: {}", vec1.iter().any(|&x| x == 2));

    // `into_iter()` for vecs yeilds `i32`. No destructuring required.
    println!("2 in vec2: {}", vec2.into_iter().any(|x| x == 2));

    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];

    // `iter()` for arrays yields `&i32`
    println!("2 in array1: {}", array1.iter().any(|&x| x == 2));
    // `into_iter()` for arrays unsually yields `&i32`
    println!("2 in array2: {}", array2.into_iter().any(|&x| x == 2));

    // Iterator::find is a function which when passed an iterator, will
    // return the first element ath satisfies the predicate as an Option

    let mut iter = vec1.iter(); // &i32
    let mut into_iter = vec2.into_iter(); // i32

    // a reference to what is yielded is `&&i32`. Destructure to `i32`
    println!("Find 2 in vec1: {:?}", iter.find(|&&x| x == 2));
    // a reference to what is yeilded is `&i32`. Destructure to `i32`
    println!("Find 2 in vec2: {:?}", into_iter.find(|&x| x == 2));

    println!("Find 2 in array1: {:?}", array1.iter().find(|&&x| x == 2));

    println!(
        "Find 2 in array2: {:?}",
        array2.into_iter().find(|&&x| x == 2)
    );
}
