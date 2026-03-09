use crate::dsa::node::GraphNode;
use std::{cell::RefCell, collections::HashSet, fmt::Debug, rc::Rc};

pub trait DFSTrait: GraphNode {
    fn dfs_down(
        node: &Rc<RefCell<Self>>,
        out: &mut Vec<Rc<RefCell<Self>>>,
        visited: &mut HashSet<*const RefCell<Self>>,
    ) where
        Self: DFSTrait,
        Self: Debug,
    {
        let ptr = Rc::as_ptr(node);
        if !visited.insert(ptr) {
            return;
        }
        // println!("node: {:#?}", node.borrow());

        out.push(node.clone());

        let n = node.borrow();
        for child in n.get_children() {
            Self::dfs_down(&child, out, visited);
        }
    }

    fn dfs_up(
        node: &Rc<RefCell<Self>>,
        out: &mut Vec<Rc<RefCell<Self>>>,
        visited: &mut HashSet<*const RefCell<Self>>,
    ) where
        Self: DFSTrait,
    {
        let ptr = Rc::as_ptr(node);
        if !visited.insert(ptr) {
            return;
        }

        out.push(node.clone());

        let n = node.borrow();
        for parent_weak in n.get_parents() {
            if let Some(parent) = parent_weak.upgrade() {
                Self::dfs_up(&parent, out, visited);
            }
        }
    }
}
