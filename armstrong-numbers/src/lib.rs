pub fn is_armstrong_number(num: u32) -> bool {
    let num_str = format!("{}", num);
    let len = num_str.len() as u32;
    let armstrong = num_str
        .chars()
        .map(|c| c.to_digit(10).unwrap().pow(len))
        .sum::<u32>();
    armstrong == num
}
