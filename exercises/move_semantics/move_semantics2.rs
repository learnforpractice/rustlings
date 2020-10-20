// move_semantics2.rs
// Make me compile without changing line 13!
// Execute `rustlings hint move_semantics2` for hints :)

// I AM NOT DONE

// 1. Make another, separate version of the data that's in `vec0` and pass that
//    to `fill_vec` instead.
// 2. Make `fill_vec` borrow its argument instead of taking ownership of it,
//    and then copy the data within the function in order to return an owned
//    `Vec<i32>`
// 3. Make `fill_vec` *mutably* borrow its argument (which will need to be
//    mutable), modify it directly, then not return anything. Then you can get rid
//    of `vec1` entirely -- note that this will change what gets printed by the
//    first `println!`

fn test1() {
    let vec0 = Vec::new();

    let mut vec1 = fill_vec1(vec0.clone());

    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

fn fill_vec1(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}

fn test2() {
    let mut vec0 = Vec::new();

    let mut vec1 = fill_vec2(&mut vec0);

    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

fn fill_vec2(vec: &mut Vec<i32>) -> Vec<i32> {
    let mut vec = vec;

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec.clone()
}

fn test3() {
    let mut vec0 = Vec::new();

    fill_vec3(&mut vec0);

    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

    vec0.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec0.len(), vec0);
}

fn fill_vec3(vec: &mut Vec<i32>) {
    let mut vec = vec;

    vec.push(22);
    vec.push(44);
    vec.push(66);
}


fn test4() {
    let mut vec0 = Vec::new();

{
    fill_vec4(&mut vec0);
}

    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

    vec0.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec0.len(), vec0);
}

fn fill_vec4(vec: &mut Vec<i32>) -> &mut Vec<i32> {
    let mut vec = vec;

    vec.push(22);
    vec.push(44);
    vec.push(66);
    vec
}

fn main() {
    test1();
    test2();
    test3();
    test4();
}


