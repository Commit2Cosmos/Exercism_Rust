pub fn annotate(minefield: &[&str]) -> Vec<String> {
    (0..minefield.len()).map(|x| {
        String::from_utf8(minefield[x].as_bytes().iter().enumerate().map(|(y, c)|  {
            match c {
                b' ' => {
                    let mut mines: u8 = 0;
                    for j in x.saturating_sub(1)..=(x+1).min(minefield.len()-1) {
                        mines += minefield[j][y.saturating_sub(1)..=(y+1).min(minefield[x].len()-1)].as_bytes().iter().filter(|&c| *c == b'*').count() as u8
                    }

                    match mines {
                        0 => b' ',
                        _ => mines + b'0'
                    }
                },
                _ => *c
            }
        }).collect()).unwrap()
    })
    .collect::<Vec<String>>()
}
