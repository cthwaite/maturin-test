fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    #[test]
    fn two_plus_two_is_four() {
        assert_eq!(2 + 2, 4);
    }
}
