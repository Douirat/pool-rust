pub fn tic_tac_toe(table: [[char; 3]; 3]) -> String {
    if horizontal('X', table) || vertical('X', table) || diagonals('X', table) {
        return "player X won".to_string();
    }
    if horizontal('O', table) || vertical('O', table) || diagonals('O', table) {
        return "player O won".to_string();
    }
    "tie".to_string()
}

pub fn horizontal(player: char, table: [[char; 3]; 3]) -> bool {
    for row in table.iter() {
        if row[0] == player && row[0] == row[1] && row[1] == row[2] {
            return true;
        }
    }
    false
}

pub fn vertical(player: char, table: [[char; 3]; 3]) -> bool {
    for col in 0..3 {
        if table[0][col] == player && table[0][col] == table[1][col] && table[1][col] == table[2][col] {
            return true;
        }
    }
    false
}

pub fn diagonals(player: char, table: [[char; 3]; 3]) -> bool {
    // top-left to bottom-right diagonal
    if table[0][0] == player && table[0][0] == table[1][1] && table[1][1] == table[2][2] {
        return true;
    }
    // top-right to bottom-left diagonal
    if table[0][2] == player && table[0][2] == table[1][1] && table[1][1] == table[2][0] {
        return true;
    }
    false
}