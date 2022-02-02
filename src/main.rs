/*
 * I think what I'd like to do is to have a struct representing a line of text, one for each term
 * column, with a `tick` method, and then a function that takes a list of such structs and prints
 * them. Maybe the structs should have a vector of crossterm::style::StyledContent for not just
 * characters but also colors.
 */

use colored::Colorize;
use crossterm::cursor::{MoveDown, MoveLeft, MoveTo};
use crossterm::style::Print;
use crossterm::terminal::{
    /*disable_raw_mode,*/ enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use crossterm::{event, execute, queue};
use rand::Rng;
use std::io;
use std::io::{stdout, Write};
use std::time::Duration;
use term_size;

fn rand_char(candidates: &Vec<char>) -> char {
    let mut rng = rand::thread_rng();

    candidates[rng.gen_range(0..candidates.len())]
}

fn rand_line(length: u32, candidates: &Vec<char>) -> Vec<char> {
    let mut s = vec![];
    for _ in 0..length {
        s.push(rand_char(candidates));
    }

    s
}

fn main() -> Result<(), io::Error> {
    let (width, height) = term_size::dimensions().unwrap();
    let delay = 50; // Milliseconds

    let mut stdout = stdout();
    let mut rng = rand::thread_rng();

    // Characters to randomly select from to display:
    let katakana = vec![
        'ｦ', 'ｧ', 'ｨ', 'ｩ', 'ｪ', 'ｫ', 'ｬ', 'ｭ', 'ｮ', 'ｯ', 'ｰ', 'ｱ', 'ｲ', 'ｳ', 'ｴ', 'ｵ', 'ｶ', 'ｷ',
        'ｸ', 'ｹ', 'ｺ', 'ｻ', 'ｼ', 'ｽ', 'ｾ', 'ｿ', 'ﾀ', 'ﾁ', 'ﾂ', 'ﾃ', 'ﾄ', 'ﾅ', 'ﾆ', 'ﾇ', 'ﾈ', 'ﾉ',
        'ﾊ', 'ﾋ', 'ﾌ', 'ﾍ', 'ﾎ', 'ﾏ', 'ﾐ', 'ﾑ', 'ﾒ', 'ﾓ', 'ﾔ', 'ﾕ', 'ﾖ', 'ﾗ', 'ﾘ', 'ﾙ', 'ﾚ', 'ﾛ',
        'ﾜ', 'ﾝ',
    ];
    let ascii = vec![
        '!', '\"', '#', '$', '%', '&', '\'', '(', ')', '*', '+', ',', '-', '.', '/', '0', '1', '2',
        '3', '4', '5', '6', '7', '8', '9', ':', ';', '<', '=', '>', '?', '@', 'A', 'B', 'C', 'D',
        'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V',
        'W', 'X', 'Y', 'Z', '[', '\\', ']', '^', '_', '`', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h',
        'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
        '{', '|', '}', '~',
    ];
    let candidates = [katakana, ascii].concat();

    let mut lines = vec![];
    let mut y_positions: Vec<i16> = vec![];
    let mut lengths = vec![];
    for _ in 0..width {
        y_positions.push(rng.gen_range(-(height as i16)..1));
        lines.push(vec![]);
        lengths.push(rng.gen_range(5..15));
    }

    enable_raw_mode()?;

    queue!(stdout, EnterAlternateScreen, MoveTo(0, 0))?;

    loop {
        for col in 0..width {
            y_positions[col] += 1;
            let y_pos = y_positions[col];
            let length = lengths[col];
            if lines[col].len() >= length {
                lines[col] = lines[col][1..].to_vec();
            }
            lines[col].push(rand_char(&candidates));
            let line = &mut lines[col];

            let visible_tail_length = if y_pos < line.len() as i16 {
                y_pos
            } else {
                line.len() as i16
            };

            let end_of_visible_tail = y_pos - visible_tail_length;

            // Clear the character one up from the end of the line:
            if end_of_visible_tail > 0 {
                queue!(
                    stdout,
                    MoveTo(col as u16, (end_of_visible_tail - 1) as u16),
                    Print(" ")
                )?;
            }

            // Print the line:
            queue!(stdout, MoveTo(col as u16, end_of_visible_tail as u16))?;
            for n in 0..visible_tail_length {
                if y_pos - (visible_tail_length - n) <= height as i16 {
                    let c = if n == visible_tail_length - 1 {
                        line[line.len() - visible_tail_length as usize..][n as usize]
                            .to_string()
                            .white()
                    } else {
                        line[line.len() - visible_tail_length as usize..][n as usize]
                            .to_string()
                            .green()
                    };
                    print!("{}", c);
                    queue!(stdout, MoveDown(1), MoveLeft(1))?;
                }
            }

            if end_of_visible_tail as usize > height {
                y_positions[col] = rng.gen_range(-30..1);
                lines[col] = rand_line(rng.gen_range(5..15), &candidates);
            }
        }
        stdout.flush()?;

        // Spend `duration` waiting for keyboard input:
        if event::poll(Duration::from_millis(delay)).unwrap() {
            execute!(stdout, LeaveAlternateScreen)?;
            // disable_raw_mode()?; // Seems unnecessary?
            return Ok(());
        }
    }
}
