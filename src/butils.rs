pub fn xy_to_position(x: usize, y: usize) -> u8 {
    match y {
        0 => match x {
            0 => 8,
            1 => 7,
            2 => 6,
            _ => panic!("WTF!"),
        },
        1 => match x {
            0 => 5,
            1 => 4,
            2 => 3,
            _ => panic!("WTF!"),
        },
        2 => match x {
            0 => 2,
            1 => 1,
            2 => 0,
            _ => panic!("WTF!"),
        },
        _ => panic!("WTF!"),
    }
}

pub fn identify_position_state(position: u8, board: u32) -> char {
    let state = (board & (0b11 << ((position as u32) << 1))) >> ((position as u32) << 1);
    match state {
        0b00 => ' ',
        0b01 => 'o',
        0b10 => 'x',
        _ => unreachable!("This state shouldn't exit!"),
    }
}

pub fn identify_position_player(position: u8, board: u32) -> bool {
    let state = (board & (0b11 << ((position as u32) << 1))) >> ((position as u32) << 1);
    match state {
        0b01 => false,
        0b10 => true,
        _ => unreachable!("This state shouldn't exit!"),
    }
}

pub fn get_empty_elements(board: u32) -> Vec<u8> {
    let mut empty_elements: Vec<u8> = Vec::new();
    for p in 0..9 {
        if identify_position_state(p, board) == ' ' {
            empty_elements.push(p);
        }
    }
    empty_elements
}
