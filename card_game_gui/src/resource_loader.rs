use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use crate::xml_parser::{CardDataFiles, FileToInclude};
use crate::misc::loadFile;
use hard_xml::XmlRead;


#[derive(Debug)]
pub struct MtgCard {
    name: String,
    set: String ,
    image: String ,
    color: String ,
    color_id: String ,
    cost: String ,
    mana_value: String ,
    card_type: String ,
    power: String ,
    thoughness: String ,
    loyalty: String ,
    rarity: String ,
    draft_qualities: String ,
    sound: String ,
    script: String ,
    text: String ,
}

pub fn loadMtgCards() {
    // if let Ok(dataFiles) = loadFile("ListOfCardDataFiles.txt") {

    // }
    // else {
    //     println!(""); 
    // }
    let data_files_text = loadFile("ListOfCardDataFiles.txt").unwrap();
    let data_files = CardDataFiles::from_str(&data_files_text).unwrap(); 
    let mut cards: Vec<MtgCard> = Vec::<MtgCard>::new();
    
    for dataFile in data_files.files.iter() {
        let file = String::from("sets/") + &dataFile.text; //.push_str(&dataFile.text); 
        parseCards(&mut cards, &file);
    }
}

fn parseCards(cards: &mut Vec<MtgCard>, file: &str) {

    let f = File::open(file);
    match f {
        Ok(file_content) => {
            let reader = BufReader::new(file_content);
            for s in reader.lines() {
                // Could this fail?
                if let Some(card) = parseCard(&s.unwrap()) {
                    println!("{:?}", card);
                }
            }
        },
        Err(e) => println!("{:?}", e),
    }
}

fn parseCard(input: &str) -> Option<MtgCard> { // -> Result<MtgCard, &'static str> {
    let splitted = input.split('\t').collect::<Vec<&str>>(); 

    // Check the actual len
    if splitted.len() != 17 { None } 
    else {
        Some(MtgCard {
                name: splitted[0].to_owned(),
                set: splitted[1].to_owned() ,
                image: splitted[2].to_owned() ,
                color: splitted[3].to_owned() ,
                color_id: splitted[4].to_owned() ,
                cost: splitted[5].to_owned() ,
                mana_value: splitted[6].to_owned() ,
                card_type: splitted[7].to_owned() ,
                power: splitted[8].to_owned() ,
                thoughness: splitted[9].to_owned() ,
                loyalty: splitted[10].to_owned() ,
                rarity: splitted[11].to_owned() ,
                draft_qualities: splitted[12].to_owned() ,
                sound: splitted[13].to_owned() ,
                script: splitted[14].to_owned() ,
                text: splitted[15].to_owned() ,
            }
       )
    }
}
