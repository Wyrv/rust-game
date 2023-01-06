extern crate rand; // Usando versao mais antiga
use rand::Rng;

use std::fs::File;
use std::io::prelude::*;
use std::process::Command;

use std::io;

const ALLOWED_ATTEMPTS: u8 = 5;

struct Letter{
    character: char,
    revealed: bool
}
 

//MAIN INI ===============================================================================
fn main() {
    let mut turns_left = ALLOWED_ATTEMPTS;
    let selected_word = select_word();
    let mut letters = create_letters(&selected_word);
    
    loop{
        let mut cmd = Command::new("");
        cmd.arg("clear");
        println!("Voce tem {} turnos restantes.",turns_left);
        display_progress(&letters);
        
        println!("Entre com o palpite da sua letra: ");
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
                at_least_one_revealed = true
            }
        }
        
        //Se ele nao tiver acertado nenhuma, perde uma vez
        if !at_least_one_revealed{
            turns_left -= 1;
        }

    }
    
    println!("Palavra escolhida foi: {}", selected_word);
}

//MAIN FIM ===============================================================================

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
