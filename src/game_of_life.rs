use raylib::prelude::*;
use crate::framebuffer::Framebuffer;
use crate::patterns::*;

pub struct GameOfLife {
    pub width: u32,
    pub height: u32,
    current_state: Vec<Vec<bool>>,
    next_state: Vec<Vec<bool>>,
}

impl GameOfLife {
    pub fn new(width: u32, height: u32) -> Self {
        let current_state = vec![vec![false; width as usize]; height as usize];
        let next_state = vec![vec![false; width as usize]; height as usize];
        
        GameOfLife {
            width,
            height,
            current_state,
            next_state,
        }
    }

    pub fn set_cell(&mut self, x: u32, y: u32, alive: bool) {
        if x < self.width && y < self.height {
            self.current_state[y as usize][x as usize] = alive;
        }
    }

    pub fn is_alive(&self, x: u32, y: u32) -> bool {
        if x < self.width && y < self.height {
            self.current_state[y as usize][x as usize]
        } else {
            false
        }
    }

    fn count_neighbors(&self, x: u32, y: u32) -> u8 {
        let mut count = 0;
        
        for dy in -1..=1 {
            for dx in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;
                
                if nx >= 0 && nx < self.width as i32 && ny >= 0 && ny < self.height as i32 {
                    if self.current_state[ny as usize][nx as usize] {
                        count += 1;
                    }
                }
            }
        }
        
        count
    }

    pub fn update(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let neighbors = self.count_neighbors(x, y);
                let alive = self.is_alive(x, y);
                
                self.next_state[y as usize][x as usize] = match (alive, neighbors) {
                    // (underpopulation)
                    (true, n) if n < 2 => false,
                    // (survival)
                    (true, 2) | (true, 3) => true,
                    // (overpopulation)
                    (true, n) if n > 3 => false,
                    // (reproduction)
                    (state, _) => state,
                };
            }
        }
        
        std::mem::swap(&mut self.current_state, &mut self.next_state);
    }

    pub fn render(&self, framebuffer: &mut Framebuffer) {
        for y in 0..self.height {
            for x in 0..self.width {
                let color = if self.is_alive(x, y) {
                    Color::WHITE 
                } else {
                    Color::BLACK 
                };
                framebuffer.set_pixel_with_color(x, y, color);
            }
        }
    }

    pub fn initialize_with_pattern(&mut self, pattern: Pattern, offset_x: u32, offset_y: u32) {
        let coords = get_pattern_coordinates(pattern);
        for (x, y) in coords {
            self.set_cell(offset_x + x, offset_y + y, true);
        }
    }
}