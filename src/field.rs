use rand::seq::SliceRandom;
use rand::Rng;

pub use container::FieldContent;

pub struct Coords {
    pub x: u8,
    pub y: u8
}

#[derive(Copy, Clone)]
pub struct Cell {
    pub content: FieldContent,
    pub fixed: bool
}

#[derive(Copy, Clone)]
pub struct Field {
    pub cells: [[Cell; 9]; 9]
}

impl Field {
    pub fn new() -> Field {
        let mut field = Field {
            cells: [[Cell{ content: FieldContent::Hints([true; 9]), fixed: false }; 9]; 9]
        };
        field.fill_random();
        field
    }

    pub fn get_cell(&self, x: u8, y: u8) -> &Cell {
        &self.cells[y as usize][x as usize]
    }

    pub fn get_cell_mut(&mut self, x: u8, y: u8) -> &mut Cell {
        &mut self.cells[y as usize][x as usize]
    }

    pub fn set_value(&mut self, x: u8, y: u8, value: u8) {
        for i in 0..9 {
            if let FieldContent::Hints(ref mut hints) = self.get_cell_mut(i, y).content {
                hints[(value - 1) as usize] = false;
            }
            if let FieldContent::Hints(ref mut hints) = self.get_cell_mut(x, i).content {
                hints[(value - 1) as usize] = false;
            }
            if let FieldContent::Hints(ref mut hints) = self.get_cell_mut((x / 3) * 3 + i % 3, (y / 3) * 3 + (i / 3)).content {
                hints[(value - 1) as usize] = false;
            }
        }
        self.cells[y as usize][x as usize].content = FieldContent::Digit(value)
    }

    pub fn set_hints(&mut self, x: u8, y: u8) {
        let c = Coords{ x: x, y: y };
        let mut hints = [true; 9];
        for i in 0..9 {
            if let Some(_) = self.find_conflict(&c, i + 1) {
                hints[i as usize] = false;
            }
        }
        self.set_content(x, y, FieldContent::Hints(hints));
    }

    pub fn set_content(&mut self, x: u8, y: u8, value: FieldContent) {
        self.cells[y as usize][x as usize].content = value
    }

    pub fn find_conflict(&self, coords: &Coords, digit: u8) -> Option<Coords> {
        for x in 0..9 {
            if x != coords.x {
                if let FieldContent::Digit(cell_digit) = self.get_cell(x, coords.y).content {
                    if cell_digit == digit {
                        return Some(Coords{ x: x, y: coords.y});
                    }
                }
            }
        }

        for y in 0..9 {
            if y != coords.y {
                if let FieldContent::Digit(cell_digit) = self.get_cell(coords.x, y).content {
                    if cell_digit == digit {
                        return Some(Coords{ x: coords.x, y: y});
                    }
                }
            }
        }

        let section = Coords{ x: coords.x / 3, y: coords.y / 3};
        for x in section.x * 3 .. (section.x + 1) * 3 {
            for y in section.y * 3 .. (section.y + 1) * 3 {
                if x != coords.x || y != coords.y {
                    if let FieldContent::Digit(cell_digit) = self.get_cell(x, y).content {
                        if cell_digit == digit {
                            return Some(Coords{ x: x, y: y});
                        }
                    }
                }
            }
        }

        None
    }

    pub fn clear(&mut self) {
        for y in 0..9 {
            for x in 0..9 {
                self.cells[x][y] = Cell{ content: FieldContent::Hints([true; 9]), fixed: false };
            }
        }
    }

    pub fn fill_random(&mut self) {
        self.clear();

        let x = rand::thread_rng().gen_range(0u8, 9u8);
        let y = rand::thread_rng().gen_range(0u8, 9u8);
        let digit = rand::thread_rng().gen_range(1u8, 10u8);
        self.set_value(x, y, digit);

        let solution = self.find_solution().unwrap();
        self.cells = solution.cells;

        loop {
            let mut x;
            let mut y;
            let digit;

            loop {
                x = rand::thread_rng().gen_range(0u8, 9u8);
                y = rand::thread_rng().gen_range(0u8, 9u8);
                if self.get_cell(x, y).content.is_none() {
                    continue;
                }
                digit = self.get_cell(x, y).content.unwrap();
                self.set_content(x, y, FieldContent::None);
                break;
            }

            let solutions = self.find_solutions(2);
            if solutions.len() == 1 {
                continue;
            }
            self.set_value(x, y, digit);

            break;
        }

        // FIXME(xairy): generates perfect sudoku, but slow.
        /*
        let mut cells = Vec::new();
        for y in 0..9 {
            for x in 0..9 {
                cells.push((x, y));
            }
        }
        rand::thread_rng().shuffle(&mut cells);

        for &(x, y) in cells.iter() {
            let digit = self.get_cell(x, y).digit.unwrap();
            self.get_cell(x, y).digit = None;
            let solutions = self.find_solutions(2);
            if solutions.len() > 1 {
                self.get_cell(x, y).digit = Some(digit);
            }
        }
        */


        for y in 0..9 {
            for x in 0..9 {
                if self.get_cell(x, y).content.is_some() {
                    self.get_cell_mut(x, y).fixed = true;
                } else {
                    self.set_hints(x, y);
                }
            }
        }
    }

    pub fn fill_solution(&mut self) {
        if let Some(s) = self.find_solution() {
            self.cells = s.cells;
        }
    }

    pub fn find_solution(&mut self) -> Option<Field> {
        let solutions = self.find_solutions(1);
        if solutions.len() > 0 {
            return Some(solutions[0]);
        }
        None
    }

    fn find_solutions(&mut self, stop_at: u32) -> Vec<Field> {
        let mut solutions = Vec::new();
        let mut field = self.clone();
        field.find_solutions_impl(&mut solutions, stop_at);
        solutions
    }

    fn find_solutions_impl(&mut self, solutions: &mut Vec<Field>,
                           stop_at: u32) -> bool {
        let mut empty_cell: Option<Coords> = None;
        'outer: for y in 0..9 {
            'inner: for x in 0..9 {
                if self.get_cell(x, y).content.is_none() {
                    empty_cell = Some(Coords{ x: x, y: y });
                    break 'outer;
                }
            }
        }

        if empty_cell.is_none() {
            solutions.push(self.clone());
            return solutions.len() >= (stop_at as usize);
        }
        let coords = empty_cell.unwrap();

        let mut digits: Vec<u8> = (1..10).collect();

        let mut rng = rand::thread_rng();
        digits.shuffle(&mut rng);

        for &digit in digits.iter() {
            if self.find_conflict(&coords, digit).is_none() { 
                self.set_value(coords.x, coords.y, digit);
                if self.find_solutions_impl(solutions, stop_at) {
                    return true;
                }
                self.set_content(coords.x, coords.y, FieldContent::None);
            }
        }

        return false;
    }
}
