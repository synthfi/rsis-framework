mod sine;
use crate::sine::*;
use rsis_block::*;

impl Block for Sine<f32> {
    fn init(&mut self) -> InitStatus {
        self.data.phase = self.input.offset;
        if self.input.frequency == 0.0 {
            InitStatus::ERROR
        } else {
            InitStatus::OK
        }
    }
    fn step(&mut self) -> RunStatus {
        RunStatus::OK
    }
    fn pause(&mut self) -> RunStatus {
        RunStatus::OK
    }
    fn end(&mut self) {
        //
    }
}


impl Block for Sine<f64> {
    fn init(&mut self) -> InitStatus {
        self.data.phase = self.input.offset;
        if self.input.frequency == 0.0 {
            InitStatus::ERROR
        } else {
            InitStatus::OK
        }
    }
    fn step(&mut self) -> RunStatus {
        RunStatus::OK
    }
    fn pause(&mut self) -> RunStatus {
        RunStatus::OK
    }
    fn end(&mut self) {
        //
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = 2+2;
        assert_eq!(result, 4);
    }
}
