use std::{
    convert::TryInto,
    io::{stdin, stdout, Write},
};

fn win_condition(arr: [i32; 2]) {
    println!(
        "{} won!",
        match arr.iter().enumerate().map(|(x, y)| (y, x)).max().unwrap().1 {
            0 => "X",
            1 => "O",
            _ => {
                panic!("Invalid number")
            }
        }
    );
}

fn main() {
    let mut grid = [[' ', ' ', ' '], [' ', ' ', ' '], [' ', ' ', ' ']];
    let mut gameover = false;
    let stdin = stdin();
    let mut stdout = stdout();
    let mut cross = true;
    while !gameover {
        let mut valid = false;
        let mut arr: [usize; 2] = [0, 0];
        while !valid {
            let mut inp = String::new();
            print!("Enter row and column separated by commas: ");
            stdout.flush().expect("Failed to flush");
            stdin
                .read_line(&mut inp)
                .expect("Failed to read text from terminal");
            inp = inp.trim().to_string();
            let arr_inp = inp
                .split(',')
                .map(|m| match m.trim().parse::<usize>() {
                    Ok(i) => {
                        if i > 0 {
                            i - 1
                        } else {
                            usize::MAX
                        }
                    }
                    Err(_) => usize::MAX,
                })
                .collect::<Vec<usize>>();
            if arr_inp.contains(&usize::MAX) || arr_inp.len() != 2 {
                println!("Invalid input!");
                continue;
            } else if arr_inp.iter().any(|i| i >= &3) {
                println!("Value(s) not in range (1-3)!");
                continue;
            } else if grid[arr_inp[0]][arr_inp[1]] != ' ' {
                println!("Space is taken");
                continue;
            }
            valid = true;
            arr = arr_inp.try_into().unwrap();
        }
        grid[arr[0]][arr[1]] = if cross { 'X' } else { 'O' };
        for line in grid.iter() {
            println!("{:?}", line);
        }
        let mut v_char_count = [[0, 0], [0, 0], [0, 0]];
        for y in 0..grid.len() {
            let mut h_char_count = [0, 0]; // X, O
            for x in 0..grid[y].len() {
                match grid[y][x] {
                    ' ' => {}
                    'X' => {
                        h_char_count[0] += 1;
                        v_char_count[x][0] += 1;
                    }
                    'O' => {
                        h_char_count[1] += 1;
                        v_char_count[x][1] += 1;
                    }
                    _ => panic!("Odd character!!!"),
                }
            }
            if h_char_count.contains(&3) {
                win_condition(h_char_count);
                gameover = true;
            }
        }
        let mut total = 0;
        for value in v_char_count.iter() {
            if value.contains(&3) {
                win_condition(*value);
                gameover = true;
            }
            total += value[0] + value[1]
        }
        if total == 9 {
            println!("Draw!");
            gameover = true;
        }
        if (grid[0][0] == grid[1][1] && grid[1][1] == grid[2][2]
            || grid[0][2] == grid[1][1] && grid[1][1] == grid[2][0])
            && grid[1][1] != ' '
        {
            println!("{} won!", grid[1][1]);
            gameover = true;
        }
        cross ^= true;
    }
}
