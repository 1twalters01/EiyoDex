use std::{cell::RefCell, rc::Rc};

use crate::exercise::Exercise;

#[derive(Debug, Clone, PartialEq)]
pub struct ExerciseList {
    name: String,
    description: String,
    exercises: Vec<Rc<RefCell<Exercise>>>,
}

impl ExerciseList {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            description: String::new(),
            exercises: Vec::new(),
        }
    }

    pub fn from_vec(exercises: Vec<Rc<RefCell<Exercise>>>) -> Self {
        Self {
            name: String::new(),
            description: String::new(),
            exercises: exercises,
        }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn get_description(&self) -> String {
        self.description.clone()
    }

    pub fn set_description(&mut self, description: String) {
        self.description = description;
    }

    pub fn get_exercises(&self) -> Vec<Rc<RefCell<Exercise>>> {
        self.exercises.clone()
    }

    pub fn set_exercises(&mut self, exercises: Vec<Rc<RefCell<Exercise>>>) {
        self.exercises = exercises
    }

    pub fn get_exercise_names(&self) -> Vec<String> {
        self.exercises
            .iter()
            .map(|exercise| exercise.borrow().get_name())
            .collect()
    }

    pub fn push(&mut self, exercise: Rc<RefCell<Exercise>>) {
        self.exercises.push(exercise)
    }

    pub fn extend(&mut self, exercises: Vec<Rc<RefCell<Exercise>>>) {
        self.exercises.extend(exercises);
    }

    pub fn remove(&mut self, exercise: Rc<RefCell<Exercise>>) {
        self.exercises.retain(|n| Rc::ptr_eq(n, &exercise))
    }
}
