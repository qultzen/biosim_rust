#[derive(Clone, Debug, PartialEq)]
pub struct Cell {
    name: &'static str,
    fauna: Option<()>,
    f_max: u32,
}

pub const WATER: Cell = Cell {
    name: "WATER",
    fauna: None,
    f_max: 0,
};
pub const DESERT: Cell = Cell {
    name: "DESSERT",
    fauna: None,
    f_max: 0,
};
pub const LOWLAND: Cell = Cell {
    name: "LOWLAND",
    fauna: None,
    f_max: 800,
};
pub const HIGHLAND: Cell = Cell {
    name: "HIGHLAND",
    fauna: None,
    f_max: 300,
};

pub fn from_char(c: char) -> Cell {
    match c {
        'W' => WATER,
        'D' => DESERT,
        'L' => LOWLAND,
        'H' => HIGHLAND,
        _ => panic!(),
    }
}
