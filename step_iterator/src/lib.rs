use std::ops::Add;

pub struct StepIterator<T> {
    current: T,
    end: T,
    step: T,
    finished: bool,
}

impl<T> StepIterator<T>
where
    T: Copy,
{
    pub fn new(beg: T, end: T, step: T) -> Self {
        StepIterator {
            current: beg,
            end,
            step,
            finished: false,
        }
    }
}

impl<T> Iterator for StepIterator<T>
where
    T: Copy + PartialOrd + Add<Output = T>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }

        // Si on dépasse end → stop
        if self.current > self.end {
            self.finished = true;
            return None;
        }

        let value = self.current;
        self.current = self.current + self.step;

        Some(value)
    }
}