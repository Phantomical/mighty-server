use server::*;
use specs::*;

use component::*;
use config as ctfconfig;

use htmlescape;

use server::protocol::server::{ServerMessage, ServerPacket};
use server::protocol::{to_bytes, ServerMessageType};

pub struct PickupMessageSystem {
	reader: Option<OnFlagReader>,
}

#[derive(SystemData)]
pub struct PickupMessageSystemData<'a> {
	pub channel: Read<'a, OnFlag>,
	pub conns: Read<'a, Connections>,

	pub names: ReadStorage<'a, Name>,
	pub teams: ReadStorage<'a, Team>,
}

impl PickupMessageSystem {
	pub fn new() -> Self {
		Self { reader: None }
	}
}

impl<'a> System<'a> for PickupMessageSystem {
	type SystemData = PickupMessageSystemData<'a>;

	fn setup(&mut self, res: &mut Resources) {
		Self::SystemData::setup(res);

		self.reader = Some(res.fetch_mut::<OnFlag>().register_reader());
	}

	fn run(&mut self, data: Self::SystemData) {
		for evt in data.channel.read(self.reader.as_mut().unwrap()) {
			let verb = match evt.ty {
				FlagEventType::Return => "Returned",
				FlagEventType::PickUp => "Taken",
				FlagEventType::Capture => "Captured",
				FlagEventType::Drop => continue,
			};

			let flag_team = data.teams.get(evt.flag).unwrap();
			let name = data.names.get(evt.player.unwrap()).unwrap();

			let msg = format!(
				"<span class=\"info inline\"><span class=\"{}\"></span></span>{} by {}",
				ctfconfig::FLAG_MESSAGE_TEAM[&flag_team],
				verb,
				htmlescape::encode_minimal(&name.0)
			);

			let packet = ServerMessage {
				ty: ServerMessageType::FlagMessage,
				duration: 3000,
				text: msg,
			};

			data.conns.send_to_all(OwnedMessage::Binary(
				to_bytes(&ServerPacket::ServerMessage(packet)).unwrap(),
			));
		}
	}
}

use super::{PickupFlagSystem, ReturnFlagSystem};

impl SystemInfo for PickupMessageSystem {
	type Dependencies = (ReturnFlagSystem, PickupFlagSystem);

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self::new()
	}
}
