#[derive(Debug)]
pub struct Builder<'a> {
    version: Option<&'a String>,
    heuristic: Option<&'a String>,
    ordering: Option<&'a String>,
    workers: Option<u32>,
}

impl<'a> Builder<'a> {
    pub fn new() -> Self {
        Self {
            version: None,
            heuristic: None,
            ordering: None,
            workers: None,
        }
    }

    pub fn set_version(mut self, version: &'a String) -> Self {
        self.version = Some(version);
        self
    }

    pub fn set_heuristic(mut self, heuristic: Option<&'a String>) -> Self {
        self.heuristic = heuristic;
        self
    }

    pub fn set_ordering(mut self, ordering: Option<&'a String>) -> Self {
        self.ordering = ordering;
        self
    }

    pub fn set_workers(mut self, workers: u32) -> Self {
        self.workers = Some(workers);
        self
    }

    pub fn build(self) -> bool {
        // pub fn build(self) -> Scheduler {
        // TODO: Implement this
        println!("{:?}", self);
        false
    }
}