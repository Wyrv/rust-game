use crate::Letter;

//Separa a palavras em vetor de caracteres
pub fn create_letters(word: &String) -> Vec<Letter>{
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