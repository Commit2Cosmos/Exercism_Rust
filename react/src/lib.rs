use std::collections::{HashMap, HashSet};

/// `InputCellId` is a unique identifier for an input cell.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct InputCellId(usize);
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ComputeCellId(usize);
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CallbackId(usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CellId {
    Input(InputCellId),
    Compute(ComputeCellId),
}


struct InputCell<T> {
    value: T,
}


struct ComputeCell<'a, T> {
    value: T,
    dependencies: Vec<CellId>,
    compute_func: Box<dyn 'a + Fn(&[T]) -> T>,
    callbacks: HashMap<usize, Box<dyn 'a + FnMut(T)>>,
    next_callback_id: usize,
}


#[derive(Debug, PartialEq, Eq)]
pub enum RemoveCallbackError {
    NonexistentCell,
    NonexistentCallback,
}

#[derive(Default)]
pub struct Reactor<'a, T> {
    // Just so that the compiler doesn't complain about an unused type parameter.
    // You probably want to delete this field.
    input_cells: Vec<InputCell<T>>,
    compute_cells: Vec<ComputeCell<'a, T>>,
    //* cell -> cells to change if it changes */
    dependencies: HashMap<CellId, HashSet<ComputeCellId>>,
}

// You are guaranteed that Reactor will only be tested against types that are Copy + PartialEq.
impl<'a, T: Copy + PartialEq + Default> Reactor<'a, T> {
    pub fn new() -> Self {
        Self::default()
    }

    // Creates an input cell with the specified initial value, returning its ID.
    pub fn create_input(&mut self, initial: T) -> InputCellId {
        let input_cell_id = InputCellId(self.input_cells.len());
        self.input_cells.push(InputCell { value: initial });

        input_cell_id
    }

    // Creates a compute cell with the specified dependencies and compute function.
    // The compute function is expected to take in its arguments in the same order as specified in
    // `dependencies`.
    // You do not need to reject compute functions that expect more arguments than there are
    // dependencies (how would you check for this, anyway?).
    //
    // If any dependency doesn't exist, returns an Err with that nonexistent dependency.
    // (If multiple dependencies do not exist, exactly which one is returned is not defined and
    // will not be tested)
    //
    // Notice that there is no way to *remove* a cell.
    // This means that you may assume, without checking, that if the dependencies exist at creation
    // time they will continue to exist as long as the Reactor exists.
    pub fn create_compute<F>(
        &mut self,
        dependencies: &[CellId],
        compute_func: F,
    ) -> Result<ComputeCellId, CellId>

    where
        F: 'a + Fn(&[T]) -> T
    {
        
        let deps = dependencies.iter()
        .map(|&id| self.value(id).ok_or(id))
        .collect::<Result<Vec<T>, CellId>>()?;
        
        let compute_cell = ComputeCell {
            value: compute_func(&deps),
            dependencies: dependencies.to_vec(),
            compute_func: Box::new(compute_func),
            callbacks: Default::default(),
            next_callback_id: 0,
        };

        let compute_cell_id = ComputeCellId(self.compute_cells.len());
        self.compute_cells.push(compute_cell);


        for &dependency in dependencies {
            self.dependencies.entry(dependency)
            .or_default()
            .insert(compute_cell_id);
        }

        Ok(compute_cell_id)
    }

    // Retrieves the current value of the cell, or None if the cell does not exist.
    //
    // You may wonder whether it is possible to implement `get(&self, id: CellId) -> Option<&Cell>`
    // and have a `value(&self)` method on `Cell`.
    //
    // It turns out this introduces a significant amount of extra complexity to this exercise.
    // We chose not to cover this here, since this exercise is probably enough work as-is.
    pub fn value(&self, id: CellId) -> Option<T> {
        match id {
            CellId::Input(id) => self.input_cells.get(id.0).map(|x| x.value),
            CellId::Compute(id) => self.compute_cells.get(id.0).map(|x| x.value)
        }
    }

    // Sets the value of the specified input cell.
    //
    // Returns false if the cell does not exist.
    //
    // Similarly, you may wonder about `get_mut(&mut self, id: CellId) -> Option<&mut Cell>`, with
    // a `set_value(&mut self, new_value: T)` method on `Cell`.
    //
    // As before, that turned out to add too much extra complexity.
    pub fn set_value(&mut self, id: InputCellId, new_value: T) -> bool {
        if let Some(input_cell) = self.input_cells.get_mut(id.0) {
            input_cell.value = new_value;

            //* hashmap to check if the value actually changed (to check if callback should be called) */
            let mut updated: HashMap<ComputeCellId, T> = HashMap::new();
            self.update_dependencies(&CellId::Input(id), &mut updated);

            for (compute_cell_id, old_value) in updated {
                let compute_cell = self.compute_cells.get_mut(compute_cell_id.0).unwrap();

                if compute_cell.value != old_value {
                    for cb in compute_cell.callbacks.values_mut() {
                        cb(compute_cell.value)
                    }
                }
            }
            true
        } else {
            false
        }
    }


    //* Updates dependencies recursively */
    fn update_dependencies(
        &mut self,
        changed_id: &CellId,
        updated: &mut HashMap<ComputeCellId, T>
    ) {
        if let Some(compute_cell_ids) = self.dependencies.get(changed_id) {
            for compute_cell_id in compute_cell_ids.to_owned() {
                let compute_cell = &self.compute_cells[compute_cell_id.0];
                let depends_on = compute_cell.dependencies.iter()
                .map(|&id| self.value(id).ok_or(id))
                .collect::<Result<Vec<T>, CellId>>().unwrap();

                let new_value = (compute_cell.compute_func)(&depends_on);

                if new_value != compute_cell.value {
                    updated.entry(compute_cell_id)
                    .or_insert(compute_cell.value);
                    
                    self.compute_cells[compute_cell_id.0].value = new_value;
                    self.update_dependencies(&CellId::Compute(compute_cell_id), updated)
                }
            }
        }
    }

    // Adds a callback to the specified compute cell.
    //
    // Returns the ID of the just-added callback, or None if the cell doesn't exist.
    //
    // Callbacks on input cells will not be tested.
    //
    // The semantics of callbacks (as will be tested):
    // For a single set_value call, each compute cell's callbacks should each be called:
    // * Zero times if the compute cell's value did not change as a result of the set_value call.
    // * Exactly once if the compute cell's value changed as a result of the set_value call.
    //   The value passed to the callback should be the final value of the compute cell after the
    //   set_value call.
    pub fn add_callback<F: FnMut(T) + 'a>(
        &mut self,
        id: ComputeCellId,
        callback: F,
    ) -> Option<CallbackId> {
        let compute_cell = self.compute_cells.get_mut(id.0)?;
        compute_cell.next_callback_id += 1;
        compute_cell.callbacks.insert(compute_cell.next_callback_id, Box::new(callback));

        Some(CallbackId(compute_cell.next_callback_id))
    }

    // Removes the specified callback, using an ID returned from add_callback.
    //
    // Returns an Err if either the cell or callback does not exist.
    //
    // A removed callback should no longer be called.
    pub fn remove_callback(
        &mut self,
        cell: ComputeCellId,
        callback: CallbackId,
    ) -> Result<(), RemoveCallbackError> {
        if let Some(compute_cell) = self.compute_cells.get_mut(cell.0) {
            if let Some(_) = compute_cell.callbacks.remove(&callback.0) {
                Ok(())
            } else {
                Err(RemoveCallbackError::NonexistentCallback)
            }
        } else {
            Err(RemoveCallbackError::NonexistentCell)
        }
    }
}
