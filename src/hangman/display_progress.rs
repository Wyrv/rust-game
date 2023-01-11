use crate::Letter;

//Mostra o progresso do jogo
pub fn display_progress(letters: &Vec<Letter>, used_letters: &Vec<Letter>){
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