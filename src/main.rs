use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("invalid number of arguments");
        std::process::exit(1);
    }

    print!(".intel_syntax noprefix\n");
    print!(".global _main\n\n");
    print!("_main:\n");
    print!("    mov rax, {}\n", args[1].parse::<i64>().unwrap());
    print!("    ret\n");
}

#[cfg(test)]
mod tests {
    #[test]
    fn for_add_1_3() {
        assert_eq!(1 + 3, 4);
    }
}
