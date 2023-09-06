use dioxus::prelude::*;
use std::fs::{File, read_to_string};
use std::io::{self, prelude::*, BufReader};
use crate::xml_parser::{CardDataFiles, FileToInclude};
use crate::misc::loadFile;
use crate::mtg_card::MtgCard;
use hard_xml::XmlRead;

/// Load all mtg cards.
pub fn load_mtg_cards() -> Result<Vec<MtgCard>, String> {

    let mut result: Vec<MtgCard> = Vec::with_capacity(60000);
    let mut error = String::new();

    let file_str = loadFile("ListOfCardDataFiles.txt").map_err(|_| "Failed to load file 'ListOfCardDataFiles.txt'".to_string())?;
    let card_files = CardDataFiles::from_str(&file_str).map_err(|_|"Failed to parse file 'ListOfCardDataFiles.txt'".to_string())?;

    for dataFile in card_files.files.iter() {
        let file = String::from("sets/") + &dataFile.text;
        parse_mtg_cards(&mut result, &file);
    }

    Ok(result)
}

/// Parse all mtg cards from the given file. Append parsed cards to given cards vector. 
fn parse_mtg_cards(cards: &mut Vec<MtgCard>, file: &str) -> Result<(), String> {

    let src = loadFile(file).map_err(|_| format!("Failed to load file {:?}", file).to_string())?;

    let mut result: Result<(), String> = Ok(());
    for s in src.lines() {
        if let Some(card) = MtgCard::parse(&s) {
            cards.push(card);
        }
    }
    result
}

// /// Parse a single mtg card from given input str.
// fn parse_mtg_card(input: &str) -> Option<MtgCard> { // -> Result<MtgCard, &'static str> {
//     let splitted = input.split('\t').collect::<Vec<&str>>(); 
// 
//     // Check the actual len
//     if splitted.len() != 17 { None } 
//     else {
//         Some(MtgCard {
//                 name: splitted[0].to_owned(),
//                 set: splitted[1].to_owned(),
//                 image: splitted[2].to_owned(),
//                 color: splitted[3].to_owned(),
//                 color_id: splitted[4].to_owned(),
//                 cost: splitted[5].to_owned(),
//                 mana_value: splitted[6].to_owned(),
//                 card_type: splitted[7].to_owned(),
//                 power: splitted[8].to_owned(),
//                 thoughness: splitted[9].to_owned(),
//                 loyalty: splitted[10].to_owned(),
//                 rarity: splitted[11].to_owned(),
//                 draft_qualities: splitted[12].to_owned(),
//                 sound: splitted[13].to_owned(),
//                 script: splitted[14].to_owned(),
//                 text: splitted[15].to_owned(),
//             }
//        )
//     }
// }
