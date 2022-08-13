// move_semantics3.rs
// Make me compile without adding new lines-- just changing existing lines!
// (no lines with multiple semicolons necessary!)
// Execute `rustlings hint move_semantics3` or use the `hint` watch subcommand for a hint.

fn main() {
    let mut vector = Vec::new();

    fill_vec(&mut vector);

    println!("{} has length {} content `{:?}`", "vector", vector.len(), vector);

    vector.push(88);

    println!("{} has length {} content `{:?}`", "vector", vector.len(), vector);
}

fn fill_vec(vec: &mut Vec<i32>) {
    vec.push(22);
    vec.push(44);
    vec.push(66);
}
