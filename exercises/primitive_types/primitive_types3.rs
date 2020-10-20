// primitive_types3.rs
// Create an array with at least 100 elements in it where the ??? is.
// Execute `rustlings hint primitive_types3` for hints!

// I AM NOT DONE

fn test1() {
    let mut a: [Option<i32>; 10] = [None; 10];
    for i in 0..a.len() {
        let x: Option<i32> = {
            Some(i as i32 *2)
        };
        a[i as usize] = x;
    }
    println!("{:?}", a);
}

fn test2() {
    let a: [Option<i32>; 16] = [None; 16];
    let b: [Option<i32>; 10] = [None; 10];
    println!("{:?}", a);
}

fn main() {
    test1();
    test2();

    let a = vec![1, 2, 3, 4];

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
    }
}
