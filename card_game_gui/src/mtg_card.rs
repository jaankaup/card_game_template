use dioxus::prelude::*;

const image_style: &str = r#"
    padding-left: 2px;
    display: inline-block;
"#;

#[derive(Debug, PartialEq, Clone)]
pub struct MtgCard {
    pub name: String,
    pub set: String,
    pub image: String,
    pub color: String,
    pub color_id: String,
    pub cost: String,
    pub mana_value: String,
    pub card_type: String,
    pub power: String,
    pub thoughness: String,
    pub loyalty: String,
    pub rarity: String,
    pub draft_qualities: String,
    pub sound: String,
    pub script: String,
    pub text: String,
}

impl MtgCard {

    /// Create a new MtgCard from a string. TODO: validate.
    pub fn parse(input_str: &str) -> Option<MtgCard> {

        let splitted = input_str.split('\t').collect::<Vec<&str>>(); 

        // Check the actual len
        if splitted.len() != 17 { None } 
        else {
            Some(MtgCard {
                name: splitted[0].to_owned(),
                set: splitted[1].to_owned(),
                image: splitted[2].to_owned(),
                color: splitted[3].to_owned(),
                color_id: splitted[4].to_owned(),
                cost: splitted[5].to_owned(),
                mana_value: splitted[6].to_owned(),
                card_type: splitted[7].to_owned(),
                power: splitted[8].to_owned(),
                thoughness: splitted[9].to_owned(),
                loyalty: splitted[10].to_owned(),
                rarity: splitted[11].to_owned(),
                draft_qualities: splitted[12].to_owned(),
                sound: splitted[13].to_owned(),
                script: splitted[14].to_owned(),
                text: splitted[15].to_owned(),
            }
            )
        }
    }
}

fn image_file(mtg_card: &MtgCard) -> String {
    "sets/setimages/".to_owned() + &mtg_card.set.to_owned() + "/" + &mtg_card.image.to_owned() + ".jpg"
}

#[allow(non_snake_case)]
#[inline_props]
pub fn CardImage<'a>(cx: Scope<'a>, mtg_card: &'a MtgCard) -> Element {

    // let image_dimension = use_shared_state::<SharedImageDimension>(cx).unwrap();

    let style_state = use_state(cx, || image_style);

    cx.render(rsx!{
        p {
            // onmouseover: move |_| { image_dimension.modify(|_| dimensions_hover) }, //style: "{image_hovered}" },
            // onmouseleave: move |_| { image_dimension.modify(|_| dimensions_no_hover) },// style_state.write() = image_hovered; },
            style: "{style_state.current()}",
            img {
                src: "{image_file(mtg_card)}",
                width: "150px",
                height: "214px",
            }
        }
    })
}
