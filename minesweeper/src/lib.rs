pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut ms: Vec<Vec<char>> = minefield.iter().map(|x| x.chars().collect()).collect();
    let height = ms.len() as isize;
    let width = match height {
        0 => 0,
        _ => ms[0].len(),
    } as isize;

    for (line, i) in ms.clone().iter().zip(0_isize..) {
        for (&cell, j) in line.iter().zip(0_isize..) {
            ms[i as usize][j as usize] = if cell != '*' {
                let ms_ref = &ms;
                let mines = (i - 1..=i + 1)
                    .filter(|&i| i >= 0 && i < height)
                    .flat_map(|x| {
                        (j - 1..=j + 1)
                            .filter(|&j| j >= 0 && j < width)
                            .map(move |y| ms_ref[x as usize][y as usize])
                    })
                    .filter(|&c| c == '*')
                    .count();
                match mines {
                    0 => ' ',
                    _ => ((mines as u8) + 0x30) as char,
                }
            } else {
                '*'
            }
        }
    }

    ms.iter().map(|a| a.iter().collect()).collect()
}
