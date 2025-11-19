use crate::utilities::grid_point::GridPoint;
use std::collections::VecDeque;

pub struct SnakeBody {
    pub(crate) body: VecDeque<(GridPoint)>,
}

impl SnakeBody {
    pub fn new() -> Self {
        let mut body = VecDeque::<GridPoint>::new();
        body.push_back(GridPoint { x: 3, y: 5 });
        body.push_back(GridPoint { x: 4, y: 5 });
        body.push_back(GridPoint { x: 5, y: 5 });
        body.push_back(GridPoint { x: 6, y: 5 });

        Self { body }
    }

    pub fn head(&self) -> &GridPoint {
        self.body.front().unwrap()
    }

    pub fn move_forward(&mut self, new_head: GridPoint) {
        self.body.push_front(new_head);
        self.body.pop_back();
    }
}
