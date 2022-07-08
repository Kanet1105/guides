use std::fmt;

pub trait Node: fmt::Display {
    fn run(&self);
}

pub trait State {}