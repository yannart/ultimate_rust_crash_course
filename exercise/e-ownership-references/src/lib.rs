
pub fn inspect(arg : &str) {
    if arg.ends_with('s') {
        println!("{} is plural.", arg);
    } else {
        println!("{} is singular.", arg);
    }
}

pub fn change(arg : &mut String) {
    if !arg.ends_with('s') {
        arg.push_str("s")
    }
}

pub fn eat(arg : String) -> bool {
    arg.starts_with('b') && arg.contains('a')
}

pub fn add(arg1 : &i32, arg2 : &i32) -> i32{
    arg1 + arg2
}
