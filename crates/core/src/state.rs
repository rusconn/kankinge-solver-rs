mod status;

use std::rc::Rc;

use crate::{
    ConvertKind,
    object::Object,
    stage::{Instance, InstanceId, InstanceSet, Stage},
};

pub use status::Status;
pub(crate) use status::StatusCmp;

#[derive(Debug, Clone)]
pub(crate) struct State {
    pub(crate) status: Status,
    pub(crate) erased: Rc<InstanceSet>,
    boundary: InstanceSet,
}

#[derive(Debug)]
pub(crate) struct Child {
    pub(crate) state: State,
    pub(crate) action: Action,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Action {
    Root,
    Move(InstanceId),
    Convert(ConvertKind),
}

impl State {
    pub(crate) fn initial(stage: &Stage) -> Self {
        Self {
            status: Status::initial(),
            erased: Rc::new(stage.empty_instance_set()),
            boundary: stage.start_neighbors().clone(),
        }
    }

    pub(crate) fn add_children(&self, buf: &mut Vec<Child>, stage: &Stage) {
        let no_cost = self.boundary.iter(stage).find(|dest| {
            dest.is_up()
                || dest
                    .as_enemy()
                    .is_some_and(|enemy| self.status.is_no_dmg(enemy))
                || dest.is_goal()
        });

        if let Some(no_cost) = no_cost {
            self.add_moved_state(buf, no_cost, stage);
        } else {
            for neighbor in self.boundary.iter(stage) {
                self.add_moved_state(buf, neighbor, stage);
            }
            self.add_converted_states(buf);
        }
    }

    fn add_moved_state(&self, buf: &mut Vec<Child>, dest: &Instance, stage: &Stage) {
        let moved_status = match &dest.object {
            Object::U8Up { kind, amount, .. } => self.status.get_u8_up(*kind, *amount),
            Object::HpUp { amount, .. } => self.status.get_hp_up(*amount),
            Object::Gate { kind, .. } => {
                let Some(moved_status) = self.status.try_open_gate(*kind) else {
                    return;
                };
                moved_status
            }
            Object::Enemy(enemy) => {
                let Some(moved_status) = self.status.try_battle(enemy) else {
                    return;
                };
                moved_status
            }
            Object::Goal { .. } => self.status,
        };

        buf.push(Child {
            state: self.moved(moved_status, dest, stage),
            action: Action::Move(dest.id),
        });
    }

    fn moved(&self, moved_status: Status, dest: &Instance, stage: &Stage) -> Self {
        let mut erased = (*self.erased).clone();
        erased.insert(dest);

        let neighbors = stage.neighbors_of(dest.id);
        let mut boundary = neighbors.clone();
        boundary.difference_with(&self.erased);
        boundary.union_with(&self.boundary);
        boundary.remove(dest);

        Self {
            status: moved_status,
            erased: Rc::new(erased),
            boundary,
        }
    }

    fn add_converted_states(&self, buf: &mut Vec<Child>) {
        if let Some(silver) = self.status.try_convert_silver() {
            buf.push(self.converted(ConvertKind::Silver, silver));
        }

        if let Some((hp, atk, def)) = self.status.try_convert_hp_atk_def() {
            buf.push(self.converted(ConvertKind::Hp, hp));
            buf.push(self.converted(ConvertKind::Atk, atk));
            buf.push(self.converted(ConvertKind::Def, def));
        }

        if let Some(gold) = self.status.try_convert_gold() {
            buf.push(self.converted(ConvertKind::Gold, gold));
        }
    }

    fn converted(&self, kind: ConvertKind, converted_status: Status) -> Child {
        let converted_state = Self {
            status: converted_status,
            erased: Rc::clone(&self.erased),
            boundary: self.boundary.clone(),
        };

        Child {
            state: converted_state,
            action: Action::Convert(kind),
        }
    }

    pub(crate) fn compare_status(&self, other: &Self) -> StatusCmp {
        self.status.compare(other.status)
    }
}
