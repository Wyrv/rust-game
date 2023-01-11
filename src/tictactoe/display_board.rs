pub fn display_board(stage: u8){
    match stage{
        0 => {
            println!("*   1 . 2 . 3  ");
            println!("A     |   |    ");
            println!("-  ---+---+--- ");
            println!("B     |   |    ");
            println!("-  ---+---+--- ");
            println!("C     |   |    ");
        },
        _ => {
            println!("Invalid Stage.");
        }
    }
}