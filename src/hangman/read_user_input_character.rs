use crate::Letter;
use std::io;

//Le o input do usuario
pub fn read_user_input_character(used_letters: &mut Vec<Letter>) -> char{

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