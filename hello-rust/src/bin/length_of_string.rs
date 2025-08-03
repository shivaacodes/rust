fn main() {
    let name = String::from("shivasajay");
    let len = get_len_string(name);
    println!("The length of the string is: {}", len);
}

fn get_len_string(str: String) -> usize {
    return str.chars().count();
}
