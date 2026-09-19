mod cell;
mod grid;
mod instance;
mod symbols;

use std::error;

use instance::InstanceCount;
pub(crate) use instance::{Instance, InstanceId, InstanceSet};

use cell::Cell;
use grid::{Grid, GridIndex};

use crate::Point;

pub struct Stage {
    instances: Vec<Instance>,
    neighbors_list: Vec<InstanceSet>,
    start_neighbors: InstanceSet,
    goal: Instance,
    count: InstanceCount,
    width: usize,
}

impl Stage {
    pub fn parse(stage_json: &str) -> Result<Self, Box<dyn error::Error>> {
        let grid = Grid::parse(stage_json)?;

        let mut id = InstanceId::ZERO;
        let mut instances = Vec::new();
        let mut start_index = None;
        let mut goal = None;

        for (index, cell) in grid.iter().enumerate() {
            match cell {
                Cell::Wall | Cell::Road => {}
                Cell::Start => {
                    start_index = Some(index.into());
                }
                Cell::Object(object) => {
                    let instance = Instance::new(id, index, object);
                    if instance.is_goal() {
                        goal = Some(instance.clone());
                    }
                    instances.push(instance);
                    id = id.succ();
                }
            }
        }

        let start_index = start_index.ok_or("startが無い")?;
        let goal = goal.ok_or("goalが無い")?;

        let count = instances.len().try_into()?;

        let mut instance_at = vec![None; grid.len()];
        for instance in &instances {
            instance_at[instance.index.as_usize()] = Some(instance);
        }

        let neighbors_at = |index: GridIndex| {
            let mut neighbors = InstanceSet::new(count);
            for index in grid.reachables(index) {
                #[expect(clippy::missing_panics_doc)]
                let neighbor =
                    instance_at[index.as_usize()].expect("reachablesはObjectのindexしか返さない");
                neighbors.insert(neighbor);
            }
            neighbors
        };

        let neighbors_list = instances
            .iter()
            .map(|instance| neighbors_at(instance.index))
            .collect();

        let start_neighbors = neighbors_at(start_index);

        Ok(Self {
            instances,
            neighbors_list,
            start_neighbors,
            goal,
            count,
            width: grid.width(),
        })
    }

    pub(crate) fn start_neighbors(&self) -> &InstanceSet {
        &self.start_neighbors
    }

    pub(crate) fn empty_instance_set(&self) -> InstanceSet {
        InstanceSet::new(self.count)
    }

    pub(crate) fn goal(&self) -> &Instance {
        &self.goal
    }

    pub(crate) fn instance_of(&self, instance_id: InstanceId) -> &Instance {
        &self.instances[instance_id.as_usize()]
    }

    pub(crate) fn neighbors_of(&self, instance_id: InstanceId) -> &InstanceSet {
        &self.neighbors_list[instance_id.as_usize()]
    }

    pub(crate) fn point_of(&self, instance: &Instance) -> Point {
        instance.index.as_point(self.width)
    }
}
