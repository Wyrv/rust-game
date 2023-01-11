use crate::hangman::select_word::select_word;
use crate::hangman::print_stage::print_stage;
use crate::hangman::create_letters::create_letters;
use crate::hangman::display_progress::display_progress;
use crate::hangman::read_user_input_character::read_user_input_character;
use crate::hangman::check_progress::check_progress;
mod hangman;

const ALLOWED_ATTEMPTS: u8 = 5;

pub struct Letter{
    character: char,
    revealed: bool
}

pub enum GameProgress{
    InProgress,
    Won,
    Lost
}
 

fn main() {
    
    let mut turns_left = ALLOWED_ATTEMPTS;
    let selected_word = select_word();
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
        
        //Mostra figura e progresso EX: _ _ _ _ _
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