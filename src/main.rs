fn main() {
    let msg = sprintln("Hello, world!");
    println!("{}", msg);
}

fn sprintln(msg: &str) -> String {
    format!("{}\n", msg)
}
