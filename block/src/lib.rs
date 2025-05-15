
#[repr(u32)]
pub enum InitStatus {
	OK,
	ERROR,
	CONTINUE,
}

#[repr(u32)]
pub enum RunStatus {
	OK,
	END,
	ERROR,
}

pub trait Block {
	fn init(&mut self) -> InitStatus;
	fn step(&mut self) -> RunStatus;
	fn pause(&mut self) -> RunStatus;
	fn end(&mut self);
}
