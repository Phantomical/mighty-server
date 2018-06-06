
use specs::*;
use specs::prelude::*;
use types::*;

use component::time::{ThisFrame, StartTime};
use component::flag::IsMissile;
use component::reference::PlayerRef;

use websocket::OwnedMessage;
use airmash_protocol::{ServerPacket, to_bytes};
use airmash_protocol::server::{PlayerFire, PlayerFireProjectile};

pub struct MissileFireHandler;

#[derive(SystemData)]
pub struct MissileFireHandlerData<'a> {
	pub ents: Entities<'a>,
	pub pos: WriteStorage<'a, Position>,
	pub vel: WriteStorage<'a, Velocity>,
	pub rot: ReadStorage<'a, Rotation>,
	pub plane: ReadStorage<'a, Plane>,
	pub teams: WriteStorage<'a, Team>,
	pub keystate: ReadStorage<'a, KeyState>,
	pub energy: WriteStorage<'a, Energy>,
	pub config: Read<'a, Config>,
	pub flags: WriteStorage<'a, IsMissile>,
	pub mobs: WriteStorage<'a, Mob>,
	pub owner: WriteStorage<'a, PlayerRef>,
	pub conns: Read<'a, Connections>,
	pub starttime: Read<'a, StartTime>,
	pub thisframe: Read<'a, ThisFrame>
}

impl<'a> System<'a> for MissileFireHandler {
	type SystemData = MissileFireHandlerData<'a>;

	fn run(&mut self, data: Self::SystemData) {
		let clock = (data.thisframe.0 - data.starttime.0).to_clock();
		
		let MissileFireHandlerData {
		 	ents,
			mut pos,
			mut vel,
			rot,
			keystate,
			plane,
			mut teams,
			mut energy,
			config,
			mut flags,
			mut mobs,
			mut owner,
			conns,
			..
		} = data;

		let new = (&*ents, &pos, &vel, &rot, &keystate, &mut energy, &plane, &teams)
			.par_join()
			.filter_map(|(ent, pos, vel, rot, keystate, energy, plane, team)| {
				let ref info = config.planes[*plane];
				let ref missile = config.mobs[info.missile_type].missile.unwrap();

				if keystate.fire && *energy > info.fire_energy {
					let m_dir = Vector2::new(rot.cos(), rot.sin());

					// Component of velocity parallel to direction
					let vel_par = *vel * (
						(m_dir * detail::f32consts::V).dot(*vel) / vel.length2());

					let m_vel = vel_par * missile.speed_factor
						+ m_dir * missile.base_speed;
					let m_accel = m_dir * missile.accel;

					let m_ent = ents.create();

					*energy -= info.fire_energy;

					let packet = PlayerFire {
						clock: clock,
						id: ent.id() as u16,
						energy: energy.inner(),
						energy_regen: info.energy_regen.inner(),
						projectiles: vec![
							PlayerFireProjectile {
								id: m_ent.id() as u16,
								accel_x: m_accel.x.inner(),
								accel_y: m_accel.y.inner(),
								pos_x: pos.x.inner(),
								pos_y: pos.y.inner(),
								speed_x: vel.x.inner(),
								speed_y: vel.y.inner(),
								ty: info.missile_type,
								max_speed: missile.max_speed.inner()
							}
						]
					};

					conns.send_to_player(ent, OwnedMessage::Binary(
						to_bytes(&ServerPacket::PlayerFire(packet)).unwrap()
					));

					return Some((
						m_ent,
						info.missile_type,
						*pos,
						m_vel,
						*team,
						ent
					));
				}

				None
			})
			.collect::<Vec<(Entity, Mob, Position, Velocity, Team, Entity)>>();

		for v in new {
			pos.insert(v.0, v.2).unwrap();
			vel.insert(v.0, v.3).unwrap();
			mobs.insert(v.0, v.1).unwrap();
			flags.insert(v.0, IsMissile{}).unwrap();
			teams.insert(v.0, v.4).unwrap();
			owner.insert(v.0, PlayerRef(v.5)).unwrap();
		} 
	}
}
