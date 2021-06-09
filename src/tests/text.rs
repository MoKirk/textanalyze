use crate::text::*;

#[test]
fn it_can_be_created_from_a_string() {
    let txt = Text::from_string("abcd");
    assert_eq!(txt.inner_text(), "abcd");
}


#[test] 
fn it_can_split_the_text_by_lines() {
    let txt = Text::from_string("abcd\nefgh");
    let mut lines = txt.as_lines();
    assert_eq!("abcd", lines.next().unwrap());
    assert_eq!("efgh", lines.next().unwrap());
}

#[test] 
fn it_can_split_the_text_by_sentences() {
    let txt = Text::from_string("abcd. efgh? ijk! mnop. qrz ast");
    let mut sentences = txt.as_sentences().into_iter();
    // TODO: implement this
    assert_eq!("abcd.", sentences.next().unwrap());
    assert_eq!("efgh?", sentences.next().unwrap());
    assert_eq!("ijk!", sentences.next().unwrap());
    assert_eq!("mnop.", sentences.next().unwrap());
    assert_eq!("qrz ast", sentences.next().unwrap());
}
