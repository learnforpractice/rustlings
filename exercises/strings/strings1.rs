// strings1.rs
// Make me compile without changing the function signature!
// Execute `rustlings hint strings1` for hints ;)

// I AM NOT DONE

fn main() {
    {
        let answer = current_favorite_color();
        println!("My current favorite color is {}", answer);
    }
    
    {
        let answer = current_favorite_color2();
        println!("My current favorite color is {}", answer);
    }
    
}
    
fn current_favorite_color() -> &'static str {
    "blue"
}

fn current_favorite_color2() -> String {
    String::from("blue")
}
