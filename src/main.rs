use std::io;

mod hangman;

const ALLOWED_ATTEMPTS: u8 = 5;

struct Letter{
    character: char,
    revealed: bool
}

enum GameProgress{
    InProgress,
    Won,
    Lost
}
 

//MAIN INI ===============================================================================
fn main() {
    
    let mut turns_left = ALLOWED_ATTEMPTS;
    let selected_word = hangman::select_word();
    let mut letters = create_letters(&selected_word);
    let mut used_letters: Vec<Letter> = Vec::new();
    let mut start = 0;

    print!("\x1B[2J\x1B[1;1H"); //LIMPA A TELA

    println!("\n\n\x1b[92m######################################################## \x1b[0m");
    println!("\x1b[92mSeja bem vindo ao jogo da forca. Tente acertar a palavra. \x1b[0m");
    println!("\x1b[92m######################################################## \x1b[0m");

    loop{
        if start > 0{
            print!("\x1B[2J\x1B[1;1H"); //LIMPA A TELA
        }
        
        if turns_left == 1 {
            println!("\x1b[91m\n\nVoce tem {} turnos restantes.\x1b[0m",turns_left);
        }
        else {
            println!("\x1b[94m\n\nVoce tem {} turnos restantes.\x1b[0m",turns_left);
        }
        
        //Mostra progresso EX: _ _ _ _ _
        display_progress(&letters,&used_letters);
        print_stage(ALLOWED_ATTEMPTS - turns_left);
        
        println!("\nEntre com o palpite da sua letra: ");

        //Le o INPUT do usuario
        let user_char = read_user_input_character(&mut used_letters);

        //Sair se o usuario digitar um *
        if user_char == '*'{
            break;
        }

        //Atualiza o status de cada caractere se o usuario acertar. 
        let mut at_least_one_revealed = false;
        for letter in letters.iter_mut(){
            if letter.character == user_char{
                letter.revealed = true;
                at_least_one_revealed = true;
                start += 1;
            }
        }
        
        //Se ele nao tiver acertado nenhuma, perde uma vez
        if !at_least_one_revealed{
            turns_left -= 1;
            start += 1;
        }

        match check_progress(turns_left, &letters){
            GameProgress::InProgress => continue,
            GameProgress::Won => {
                print!("\x1B[2J\x1B[1;1H"); //LIMPA A TELA
                println!("\n\n\x1b[96m####################################################### \x1b[0m");
                println!("\x1b[96mParabens, voce venceu! A palavra era: {}\x1b[0m", selected_word);
                println!("\x1b[96m####################################################### \x1b[0m");
                break;
            }
            GameProgress::Lost => {
                print!("\x1B[2J\x1B[1;1H"); //LIMPA A TELA
                println!("\n\n\x1b[91m####################################################### \x1b[0m");
                println!("\x1b[91mSinto muito, voce perdeu! A palavra era: {}\x1b[0m", selected_word);
                println!("\x1b[91m####################################################### \x1b[0m");
                print_stage(ALLOWED_ATTEMPTS - turns_left);
                break;
            }
        }
    }
    println!("\x1b[97mAte mais!\n\n\n\x1b[0m");
}

//MAIN FIM ===============================================================================

//===================================================================================
//Separa a palavras em vetor de caracteres
fn create_letters(word: &String) -> Vec<Letter>{
    //Cria vetor vazio
    let mut letters: Vec<Letter> = Vec::new();

    //Colocar cada caractere na struc
    for c in word.chars(){
        letters.push(Letter{
            character: c,
            revealed: false
        });
    }
return letters;
}



//===================================================================================
//Mostra o progresso do jogo
fn display_progress(letters: &Vec<Letter>, used_letters: &Vec<Letter>){
    let mut display_string = String::from("Progresso: ");
    let mut char_used = String::from("Letras utilizadas: ");

    //Mostrar charactere apropriado (letra ou '_') pra cada letra
    for letter in letters{
        display_string.push(' ');

        if letter.revealed {
            display_string.push(letter.character);
        }
        else {
            display_string.push('_');
        }
        display_string.push(' ');
    }

    for letter in used_letters{
        char_used.push(letter.character);
        char_used.push(' ');
    }

    println!("{}", display_string);
    println!("{}", char_used);


}

//===================================================================================
//Le o input do usuario
fn read_user_input_character(used_letters: &mut Vec<Letter>) -> char{

    let mut user_input = String::new();
    match io::stdin().read_line(&mut user_input){
        Ok(_) => {
            match user_input.chars().next(){
                Some(c) => {
                    used_letters.push(Letter{
                        character: c,
                        revealed: false
                    });
                    return c;
                }
                None => {
                    return '*';
                }
            }
        } 
        Err(_) => {return '*';}
    }


}

//===================================================================================
//Checa o progresso do jogo e informa no caso de progresso, derrota, ou vitoria
fn check_progress(turns_left: u8, letters: &Vec<Letter>) -> GameProgress {
    //Checa se todas as letras foram reveladas
    let mut all_revealed = true;
    for letter in letters{
        if !letter.revealed{
            all_revealed = false;
        }
    }
    if all_revealed {
        return GameProgress::Won;
    }

    if turns_left > 0{
        return GameProgress::InProgress;
    }
    return GameProgress::Lost;
}

//===================================================================================
//IMPRIME NA TELA O STICKMAN

fn print_stage(stage: u8) {

    match stage{
        0 => {
            println!("_______ ");
            println!("|     |");
            println!("|     ┻ ");
            println!("|         ");
            println!("|        ");
            println!("|         ");
            println!("|        ");
            println!("|         ");
            println!("| ");
            println!("----------- ");
        },
        1 => {
            println!("_______ ");
            println!("|     |");
            println!("|    _┻_");
            println!("|   (._.) ");
            println!("|        ");
            println!("|         ");
            println!("|        ");
            println!("|         ");
            println!("| ");
            println!("----------- ");
        },
        2 => {
            println!("_______ ");
            println!("|     |");
            println!("|    _┻_");
            println!("|   (._.) ");
            println!("|    /|  ");
            println!("|   / |   ");
            println!("|        ");
            println!("|         ");
            println!("| ");
            println!("----------- ");
        },
        3 => {
            println!("_______ ");
            println!("|     |");
            println!("|    _┻_");
            println!("|   (._.) ");
            println!("|    /|\\ ");
            println!("|   / | \\ ");
            println!("|        ");
            println!("|         ");
            println!("| ");
            println!("----------- ");
        },
        4 => {
            println!("_______ ");
            println!("|     |");
            println!("|    _┻_");
            println!("|   (._.) ");
            println!("|    /|\\ ");
            println!("|   / | \\ ");
            println!("|    /   ");
            println!("|   /     ");
            println!("| ");
            println!("----------- ");
        },
        5 => {
            println!("_______ ");
            println!("|     |");
            println!("|    _┻_");
            println!("|   (._.) ");
            println!("|    /|\\ ");
            println!("|   / | \\ ");
            println!("|    / \\ ");
            println!("|   /   \\ ");
            println!("| ");
            println!("----------- ");
        },
        _ => println!("\x1b[91mE morreu!\x1b[0m")
    }
}

