use unicode_segmentation::UnicodeSegmentation;

//pub fn reverse(input: &str) -> String {
//    let mut res = String::new();
//    for c in input.chars().rev() {
//        res.push(c);
//    }
//    return res;
//}

pub fn reverse(input: &str) -> String {
    input.graphemes(true).rev().collect()
}


