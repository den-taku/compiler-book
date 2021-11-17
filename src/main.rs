fn main() {
    println!("Hello!");
    let a = 3 + 4 + 5 + 6 + 7;
    println!("{}", a);
}

#[cfg(test)]
mod tests {
    #[test]
    fn for_add_1_3() {
        assert_eq!(1 + 3, 4);
    }
}
