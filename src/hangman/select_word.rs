extern crate rand; // Usando versao mais antiga
use rand::Rng;
use std::fs::File;
use std::io::prelude::*;

//Escolhe a palavra da lista de palavras
pub fn select_word() -> String{
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


