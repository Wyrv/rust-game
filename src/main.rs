extern crate rand; // Usando versao mais antiga
use rand::Rng;

use std::fs::File;
use std::io::prelude::*;

use std::io;

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
    let selected_word = select_word();
    let mut letters = create_letters(&selected_word);
    let mut start = 0;

    print!("\x1B[2J\x1B[1;1H"); //LIMPA A TELA

    println!("\n\n\x1b[92m######################################################## \x1b[0m");
    println!("\x1b[92mSeja bem vindo ao jogo da focar. Tente acertar a palavra. \x1b[0m");
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
        display_progress(&letters);
        
        println!("\nEntre com o palpite da sua letra: ");

        //Le o INPUT do usuario
        let user_char = read_user_input_character();

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
                break;
            }
        }
    }
    println!("\x1b[97mAte mais!\n\n\n\x1b[0m");
}

//MAIN FIM ===============================================================================

//Escolhe a palavra da lista de palavras
fn select_word() -> String{
    //Abrir arquivo
    let mut file = File::open("words.txt").expect("Falha ao abrir o arquivo!");

    //Carregar conteudo do arquivo
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents).expect("Erro ao ler o arquivo.");

    //Separar palavras individualmente em vetor
    let avaliable_words: Vec<&str> = file_contents.trim().split(',').collect();

    //Gerar index aleatorio
    let random_index = rand::thread_rng().gen_range(0, avaliable_words.len());

    return String::from(avaliable_words[random_index]);
}

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

//Mostra o progresso do jogo
fn display_progress(letters: &Vec<Letter>){
    let mut display_string = String::from("Progress: ");

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
    println!("{}", display_string);
}

//Le o input do usuario
fn read_user_input_character() -> char{
    let mut user_input = String::new();
    match io::stdin().read_line(&mut user_input){
        Ok(_) => {
            match user_input.chars().next(){
                Some(c) => {return c;}
                None => {return '*';}
            }
        } 
        Err(_) => {return '*';}
    }
}

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
