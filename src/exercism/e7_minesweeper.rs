pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let dirs = [(-1,-1),(-1,0),(-1,1),(0,-1),(0,1),(1,-1),(1,0),(1,1)];
    minefield.iter().enumerate().map(|(r,row)| {
        row.chars().enumerate().map(|(c,cell)| {
            if cell == '*' {
                '*'
            } else {
                let count = dirs.iter().filter(|&&(dr,dc)| {
                    let (nr,nc) = (r as isize+dr,c as isize+dc);
                    nr>=0 && nr<minefield.len() as isize && nc>=0 && nc<row.len() as isize && minefield[nr as usize].chars().nth(nc as usize) == Some('*')
                }).count();
                if count==0 {' '} else {char::from_digit(count as u32,10).unwrap()}
            }
        }).collect::<String>()
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_rows() {
        let input = &[];
        let expected: &[&str] = &[];
        let actual = annotate(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn space_surrounded_by_mines() {
        let (input,expected) = (&[
            "***",
            "* *",
            "***",
        ], &[
            "***",
            "*8*",
            "***",
        ]);
        let actual = annotate(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn large_minefield() {
        let (input,expected) = (&[
            " *  * ",
            "  *   ",
            "    * ",
            "   * *",
            " *  * ",
            "      ",
        ], &[
            "1*22*1",
            "12*322",
            " 123*2",
            "112*4*",
            "1*22*2",
            "111111",
        ]);
        let actual = annotate(input);
        assert_eq!(actual, expected);
    }
}
