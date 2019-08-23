use super::boid::{Boid, VIEW_RADIUS};
use wasm_bindgen::__rt::std::collections::HashMap;
use vector2d::Vector2D;

const SQUARED_VIEW_RADIUS: f64 = VIEW_RADIUS * VIEW_RADIUS;

pub struct Lattice {
    x_cells: i32,
    y_cells: i32,
    boids: HashMap<i32, Vec<Boid>>
}

impl Lattice {
    pub fn new(boids: &Vec<Boid>, width: &f64, height: &f64) -> Lattice {
        let mut lattice: HashMap<i32, Vec<Boid>> = HashMap::new();
        let x_cells = (*width / VIEW_RADIUS * 2.0).ceil() as i32;
        let y_cells = (*height / VIEW_RADIUS * 2.0).ceil() as i32;

        boids.iter()
            .for_each(|boid| {
                let index = Self::convert_position_to_lattice_index(&boid.position, &x_cells, &y_cells);

                if lattice.contains_key(&index) {
                    lattice.get_mut(&index).unwrap().push(boid.clone());
                } else {
                    lattice.insert(index, vec![boid.clone()]);
                }
            });

        Lattice {
            x_cells,
            y_cells,
            boids: lattice
        }
    }

    pub fn get_neighbors(&self, boid: &Boid) -> Vec<&Boid> {
        let index = Self::convert_position_to_lattice_index(&boid.position, &self.x_cells, &self.y_cells);
        let neighbor_indexes = self.calculate_neighbor_indexes(&index);

        let filter_by_distance = |b: &&Boid| (b.position - boid.position).length_squared() <= SQUARED_VIEW_RADIUS;

        let self_cell_boids: Vec<&Boid> = self.boids[&index].iter()
            .filter(|b| boid.position != b.position)
            .filter(filter_by_distance)
            .collect();

        let mut neighbor_boids: Vec<&Boid> = neighbor_indexes.iter()
            .filter_map(|i| self.boids.get(i))
            .flatten()
            .filter(filter_by_distance)
            .collect();

        neighbor_boids.extend(self_cell_boids.into_iter());

        neighbor_boids
    }

    fn calculate_neighbor_indexes(&self, index: &i32) -> Vec<i32> {
        let calculate_left_and_right = |num: &i32| [*num - 1, *num + 1]
            .iter()
            .map(|i| {
                let initial_y_cell = *num / self.x_cells;
                let y_cell = *i / self.x_cells;
                if y_cell < initial_y_cell  {
                    *num + self.x_cells - 1
                } else if y_cell > initial_y_cell {
                    *num - self.x_cells + 1
                } else {
                    *i
                }
            })
            .collect::<Vec<i32>>();

        let top_and_bottom = [*index - self.x_cells, *index + self.x_cells]
            .iter()
            .map(|i| {
                if *i < 0 {
                    *index + self.x_cells * (self.y_cells - 1)
                } else if *i > self.x_cells * self.y_cells - 1 {
                    *index - self.x_cells * (self.y_cells - 1)
                } else {
                    *i
                }
            })
            .collect::<Vec<i32>>();

        let diagonals = top_and_bottom
            .iter()
            .flat_map(calculate_left_and_right)
            .collect::<Vec<i32>>();

        let mut indexes = calculate_left_and_right(index);
        indexes.extend(top_and_bottom.iter());
        indexes.extend(diagonals.iter());
        indexes
    }

    fn convert_position_to_lattice_index(position: &Vector2D<f64>, x_cells: &i32, y_cells: &i32) -> i32 {
        let Vector2D {x, y} = position;
        let x_cell = (*x / *x_cells as f64).floor();
        let y_cell = (*y / *y_cells as f64).floor();
        (y_cell * (*x_cells as f64) + x_cell) as i32
    }
}