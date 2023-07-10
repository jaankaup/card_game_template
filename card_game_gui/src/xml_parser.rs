use std::borrow::Cow;
use hard_xml::{XmlRead, XmlWrite};

// #[derive(XmlWrite, XmlRead, PartialEq, Debug)]
// #[xml(tag = "listofcarddatafiles")]
// pub struct CardDataFiles<'a> {
//     #[xml(child = "filetoinclude")]
//     filetoinclude: Vec<FileToInclude<'a>>,
// }
// 
// #[derive(XmlWrite, XmlRead, PartialEq, Debug)]
// #[xml(tag = "filetoinclude")]
// pub struct FileToInclude<'a> {
//     #[xml("text")]
//     text: Cow<'a, str>,
// }

#[derive(XmlWrite, XmlRead, PartialEq, Debug)]
#[xml(tag = "listofcarddatafiles")]
pub struct CardDataFiles<'a> {
    #[xml(child = "filetoinclude")]
    pub files: Vec<FileToInclude<'a>>,
}

#[derive(XmlWrite, XmlRead, PartialEq, Debug)]
#[xml(tag = "filetoinclude")]
pub struct FileToInclude<'a> {
    #[xml(text)]
    pub text: Cow<'a, str>,
}

// fn main() {
// 
//     let heko = CardDataFiles {
//                     filetoinclude: vec![
//                                FileToInclude { text: "eki".into()}
//                                ] }.to_string().unwrap();
// }
