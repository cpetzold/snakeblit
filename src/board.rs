use array2d::Array2D;
use bevy::{prelude::*, utils::HashMap};

pub struct Board {
    cells: Array2D<Option<Entity>>,
    pub positions: HashMap<Entity, UVec2>,
}

impl Board {
    pub fn new(rows: usize, columns: usize) -> Self {
        Self {
            cells: Array2D::filled_with(None, rows, columns),
            positions: HashMap::new(),
        }
    }

    pub fn set(&mut self, dest: UVec2, entity: Entity) -> Result<(), &str> {
        if matches!(
            self.cells.get(dest.y as usize, dest.x as usize),
            Some(Some(_))
        ) {
            return Err("Location already occupied");
        }

        if self
            .cells
            .set(dest.y as usize, dest.x as usize, Some(entity))
            .is_err()
        {
            return Err("Failed to set in 2d array");
        }

        self.positions.insert(entity, dest);
        Ok(())
    }

    pub fn remove(&mut self, entity: Entity) -> Result<UVec2, &str> {
        if let Some(pos) = self.positions.get(&entity) {
            if self
                .cells
                .set(pos.y as usize, pos.x as usize, None)
                .is_err()
            {
                self.positions.remove(&entity);
                return Err("Failed to set in 2d array");
            }
            Ok(*pos)
        } else {
            Err("Couldn't find element")
        }
    }

    pub fn move_item(&mut self, entity: Entity, offset: IVec2) -> Result<(), &str> {
        if let Ok(pos) = self.remove(entity) {
            self.set(
                (pos.as_ivec2() + offset).max(IVec2::ZERO).as_uvec2(),
                entity,
            )
        } else {
            Err("Couldn't find entity")
        }
    }
}
