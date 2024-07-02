use std::io::{stdin, stdout, Write};
use termion::event::{Event, Key};
use termion::input::{MouseTerminal, TermRead};
use termion::raw::IntoRawMode;
use crate::Game;
use crate::graphics;
use crate::graphics::draw;
use crate::Round;

pub(crate) fn move_piece(s: &mut Game) -> bool {

    let original_board = s.board.board;

    let command = get_command();

    if command_is_valid(command.clone()) == true {

        let pos = get_position(command.clone());

        if can_move(s, pos.clone()) {

            let instructions = get_instructions(s, pos.clone());

            s.board.board = execute_instructions(original_board, instructions);

            // check for mates

            if is_mate(s).0 == true {
                graphics::end_screen(is_mate(s).1);
            }

            // check if king is in danger

            return match s.round {
                Round::White => {
                    if king_in_danger(s, 11) == false {
                        update_castles_king(s);
                        promotion_screen(s);
                        true
                    } else {
                        s.board.board = original_board;
                        false
                    }
                }
                Round::Black => {
                    if king_in_danger(s, 5) == false {
                        update_castles_king(s);
                        promotion_screen(s);
                        true
                    } else {
                        s.board.board = original_board;
                        false
                    }
                }
            }


        }
    }

    return false
}

fn is_mate(s: &mut Game) -> (bool, i32) {

    let original_board = s.board.board;

    // check if there is a check on a king

    if king_in_danger(s, 11) == true {

        // check if no moves are possible

        for x in 0..8 {
            for y in 0..8 {
                if s.board.board[x][y] == 7 || s.board.board[x][y] == 8 || s.board.board[x][y] == 9 || s.board.board[x][y] == 10 || s.board.board[x][y] == 11 || s.board.board[x][y] == 12 {
                    let mut list_of_moves = get_possible_moves(vec![x as i32, y as i32], s, s.board.board[x][y] as i32);

                    for m in list_of_moves {

                        let mut piece = s.board.board[x][y];

                        let mut instructions = vec![vec![x as i32, y as i32, 0], vec![m[0] as i32, m[1] as i32, piece as i32]];

                        s.board.board = execute_instructions(s.board.board, instructions);

                        if king_in_danger(s, 11) == false {
                            s.board.board = original_board;
                            return (false, 11);
                        }

                        s.board.board = original_board;

                    }
                }
            }
        }

        return (true, 11);

    } else if king_in_danger(s, 5) == true {

        // check if no moves are possible

        for x in 0..8 {
            for y in 0..8 {
                if s.board.board[x][y] == 1 || s.board.board[x][y] == 2 || s.board.board[x][y] == 3 || s.board.board[x][y] == 4 || s.board.board[x][y] == 5 || s.board.board[x][y] == 6 {
                    let mut list_of_moves = get_possible_moves(vec![x as i32, y as i32], s, s.board.board[x][y] as i32);

                    for m in list_of_moves {

                        let mut piece = s.board.board[x][y];

                        let mut instructions = vec![vec![x as i32, y as i32, 0], vec![m[0] as i32, m[1] as i32, piece as i32]];

                        s.board.board = execute_instructions(s.board.board, instructions);

                        if king_in_danger(s, 5) == false {
                            s.board.board = original_board;
                            return (false, 5);
                        }

                        s.board.board = original_board;

                    }
                }
            }
        }

        return (true, 5);
    } else {
        return (false, 0);
    }


}

fn promotion_screen(s: &mut Game) {

    let mut pawn_position= vec![8, 8];

    // check row 0

    for x in 0..8 {
        if s.board.board[0][x] == 12 {
            graphics::promotion_screen();
            pawn_position = vec![0, x as i32];
        }
    }

    // check row 7

    for x in 0..8 {
        if s.board.board[7][x] == 6 {
            graphics::promotion_screen();
            pawn_position = vec![7, x as i32];
        }
    }

    if pawn_position == vec![8, 8] {
        return;
    }

    let mut piece: i32 = 0;

    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    for c in stdin.keys() {
        write!(stdout,
               "{}{}",
               termion::cursor::Goto(1, 1),
               termion::clear::CurrentLine)
            .unwrap();

        match c.unwrap() {
            Key::Char('R') | Key::Char('r' )=> {
                piece = 1;
                break;
            },
            Key::Char('N') | Key::Char('n') => {
                piece = 2;
                break;
            },
            Key::Char('B') | Key::Char('b') => {
                piece = 3;
                break;
            }
            Key::Char('Q') | Key::Char('q') => {
                piece = 4;
                break;
            }
            _ => {}
        }
    }

    match piece {
        1 => {
            match pawn_position[0] {
                0 => {
                    s.board.board[pawn_position[0] as usize][pawn_position[1] as usize] = 7
                }
                7 => {
                    s.board.board[pawn_position[0] as usize][pawn_position[1] as usize] = 1
                }
                _ => {}
            }
        }
        2 => {
            match pawn_position[0] {
                0 => {
                    s.board.board[pawn_position[0] as usize][pawn_position[1] as usize] = 8
                }
                7 => {
                    s.board.board[pawn_position[0] as usize][pawn_position[1] as usize] = 2
                }
                _ => {}
            }
        }
        3 => {
            match pawn_position[0] {
                0 => {
                    s.board.board[pawn_position[0] as usize][pawn_position[1] as usize] = 9

                }
                7 => {
                    s.board.board[pawn_position[0] as usize][pawn_position[1] as usize] = 3
                }
                _ => {}
            }
        }
        4 => {
            match pawn_position[0] {
                0 => {
                    s.board.board[pawn_position[0] as usize][pawn_position[1] as usize] = 10
                }
                7 => {
                    s.board.board[pawn_position[0] as usize][pawn_position[1] as usize] = 4
                }
                _ => {}
            }
        }
        _ => {}
    }


}

fn king_in_danger(s: &mut Game, k: i32) -> bool {

    // find out whose turn it is

    match k {
        11 => {
            // check if the white king is in danger (check on the white king)

            let mut danger_zones: Vec<Vec<i32>> = vec![];
            let mut white_king_position: Vec<i32> = vec![8, 8];

            for x in 0..8 {
                for y in 0..8 {
                    if s.board.board[x][y] == 11 {
                        white_king_position = vec![x as i32, y as i32];
                    }
                }
            }

            // find all the places black pieces can go

            for x in 0..8 {
                for y in 0..8 {
                    match s.board.board[x][y] {
                        1 | 2 | 3 | 4 | 5 | 6 => {
                            let mut individual_danger_zones = get_possible_moves(vec![x as i32, y as i32], s, s.board.board[x as usize][y as usize] as i32);
                            danger_zones.extend(individual_danger_zones.clone());
                        }
                        _ => {}
                    }
                }
            }

            if danger_zones.contains(&white_king_position) {
                return true
            }

        }
        5 => {
            // check if the white king is in danger (check on the white king)

            let mut danger_zones: Vec<Vec<i32>> = vec![];
            let mut black_king_position: Vec<i32> = vec![8, 8];

            for x in 0..8 {
                for y in 0..8 {
                    if s.board.board[x][y] == 5 {
                        black_king_position = vec![x as i32, y as i32];
                    }
                }
            }

            // find all the places white pieces can go

            for x in 0..8 {
                for y in 0..8 {
                    match s.board.board[x][y] {
                        7 | 8 | 9 | 10 | 11 | 12 => {
                            let mut individual_danger_zones = get_possible_moves(vec![x as i32, y as i32], s, s.board.board[x as usize][y as usize] as i32);
                            danger_zones.extend(individual_danger_zones.clone());
                        }
                        _ => {}
                    }
                }
            }

            if danger_zones.contains(&black_king_position) {
                return true
            }

        }
        _ => {}
    }

    return false
}

fn execute_instructions(board: [[usize; 8]; 8], instructions: Vec<Vec<i32>>) -> [[usize; 8]; 8] {

    let mut new_board = board;

    for instruction in instructions {
        new_board[instruction[0] as usize][instruction[1] as usize] = instruction[2] as usize;
    }

    return new_board;
}

fn get_instructions(s: &mut Game, position: Vec<i32>) -> Vec<Vec<i32>> {
    let mut instructions = vec![];

    if is_castling(s, position.clone()) {

        match s.board.board[position[0].clone() as usize][position[1].clone() as usize] {
            11 => {
                match position[3] {
                    0 => {
                        instructions.push(vec![7, 4, 0]);
                        instructions.push(vec![7, 0, 0]);

                        instructions.push(vec![7, 2, 11]);
                        instructions.push(vec![7, 3, 7]);
                    }
                    7 => {
                        instructions.push(vec![7, 4, 0]);
                        instructions.push(vec![7, 7, 0]);

                        instructions.push(vec![7, 6, 11]);
                        instructions.push(vec![7, 5, 7]);
                    }
                    _ => {}
                }
            }
            5 => {
                match position[3] {
                    0 => {
                        instructions.push(vec![0, 4, 0]);
                        instructions.push(vec![0, 0, 0]);

                        instructions.push(vec![0, 2, 5]);
                        instructions.push(vec![0, 3, 1]);
                    }
                    7 => {
                        instructions.push(vec![0, 4, 0]);
                        instructions.push(vec![0, 7, 0]);

                        instructions.push(vec![0, 6, 5]);
                        instructions.push(vec![0, 5, 1]);
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    } else {
        let last_piece = s.board.board[position[0].clone() as usize][position[1].clone() as usize];

        instructions.push(vec![position[0], position[1], 0]);
        instructions.push(vec![position[2], position[3], last_piece as i32]);
    }

    return instructions
}

fn command_is_valid(command: String) -> bool{

    // check if command is correct length (7)

    let mut length_index = 0;

    for char in command.chars() {
        length_index += 1;
    }

    if length_index != 7 {
        return false
    }

    // check if command follows the format N:N>N:N

    let number_index = [0, 2, 4, 6];
    let duble_point_index = [1, 5];

    for x in 0..4 {

        let index = number_index[x];

        match command.chars().nth(index) {
            Some('0') | Some('1') | Some('2') | Some('3') | Some('4') | Some('5') | Some('6') | Some('7') => {}
            _ => {
                return false
            }
        }
    }

    for x in 0..2 {

        let index = duble_point_index[x];

        match command.chars().nth(index) {
            Some(':') => {}
            _ => {
                return false
            }
        }

    }

    match command.chars().nth(3) {
        Some('>') => {}
        _ => {
            return false
        }
    }

    return true

}

fn get_position(command: String) -> Vec<i32> {

    let mut position = vec![];

    for char in command.chars() {
        match char.to_string().parse::<i32>() {
            Ok(value) => {
                position.push(value)
            }
            Err(e) => {}
        }
    }

    return position

}

fn get_command() -> String {
    let mut command: String = "".to_string();

    let mut stdout = MouseTerminal::from(stdout().into_raw_mode().unwrap());
    let stdin = stdin();

    stdout.flush().unwrap();

    graphics::display_move(command.clone());

    for c in stdin.events() {
        let evt = c.unwrap();
        match evt {
            Event::Key(Key::Char('q')) => {
                graphics::draw(5, 12, "                     ".to_string(), "white");
                break
            },
            Event::Key(Key::Char('e')) => {
                graphics::draw(5, 12, "                     ".to_string(), "white");
                break
            }

            Event::Key(Key::Char(c)) => {
                command.push(c);
                graphics::display_move(command.clone());
            },

            _ => {}
        }
        stdout.flush().unwrap();
    }

    return command;
}

fn get_possible_moves(position: Vec<i32>, s: &mut Game, piece: i32) -> Vec<Vec<i32>> {
    let mut positions = vec![];

    match piece {

        // White pawn

        12 => {

            if position[0] != 0 {

                if s.board.board[position[0] as usize - 1][position[1] as usize] == 0 {
                    positions.push(vec![position[0] - 1, position[1]])
                }

                // check to the left

                if position[1] != 0 {
                    if s.board.board[position[0] as usize - 1][position[1] as usize - 1] == 0 {
                        positions.push(vec![position[0] - 1, position[1] - 1])
                    }
                }

                // check to the right

                if position[1] != 7 {
                    if s.board.board[position[0] as usize - 1][position[1] as usize + 1] == 0 {
                        positions.push(vec![position[0] - 1, position[1] + 1])
                    }
                }

            }

            if position[0] == 6 {
                if s.board.board[position[0] as usize - 2][position[1] as usize] == 0 {
                    positions.push(vec![position[0] - 2, position[1]])
                }
            }

        },

        // Black pawn

        6 => {
            if position[0] != 7 {

                if s.board.board[position[0] as usize + 1][position[1] as usize] == 0 {
                    positions.push(vec![position[0] + 1, position[1]])
                }

                // check to the left

                if position[1] != 0 {
                    if s.board.board[position[0] as usize + 1][position[1] as usize - 1] == 0 {
                        positions.push(vec![position[0] + 1, position[1] - 1])
                    }
                }

                // check to the right

                if position[1] != 7 {
                    if s.board.board[position[0] as usize + 1][position[1] as usize + 1] == 0 {
                        positions.push(vec![position[0] + 1, position[1] + 1])
                    }
                }

            }

            if position[0] == 1 {
                if s.board.board[position[0] as usize + 2][position[1] as usize] == 0 {
                    positions.push(vec![position[0] + 2, position[1]])
                }
            }
        }

        // Towers

        7 | 1 => {

            let mut token = 1;

            // Top

            let mut top_token: bool = true;

            while top_token == true {
                if (position[0] + 10) - token <= 9 {
                    top_token = false
                } else {
                    match s.board.board[position[0] as usize - token as usize][position[1] as usize] {
                        // Checks if the position up token is White or Black

                        // Black => Pushes that position and closes loop

                        1 | 2 | 3 | 4 | 5 | 6 => {
                            match piece {
                                7 => {
                                    positions.push(vec![position[0] - token, position[1]]);
                                    top_token = false;
                                }
                                1 => {
                                    top_token = false;
                                }
                                _ => {}
                            }

                        }

                        // White => Closes loop

                        7 | 8 | 9 | 10 | 11 | 12 => {
                            match piece {
                                1 => {
                                    positions.push(vec![position[0] - token, position[1]]);
                                    top_token = false;
                                }
                                7 => {
                                    top_token = false;
                                }
                                _ => {}
                            }
                        }

                        _ => {
                            positions.push(vec![position[0] - token, position[1]]);
                            token += 1
                        }
                    }
                }
            }

            // Down

            token = 1;

            let mut down_token: bool = true;

            while down_token == true {

                if (position[0] + 10) + token >= 18  {
                    down_token = false
                } else {
                    match s.board.board[position[0] as usize + token as usize][position[1] as usize] {
                        // Checks if the position up token is White or Black

                        // Black => Pushes that position and closes loop

                        1 | 2 | 3 | 4 | 5 | 6 => {
                            match piece {
                                7 => {
                                    positions.push(vec![position[0] + token, position[1]]);
                                    down_token = false;
                                }
                                1 => {
                                    down_token = false;
                                }
                                _ => {}
                            }
                        }

                        // White => Closes loop

                        7 | 8 | 9 | 10 | 11 | 12 => {
                            match piece {
                                1 => {
                                    positions.push(vec![position[0] + token, position[1]]);
                                    down_token = false;
                                }
                                7 => {
                                    down_token = false;
                                }
                                _ => {}
                            }
                        }

                        _ => {
                            positions.push(vec![position[0] + token, position[1]]);
                            token += 1
                        }
                    }
                }
            }

            // Left

            token = 1;

            let mut left_token: bool = true;

            while left_token == true {

                if (position[1] + 10) - token <= 9  {
                    left_token = false
                } else {
                    match s.board.board[position[0] as usize][position[1] as usize - token as usize] {
                        // Checks if the position up token is White or Black

                        // Black => Pushes that position and closes loop

                        1 | 2 | 3 | 4 | 5 | 6 => {
                            match piece {
                                7 => {
                                    positions.push(vec![position[0], position[1] - token]);
                                    left_token = false;
                                }
                                1 => {
                                    left_token = false;
                                }
                                _ => {}
                            }
                        }

                        // White => Closes loop

                        7 | 8 | 9 | 10 | 11 | 12 => {
                            match piece {
                                1 => {
                                    positions.push(vec![position[0], position[1] - token]);
                                    left_token = false;
                                }
                                7 => {
                                    left_token = false;
                                }
                                _ => {}
                            }
                        }

                        _ => {
                            positions.push(vec![position[0], position[1] - token]);
                            token += 1
                        }
                    }
                }
            }

            // Right

            token = 1;

            let mut right_token: bool = true;

            while right_token == true {
                if (position[1] + 10) + token >= 18 {
                    right_token = false
                } else {
                    match s.board.board[position[0] as usize][position[1] as usize + token as usize] {
                        // Checks if the position up token is White or Black

                        // Black => Pushes that position and closes loop

                        1 | 2 | 3 | 4 | 5 | 6 => {
                            match piece {
                                7 => {
                                    positions.push(vec![position[0], position[1] + token]);
                                    right_token = false;
                                }
                                1 => {
                                    right_token = false;
                                }
                                _ => {}
                            }
                        }

                        // White => Closes loop

                        7 | 8 | 9 | 10 | 11 | 12 => {
                            match piece {
                                1 => {
                                    positions.push(vec![position[0], position[1] + token]);
                                    right_token = false;
                                }
                                7 => {
                                    right_token = false;
                                }
                                _ => {}
                            }
                        }

                        _ => {
                            positions.push(vec![position[0], position[1] + token]);
                            token += 1
                        }
                    }
                }
            }


        }

        // Knights

        8 | 2 => {

            // Up 2 positions

            if (position[0] + 10) - 2 >= 10 {

                // Left

                if (position[1] + 10) - 1 >= 10 {
                    match piece {
                        8 => {
                            match s.board.board[position[0] as usize - 2][position[1] as usize - 1] {
                                0 | 1 | 2 | 3 | 4 | 5 | 6 => {
                                    positions.push(vec![position[0] - 2, position[1] - 1])
                                }
                                _ => {}
                            }
                        }
                        2 => {
                            match s.board.board[position[0] as usize - 2][position[1] as usize - 1] {
                                0 | 7 | 8 | 9 | 10 | 11 | 12 => {
                                    positions.push(vec![position[0] - 2, position[1] - 1])
                                }
                                _ => {}
                            }
                        }
                        _ => {}
                    }
                }

                // Right

                if position[1] + 1 <= 7 {

                    match piece {
                        8 => {
                            match s.board.board[position[0] as usize - 2][position[1] as usize + 1] {
                                0 | 1 | 2 | 3 | 4 | 5 | 6 => {
                                    positions.push(vec![position[0] - 2, position[1] + 1])
                                }
                                _ => {}
                            }
                        }
                        2 => {
                            match s.board.board[position[0] as usize - 2][position[1] as usize + 1] {
                                0 | 7 | 8 | 9 | 10 | 11 | 12 => {
                                    positions.push(vec![position[0] - 2, position[1] + 1])
                                }
                                _ => {}
                            }
                        }
                        _ => {}
                    }

                }

            }

            // Up 1 position

            if (position[0] + 10) - 1 >= 10 {

                // Left

                if (position[1] + 10) - 2 >= 10 {

                    match piece {
                        8 => {
                            match s.board.board[position[0] as usize - 1][position[1] as usize - 2] {
                                0 | 1 | 2 | 3 | 4 | 5 | 6 => {
                                    positions.push(vec![position[0] - 1, position[1] - 2])
                                }
                                _ => {}
                            }
                        }
                        2 => {
                            match s.board.board[position[0] as usize - 1][position[1] as usize - 2] {
                                0 | 7 | 8 | 9 | 10 | 11 | 12 => {
                                    positions.push(vec![position[0] - 1, position[1] - 2])
                                }
                                _ => {}
                            }
                        }
                        _ => {}
                    }

                }

                // Right

                if position[1] + 2 <= 7 {

                    match piece {
                        8 => {
                            match s.board.board[position[0] as usize - 1][position[1] as usize + 2] {
                                0 | 1 | 2 | 3 | 4 | 5 | 6 => {
                                    positions.push(vec![position[0] - 1, position[1] + 2])
                                }
                                _ => {}
                            }
                        }
                        2 => {
                            match s.board.board[position[0] as usize - 1][position[1] as usize + 2] {
                                0 | 7 | 8 | 9 | 10 | 11 | 12 => {
                                    positions.push(vec![position[0] - 1, position[1] + 2])
                                }
                                _ => {}
                            }
                        }
                        _ => {}
                    }

                }

            }

            // Down 2 positions

            if position[0] + 2 <= 7 {

                // Left

                if (position[1] + 10) - 1 >= 10 {

                    match piece {
                        8 => {
                            match s.board.board[position[0] as usize + 2][position[1] as usize - 1] {
                                0 | 1 | 2 | 3 | 4 | 5 | 6 => {
                                    positions.push(vec![position[0] + 2, position[1] - 1])
                                }
                                _ => {}
                            }
                        }
                        2 => {
                            match s.board.board[position[0] as usize + 2][position[1] as usize - 1] {
                                0 | 7 | 8 | 9 | 10 | 11 | 12 => {
                                    positions.push(vec![position[0] + 2, position[1] - 1])
                                }
                                _ => {}
                            }
                        }
                        _ => {}
                    }

                }

                // Right

                if position[1] + 1 <= 7 {

                    match piece {
                        8 => {
                            match s.board.board[position[0] as usize + 2][position[1] as usize + 1] {
                                0 | 1 | 2 | 3 | 4 | 5 | 6 => {
                                    positions.push(vec![position[0] + 2, position[1] + 1])
                                }
                                _ => {}
                            }
                        }
                        2 => {
                            match s.board.board[position[0] as usize + 2][position[1] as usize + 1] {
                                0 | 7 | 8 | 9 | 10 | 11 | 12 => {
                                    positions.push(vec![position[0] + 2, position[1] + 1])
                                }
                                _ => {}
                            }
                        }
                        _ => {}
                    }

                }

            }

            // Down 1 position

            if position[0] + 1 <= 7 {

                // Left

                if (position[1] + 10) - 2 >= 10 {

                    match piece {
                        8 => {
                            match s.board.board[position[0] as usize + 1][position[1] as usize - 2] {
                                0 | 1 | 2 | 3 | 4 | 5 | 6 => {
                                    positions.push(vec![position[0] + 1, position[1] - 2])
                                }
                                _ => {}
                            }
                        }
                        2 => {
                            match s.board.board[position[0] as usize + 1][position[1] as usize - 2] {
                                0 | 7 | 8 | 9 | 10 | 11 | 12 => {
                                    positions.push(vec![position[0] + 1, position[1] - 2])
                                }
                                _ => {}
                            }
                        }
                        _ => {}
                    }

                }

                // Right

                if position[1] + 2 <= 7 {

                    match piece {
                        8 => {
                            match s.board.board[position[0] as usize + 1][position[1] as usize + 2] {
                                0 | 1 | 2 | 3 | 4 | 5 | 6 => {
                                    positions.push(vec![position[0] + 1, position[1] + 2])
                                }
                                _ => {}
                            }
                        }
                        2 => {
                            match s.board.board[position[0] as usize + 1][position[1] as usize + 2] {
                                0 | 7 | 8 | 9 | 10 | 11 | 12 => {
                                    positions.push(vec![position[0] + 1, position[1] + 2])
                                }
                                _ => {}
                            }
                        }
                        _ => {}
                    }

                }

            }

        }

        // Bishops

        9 | 3 => {
            let mut token = 1;

            // Up-Left

            let mut up_left_token = true;

            while up_left_token == true {
                if (position[0] + 10) - token >= 10 && (position[1] + 10) - token >= 10 {
                    match piece {
                        9 => {
                            match s.board.board[position[0] as usize - token as usize][position[1] as usize - token as usize] {
                                1 | 2 | 3 | 4 | 5 | 6 => {
                                    positions.push(vec![position[0] - token, position[1] - token]);
                                    up_left_token = false
                                }
                                7 | 8 | 9 | 10 | 11 | 12 => {
                                    up_left_token = false
                                }
                                _ => {
                                    positions.push(vec![position[0] - token, position[1] - token]);
                                    //println!("{:?}", positions);
                                    token += 1;
                                }
                            }
                        }
                        3 => {
                            match s.board.board[position[0] as usize - token as usize][position[1] as usize - token as usize] {
                                7 | 8 | 9 | 10 | 11 | 12 => {
                                    positions.push(vec![position[0] - token, position[1] - token]);
                                    up_left_token = false
                                }
                                1 | 2 | 3 | 4 | 5 | 6 => {
                                    up_left_token = false
                                }
                                _ => {
                                    positions.push(vec![position[0] - token, position[1] - token]);
                                    token += 1;
                                }
                            }
                        }
                        _ => {}
                    }
                } else {
                    up_left_token = false;
                }
            }

            // Up-Right

            token = 1;

            let mut up_right_token = true;

            while up_right_token == true {
                if (position[0] + 10) - token >= 10 && position[1] + token <= 7 {
                    match piece {
                        9 => {
                            match s.board.board[position[0] as usize - token as usize ][position[1] as usize + token as usize ] {
                                1 | 2 | 3 | 4 | 5 | 6 => {
                                    positions.push(vec![position[0] - token, position[1] + token]);
                                    up_right_token = false
                                }
                                7 | 8 | 9 | 10 | 11 | 12 => {
                                    up_right_token = false
                                }
                                _ => {
                                    positions.push(vec![position[0] - token, position[1] + token]);
                                    token += 1;
                                }
                            }
                        }
                        3 => {
                            match s.board.board[position[0] as usize - token as usize ][position[1] as usize + token as usize ] {
                                7 | 8 | 9 | 10 | 11 | 12 => {
                                    positions.push(vec![position[0] - token, position[1] + token]);
                                    up_right_token = false
                                }
                                1 | 2 | 3 | 4 | 5 | 6 => {
                                    up_right_token = false
                                }
                                _ => {
                                    positions.push(vec![position[0] - token, position[1] + token]);
                                    token += 1;
                                }
                            }
                        }
                        _ => {}
                    }
                } else {
                    up_right_token = false;
                }
            }

            token = 1;

            // Down-Left

            let mut down_left_token = true;

            while down_left_token == true {
                if position[0] + token <= 7 && (position[1] + 10) - token >= 10 {
                    match piece {
                        9 => {
                            match s.board.board[position[0] as usize + token as usize][position[1] as usize - token as usize] {
                                1 | 2 | 3 | 4 | 5 | 6 => {
                                    positions.push(vec![position[0] + token, position[1] - token]);
                                    down_left_token = false
                                }
                                7 | 8 | 9 | 10 | 11 | 12 => {
                                    down_left_token = false
                                }
                                _ => {
                                    positions.push(vec![position[0] + token, position[1] - token]);
                                    //println!("{:?}", positions);
                                    token += 1;
                                }
                            }
                        }
                        3 => {
                            match s.board.board[position[0] as usize + token as usize][position[1] as usize - token as usize] {
                                7 | 8 | 9 | 10 | 11 | 12 => {
                                    positions.push(vec![position[0] + token, position[1] - token]);
                                    down_left_token = false
                                }
                                1 | 2 | 3 | 4 | 5 | 6 => {
                                    down_left_token = false
                                }
                                _ => {
                                    positions.push(vec![position[0] + token, position[1] - token]);
                                    token += 1;
                                }
                            }
                        }
                        _ => {}
                    }
                } else {
                    down_left_token = false;
                }
            }

            // Down-Right

            token = 1;

            let mut down_right_token = true;

            while down_right_token == true {
                if position[0] + token <= 7 && position[1] + token <= 7 {
                    match piece {
                        9 => {
                            match s.board.board[position[0] as usize + token as usize ][position[1] as usize + token as usize ] {
                                1 | 2 | 3 | 4 | 5 | 6 => {
                                    positions.push(vec![position[0] + token, position[1] + token]);
                                    down_right_token = false
                                }
                                7 | 8 | 9 | 10 | 11 | 12 => {
                                    down_right_token = false
                                }
                                _ => {
                                    positions.push(vec![position[0] + token, position[1] + token]);
                                    token += 1;
                                }
                            }
                        }
                        3 => {
                            match s.board.board[position[0] as usize + token as usize ][position[1] as usize + token as usize ] {
                                7 | 8 | 9 | 10 | 11 | 12 => {
                                    positions.push(vec![position[0] + token, position[1] + token]);
                                    down_right_token = false
                                }
                                1 | 2 | 3 | 4 | 5 | 6 => {
                                    down_right_token = false
                                }
                                _ => {
                                    positions.push(vec![position[0] + token, position[1] + token]);
                                    token += 1;
                                }
                            }
                        }
                        _ => {}
                    }
                } else {
                    down_right_token = false;
                }
            }

        }

        // Queens

        10 | 4 => {

            let bishop_numeric_value: usize;
            let tower_numeric_value: usize;

            match piece {
                10 => {
                    bishop_numeric_value = 9;
                    tower_numeric_value = 7;
                }
                4 => {
                    bishop_numeric_value = 3;
                    tower_numeric_value = 1;
                }

                _ => {
                    bishop_numeric_value = 0;
                    tower_numeric_value = 0;
                }
            }

            // Straights

            let mut straights = get_possible_moves(position.clone(), s, tower_numeric_value.clone() as i32);

            // Diagonals

            let mut diagonals = get_possible_moves(position.clone(), s, bishop_numeric_value.clone() as i32);

            // Concatenating

            for p in straights {
                positions.push(p);
            }

            for p in diagonals {
                positions.push(p);
            }


        }

        // Kings

        11 | 5 => {

            // Up

            if (position[0] + 10) - 1 >= 10 {

                // Straight

                match piece {
                    11 => {
                        match s.board.board[position[0] as usize - 1][position[1] as usize] {
                            0 | 1 | 2 | 3 | 4 | 5 | 6 => {
                                positions.push(vec![position[0] - 1, position[1]])
                            }
                            _ => {}
                        }
                    }

                    5 => {
                        match s.board.board[position[0] as usize - 1][position[1] as usize] {
                            0 | 7 | 8 | 9 | 10 | 11 | 12 => {
                                positions.push(vec![position[0] - 1, position[1]])
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }

                // Left

                if (position[1] + 10) - 1 >= 10 {
                    match piece {
                        11 => {
                            match s.board.board[position[0] as usize - 1][position[1] as usize - 1] {
                                0 | 1 | 2 | 3 | 4 | 5 | 6 => {
                                    positions.push(vec![position[0] - 1, position[1] - 1])
                                }
                                _ => {}
                            }
                        }

                        5 => {
                            match s.board.board[position[0] as usize - 1][position[1] as usize - 1] {
                                0 | 7 | 8 | 9 | 10 | 11 | 12 => {
                                    positions.push(vec![position[0] - 1, position[1] - 1])
                                }
                                _ => {}
                            }
                        }
                        _ => {}
                    }
                }

                // Right

                if position[1] + 1 <= 7 {
                    match piece {
                        11 => {
                            match s.board.board[position[0] as usize - 1][position[1] as usize + 1] {
                                0 | 1 | 2 | 3 | 4 | 5 | 6 => {
                                    positions.push(vec![position[0] - 1, position[1] + 1])
                                }
                                _ => {}
                            }
                        }

                        5 => {
                            match s.board.board[position[0] as usize - 1][position[1] as usize + 1] {
                                0 | 7 | 8 | 9 | 10 | 11 | 12 => {
                                    positions.push(vec![position[0] - 1, position[1] + 1])
                                }
                                _ => {}
                            }
                        }
                        _ => {}
                    }
                }

            }

            // Down

            if position[0] + 1 <= 7 {

                // Straight

                match piece {
                    11 => {
                        match s.board.board[position[0] as usize + 1][position[1] as usize] {
                            0 | 1 | 2 | 3 | 4 | 5 | 6 => {
                                positions.push(vec![position[0] + 1, position[1]])
                            }
                            _ => {}
                        }
                    }

                    5 => {
                        match s.board.board[position[0] as usize + 1][position[1] as usize] {
                            0 | 7 | 8 | 9 | 10 | 11 | 12 => {
                                positions.push(vec![position[0] + 1, position[1]])
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }

                // Left

                if (position[1] + 10) - 1 >= 10 {
                    match piece {
                        11 => {
                            match s.board.board[position[0] as usize + 1][position[1] as usize - 1] {
                                0 | 1 | 2 | 3 | 4 | 5 | 6 => {
                                    positions.push(vec![position[0] + 1, position[1] - 1])
                                }
                                _ => {}
                            }
                        }

                        5 => {
                            match s.board.board[position[0] as usize + 1][position[1] as usize - 1] {
                                0 | 7 | 8 | 9 | 10 | 11 | 12 => {
                                    positions.push(vec![position[0] + 1, position[1] - 1])
                                }
                                _ => {}
                            }
                        }
                        _ => {}
                    }
                }

                // Right

                if position[1] + 1 <= 7 {
                    match piece {
                        11 => {
                            match s.board.board[position[0] as usize + 1][position[1] as usize + 1] {
                                0 | 1 | 2 | 3 | 4 | 5 | 6 => {
                                    positions.push(vec![position[0] + 1, position[1] + 1])
                                }
                                _ => {}
                            }
                        }

                        5 => {
                            match s.board.board[position[0] as usize + 1][position[1] as usize + 1] {
                                0 | 7 | 8 | 9 | 10 | 11 | 12 => {
                                    positions.push(vec![position[0] + 1, position[1] + 1])
                                }
                                _ => {}
                            }
                        }
                        _ => {}
                    }
                }

            }

            // Left

            if (position[1] + 10) - 1 >= 10 {
                match piece {
                    11 => {
                        match s.board.board[position[0] as usize][position[1] as usize - 1] {
                            0 | 1 | 2 | 3 | 4 | 5 | 6 => {
                                positions.push(vec![position[0], position[1] - 1])
                            }
                            _ => {}
                        }
                    }

                    5 => {
                        match s.board.board[position[0] as usize][position[1] as usize - 1] {
                            0 | 7 | 8 | 9 | 10 | 11 | 12 => {
                                positions.push(vec![position[0], position[1] - 1])
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }

            // Right

            if position[1] + 1 <= 7 {
                match piece {
                    11 => {
                        match s.board.board[position[0] as usize][position[1] as usize + 1] {
                            0 | 1 | 2 | 3 | 4 | 5 | 6 => {
                                positions.push(vec![position[0], position[1] + 1])
                            }
                            _ => {}
                        }
                    }

                    5 => {
                        match s.board.board[position[0] as usize][position[1] as usize+ 1] {
                            0 | 7 | 8 | 9 | 10 | 11 | 12 => {
                                positions.push(vec![position[0], position[1] + 1])
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }

        }

        _ => {}
    }

    return positions;
}

fn can_move(s: &mut Game, positions: Vec<i32>) -> bool {

    let piece = s.board.board[positions[0] as usize][positions[1] as usize];

    // if is the correct turn

    match s.round {
        Round::White => {
            match piece {
                7 | 8 | 9 | 10 | 11 | 12 => {}
                _ => {
                    return false
                }
            }
        }
        Round::Black => {
            match piece {
                1 | 2 | 3 | 4 | 5 | 6 => {}
                _ => {
                    return false
                }
            }
        }
    }

    // special exception if there is castling

    if is_castling(s, positions.clone()) {
        return true
    }

    let possible_moves = get_possible_moves(positions.clone(), s, piece as i32);

    let future_position = vec![positions[2], positions[3]];

    if possible_moves.contains(&future_position) {
        return true
    }

    return false
}

fn is_castling(s: &mut Game, positions: Vec<i32>) -> bool {

    match s.board.board[positions[0] as usize][positions[1] as usize] {

        // white king

        11 => {

            // check if white king is in original position

            if s.board.white_original_position_checkers[0] == false {
                return false
            }

            // if the future position is a corner piece

            match positions[3] {

                0 => {

                    // check if corner piece is a castle

                    if s.board.board[positions[2] as usize][positions[3] as usize] != 7 {
                        return false
                    }

                    // if between king and castle is empty

                    for x in 1..4 {
                        if s.board.board[7][x] != 0 {
                            return false
                        }
                    }

                    // if castle hasn't moved

                    if s.board.white_original_position_checkers[1] == true {
                        return true
                    }
                }

                7 => {

                    // check if corner piece is a castle

                    if s.board.board[positions[2] as usize][positions[3] as usize] != 7 {
                        return false
                    }

                    // if between king and castle is empty

                    for x in 5..7 {
                        if s.board.board[7][x] != 0 {
                            return false
                        }
                    }

                    // if castle hasn't moved

                    if s.board.white_original_position_checkers[2] == true {
                        return true
                    }
                }

                _ => {
                    return false
                }
            }

        }

        // black king

        5 => {
            // check if white king is in original position

            if s.board.black_original_position_checkers[0] == false {
                return false
            }

            // if the future position is a corner piece

            match positions[3] {

                0 => {

                    // check if corner piece is a castle

                    if s.board.board[positions[2] as usize][positions[3] as usize] != 1 {
                        return false
                    }

                    // if between king and castle is empty

                    for x in 1..4 {
                        if s.board.board[0][x] != 0 {
                            return false
                        }
                    }

                    // if castle hasn't moved

                    if s.board.black_original_position_checkers[1] == true {
                        return true
                    }
                }

                7 => {

                    // check if corner piece is a castle

                    if s.board.board[positions[2] as usize][positions[3] as usize] != 1 {
                        return false
                    }

                    // if between king and castle is empty

                    for x in 5..7 {
                        if s.board.board[0][x] != 0 {
                            return false
                        }
                    }

                    // if castle hasn't moved

                    if s.board.black_original_position_checkers[2] == true {
                        return true
                    }
                }

                _ => {
                    return false
                }
            }
        }

        _ => {
            return false
        }

    }

    return false

}

fn update_castles_king(s: &mut Game) {

    // white checkers

    // white king

    if s.board.board[7][4] != 11 {
        s.board.white_original_position_checkers[0] = false;
    }

    // left white rook

    if s.board.board[7][0] != 7 {
        s.board.white_original_position_checkers[1] = false;
    }

    // right white rook

    if s.board.board[7][7] != 7 {
        s.board.white_original_position_checkers[2] = false;
    }

    // black checkers

    // black king

    if s.board.board[0][4] != 5 {
        s.board.black_original_position_checkers[0] = false;
    }

    // left black castle

    if s.board.board[0][0] != 1 {
        s.board.black_original_position_checkers[1] = false;
    }

    // right black castle

    if s.board.board[0][7] != 1 {
        s.board.black_original_position_checkers[2] = false;
    }

}