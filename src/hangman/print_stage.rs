//Imprime na tela o stickman

pub fn print_stage(stage: u8) {

    match stage{
        0 => {
            println!("╔═════╕ ");
            println!("║     |");
            println!("║     ┻ ");
            println!("║         ");
            println!("║        ");
            println!("║         ");
            println!("║        ");
            println!("║         ");
            println!("║ ");
            println!("----------- ");
        },
        1 => {
            println!("╔═════╕ ");
            println!("║     |");
            println!("║    _┻_");
            println!("║   (._.) ");
            println!("║        ");
            println!("║         ");
            println!("║        ");
            println!("║         ");
            println!("║ ");
            println!("----------- ");
        },
        2 => {
            println!("╔═════╕ ");
            println!("║     |");
            println!("║    _┻_");
            println!("║   (._.) ");
            println!("║    /|  ");
            println!("║   / |   ");
            println!("║        ");
            println!("║         ");
            println!("║ ");
            println!("----------- ");
        },
        3 => {
            println!("╔═════╕ ");
            println!("║     |");
            println!("║    _┻_");
            println!("║   (._.) ");
            println!("║    /|\\ ");
            println!("║   / | \\ ");
            println!("║        ");
            println!("║         ");
            println!("║ ");
            println!("----------- ");
        },
        4 => {
            println!("╔═════╕ ");
            println!("║     |");
            println!("║    _┻_");
            println!("║   (._.) ");
            println!("║    /|\\ ");
            println!("║   / | \\ ");
            println!("║    /   ");
            println!("║   /     ");
            println!("║ ");
            println!("----------- ");
        },
        5 => {
            println!("╔═════╕ ");
            println!("║     |");
            println!("║    _┻_");
            println!("║   (x_x) ");
            println!("║    /|\\ ");
            println!("║   / | \\ ");
            println!("║    / \\ ");
            println!("║   /   \\ ");
            println!("║ ");
            println!("----------- ");
        },
        _ => println!("\x1b[91mE morreu!\x1b[0m")
    }
}

