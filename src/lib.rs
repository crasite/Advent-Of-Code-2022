pub trait Grid {
    type Item;
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn get_at_index(&self, index: usize) -> Option<Self::Item>;
    fn get_from_coordinate(&self, x: isize, y: isize) -> Option<Self::Item> {
        if x < 0 || y < 0 {
            None
        } else {
            self.get_at_index(y as usize * self.width() + x as usize)
        }
    }
    fn get_from_index(&self, index: usize) -> Option<Self::Item> {
        self.get_at_index(index)
    }
}
