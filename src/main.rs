fn main() {
    let height: usize = 10;
    let width: usize = 10;
    print_board(height, width);
}



fn print_board(height: usize, width: usize){
    let hash = "# ";
    let top = hash.repeat(width);
    let space = "  ".repeat(width-2);
    
    println!("{}", top);

    let mut i = 0;
    while i < (height-2) {
        println!("{}{}{}", hash, space, hash);
        i += 1;
    }

    println!("{}", top);
}