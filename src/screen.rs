/*
    Used to draw to screen
*/

struct Screen {
    width: usize,
    height: usize,
}

impl Screen {
    // creates a new screen from width and height
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
        }
    }

    // draws an array of bools as blocks to screen based on width and size
    pub fn draw(&self, blocks: &[bool]) {
        // making sure enough blocks for screen
        assert_eq!(blocks.len(), self.width * self.height);
    }
}