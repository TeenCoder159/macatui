pub trait Element {
    fn draw(&self) {}
}

pub struct Viewport {
    pub viewport_width: u32,
    pub viewport_height: u32,
    pub body: Container,
}

pub enum Direction {
    Row,
    Column,
}

pub struct Container {
    direction: Direction,
    elements: Vec<Box<dyn Element>>,
}

impl Container {
    /// Create a new container that is a row
    pub fn new_row() -> Self {
        Self {
            direction: Direction::Row,
            elements: vec![],
        }
    }
    pub fn new_column() -> Self {
        Self {
            direction: Direction::Column,
            elements: vec![],
        }
    }

    pub fn clear_container(&mut self) {
        self.elements = vec![];
    }

    pub fn toggle_direction(&mut self) {
        match self.direction {
            Direction::Column => self.direction = Direction::Row,
            Direction::Row => self.direction = Direction::Column,
        }
    }

    pub fn to_row(&mut self) {
        self.direction = Direction::Row;
    }

    pub fn to_column(&mut self) {
        self.direction = Direction::Column;
    }

    pub fn add_element(&mut self, element: impl Element + 'static) {
        self.elements.push(Box::new(element));
    }

    //    pub fn remove_specific_element<T: PartialEq<dyn Element>>(&mut self, element: T) {
    //        self.elements.retain(|x| element != **x);
    //    }
    pub fn remove_element(&mut self, element_index: u32) {
        self.elements.remove(element_index as usize);
    }
}
