use specs::*;
use types::*;

use OwnedMessage;
use SystemInfo;

use component::channel::*;
use protocol::server::PlayerLevel;
use protocol::{to_bytes, PlayerLevelType, ServerPacket};

pub struct SendPlayerLevel {
	reader: Option<OnPlayerJoinReader>,
}

#[derive(SystemData)]
pub struct SendPlayerLevelData<'a> {
	pub channel: Read<'a, OnPlayerJoin>,
	pub conns: Read<'a, Connections>,

	pub level: ReadStorage<'a, Level>,
}

impl<'a> System<'a> for SendPlayerLevel {
	type SystemData = SendPlayerLevelData<'a>;

	fn setup(&mut self, res: &mut Resources) {
		Self::SystemData::setup(res);

		self.reader = Some(res.fetch_mut::<OnPlayerJoin>().register_reader());
	}

	fn run(&mut self, data: Self::SystemData) {
		let Self::SystemData {
			channel,
			conns,

			level,
		} = data;

		for evt in channel.read(self.reader.as_mut().unwrap()) {
			let packet = PlayerLevel {
				id: evt.0,
				ty: PlayerLevelType::Login,
				level: *level.get(evt.0).unwrap(),
			};

			conns.send_to_others(
				evt.0,
				OwnedMessage::Binary(to_bytes(&ServerPacket::PlayerLevel(packet)).unwrap()),
			);
		}
	}
}

impl SystemInfo for SendPlayerLevel {
	type Dependencies = (super::InitTraits, super::SendLogin);

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self { reader: None }
	}
}
