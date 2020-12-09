use std::error::Error;
use std::fs;

pub fn read_into_matrix(filename: &str) -> Result<Vec<Vec<char>>, Box<dyn Error>> {
    let contents = fs::read_to_string(filename)?;

    let parsed = contents
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    Ok(parsed)
}

pub fn traverse(grid: Vec<Vec<char>>, hmove: u32, vmove: u32) -> Vec<char> {
    let mut result = vec![result_at(grid[0][0])];
    let mut vindex = vmove as usize;
    let mut hindex = hmove as usize;

    loop {
        let hlength = grid[vindex].len();
        result.push(result_at(grid[vindex][hindex % hlength]));

        hindex += hmove as usize;
        if vindex >= (grid.len() - 1) {
            break;
        } else if vindex < (grid.len() - 1) && (vindex + vmove as usize) >= (grid.len() - 1) {
            vindex = grid.len() - 1;
        } else {
            vindex += vmove as usize;
        }
    }

    result
}

fn result_at(terrain: char) -> char {
    match terrain {
        '.' => 'O',
        '#' => 'X',
        _ => '!',
    }
}
