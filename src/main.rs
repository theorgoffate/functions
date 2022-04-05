fn main() {}

fn add(l: i64, r: i64) -> i64 {
    l + r
}
/*
   modules
       | main
           | tests
*/
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_nums() {
        let a = 2;
        let b = 2;
        let expected = a + b;
        let got = add(a, b);
        println!("a: {}, b: {}, expected: {}, got: {}", a, b, expected, got);
        assert_eq!(expected, got);
    }
}
