use crate::scheduler::orderings::strategy::OrderingStrategy;

pub struct Increasing;

impl Increasing {}

impl OrderingStrategy for Increasing {
	fn something(&self) {}
}