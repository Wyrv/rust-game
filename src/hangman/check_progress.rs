use crate::Letter;
use crate::GameProgress;

//Checa o progresso do jogo e informa no caso de progresso, derrota, ou vitoria
pub fn check_progress(turns_left: u8, letters: &Vec<Letter>) -> GameProgress {
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