use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

pub trait GraphNode {
    fn get_children(&self) -> Vec<Rc<RefCell<Self>>>;
    fn get_parents(&self) -> Vec<Weak<RefCell<Self>>>;
}
