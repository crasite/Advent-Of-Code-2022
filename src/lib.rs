use std::fmt::Display;

pub trait Grid {
    type Item: Display;
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn get_at_index(&self, index: usize) -> Option<Self::Item>;
    fn index_to_coord_from_width(index: usize, width: usize) -> (usize, usize) {
        let x = index % width;
        let y = index / width;
        (x, y)
    }
    fn index_to_coord(&self, index: usize) -> (usize, usize) {
        let width = self.width();
        let x = index % width;
        let y = index / width;
        (x, y)
    }
    fn get_from_coordinate(&self, x: isize, y: isize) -> Option<Self::Item> {
        if x < 0 || y < 0 || x >= self.width() as isize || y >= self.height() as isize {
            None
        } else {
            self.get_at_index(y as usize * self.width() + x as usize)
        }
    }
    fn get_from_index(&self, index: usize) -> Option<Self::Item> {
        self.get_at_index(index)
    }

    fn display_unit(&self, item: Self::Item) -> char;

    fn draw(&self) {
        for y in 0..self.height() {
            for x in 0..self.width() {
                print!(
                    "{}",
                    self.display_unit(self.get_from_coordinate(x as isize, y as isize).unwrap())
                );
            }
            println!();
        }
        println!();
    }
}
