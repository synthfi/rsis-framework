
mod BasicModel;
use crate::BasicModel::*;
use rsis_block::*;

impl Block for BasicModel::BasicModel {
    fn init(&mut self) -> InitStatus {
        for i in 0..2 {
            if self.params.limits[(i, 1)] < self.params.limits[(i, 0)] {
                return InitStatus::ERROR;
            }
        }
        InitStatus::OK
    }
    fn step(&mut self) -> RunStatus {
        if self.input.endsim {
            RunStatus::END
        } else {
            RunStatus::OK
        }
    }
    fn pause(&mut self) -> RunStatus {
        RunStatus::OK
    }
    fn end(&mut self) {
        //
    }
}
