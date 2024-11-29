use crate::scheduler::orderings::strategy::OrderingStrategy;

pub struct Decreasing;

impl Decreasing {}


impl OrderingStrategy for Decreasing {
	fn something(&self) {}
}