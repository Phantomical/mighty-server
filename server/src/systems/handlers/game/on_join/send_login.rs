use specs::*;
use types::*;

use GameMode;
use GameModeWriter;
use OwnedMessage;
use SystemInfo;

use component::channel::*;
use protocol::server::{Login, LoginPlayer};
use protocol::{to_bytes, ServerPacket, Upgrades as ProtocolUpgrades};

pub struct SendLogin {
	reader: Option<OnPlayerJoinReader>,
}

#[derive(SystemData)]
pub struct SendLoginData<'a> {
	pub channel: Read<'a, OnPlayerJoin>,
	pub conns: Read<'a, Connections>,
	pub entities: Entities<'a>,
	pub gamemode: GameModeWriter<'a, GameMode>,

	pub pos: ReadStorage<'a, Position>,
	pub rot: ReadStorage<'a, Rotation>,
	pub vel: ReadStorage<'a, Velocity>,
	pub plane: ReadStorage<'a, Plane>,
	pub team: ReadStorage<'a, Team>,
	pub status: ReadStorage<'a, Status>,
	pub flag: ReadStorage<'a, Flag>,
	pub upgrades: ReadStorage<'a, Upgrades>,
	pub powerups: ReadStorage<'a, Powerups>,
	pub name: ReadStorage<'a, Name>,
	pub level: ReadStorage<'a, Level>,
	pub session: ReadStorage<'a, Session>,
}

impl SendLogin {
	fn get_player_data<'a>(data: &SendLoginData<'a>) -> Vec<LoginPlayer> {
		(
			&*data.entities,
			&data.pos,
			&data.rot,
			&data.plane,
			&data.name,
			&data.flag,
			&data.upgrades,
			&data.level,
			&data.status,
			&data.team,
			&data.powerups,
		).join()
			.map({
				|(ent, pos, rot, plane, name, flag, upgrades, level, status, team, powerups)| {
					let upgrade_field = ProtocolUpgrades {
						speed: upgrades.speed,
						shield: powerups.shield,
						inferno: powerups.inferno,
					};

					LoginPlayer {
						id: ent,
						status: *status,
						level: *level,
						name: name.0.clone(),
						ty: *plane,
						team: *team,
						pos: *pos,
						rot: *rot,
						flag: *flag,
						upgrades: upgrade_field,
					}
				}
			})
			.collect()
	}
}

impl<'a> System<'a> for SendLogin {
	type SystemData = SendLoginData<'a>;

	fn setup(&mut self, res: &mut Resources) {
		Self::SystemData::setup(res);

		self.reader = Some(res.fetch_mut::<OnPlayerJoin>().register_reader());
	}

	fn run(&mut self, data: Self::SystemData) {
		for evt in data.channel.read(self.reader.as_mut().unwrap()) {
			let player_data = Self::get_player_data(&data);

			// TODO: Correct clock value and pass session through
			let packet = Login {
				clock: 0,
				id: evt.0,
				room: data.gamemode.get().room(),
				success: true,
				token: "none".to_owned(),
				team: *data.team.get(evt.0).unwrap(),
				ty: data.gamemode.get().gametype(),
				players: player_data,
			};

			data.conns.send_to_player(
				evt.0,
				OwnedMessage::Binary(to_bytes(&ServerPacket::Login(packet)).unwrap()),
			);
		}
	}
}

impl SystemInfo for SendLogin {
	type Dependencies = (super::InitTraits,);

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self { reader: None }
	}
}
