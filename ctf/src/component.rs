use shrev::*;
use specs::*;

use std::time::Instant;

#[derive(Copy, Clone, Debug, Default, Component)]
#[storage(NullStorage)]
pub struct IsFlag;

#[derive(Copy, Clone, Debug, Default, Component)]
#[storage(HashMapStorage)]
pub struct FlagCarrier(pub Option<Entity>);

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum FlagEventType {
	PickUp,
	Return,
	Capture,
	Drop,
}

#[derive(Copy, Clone, Debug)]
pub struct FlagEvent {
	pub ty: FlagEventType,
	/// Player that carried out the action (capturer, player that returned)
	pub player: Option<Entity>,
	pub flag: Entity,
}

#[derive(Copy, Clone, Debug, Component)]
#[storage(HashMapStorage)]
pub struct LastDrop {
	pub player: Option<Entity>,
	pub time: Instant,
}

#[derive(Copy, Clone, Debug, Default)]
pub struct GameScores {
	pub redteam: u8,
	pub blueteam: u8,
}

#[derive(Copy, Clone)]
pub struct Flags {
	pub red: Entity,
	pub blue: Entity,
}

pub type OnFlag = EventChannel<FlagEvent>;
pub type OnFlagReader = ReaderId<FlagEvent>;
