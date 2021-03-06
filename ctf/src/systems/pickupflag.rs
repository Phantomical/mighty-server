use server::types::Sqrt;
use server::*;
use specs::*;

use server::component::flag::*;
use server::component::time::ThisFrame;

use component::*;
use config as ctfconfig;

use std::cmp::Ordering;

pub struct PickupFlagSystem;

#[derive(SystemData)]
pub struct PickupFlagSystemData<'a> {
	pub config: Read<'a, Config>,
	pub entities: Entities<'a>,
	pub channel: Write<'a, OnFlag>,
	pub thisframe: Read<'a, ThisFrame>,

	// Player data
	pub plane: ReadStorage<'a, Plane>,
	pub is_player: ReadStorage<'a, IsPlayer>,
	pub is_spec: ReadStorage<'a, IsSpectating>,
	pub is_dead: ReadStorage<'a, IsDead>,

	// These ones are for both
	pub pos: WriteStorage<'a, Position>,
	pub team: ReadStorage<'a, Team>,

	// Flag Data
	pub is_flag: ReadStorage<'a, IsFlag>,
	pub carrier: WriteStorage<'a, FlagCarrier>,
	pub lastdrop: ReadStorage<'a, LastDrop>,
}

impl<'a> System<'a> for PickupFlagSystem {
	type SystemData = PickupFlagSystemData<'a>;

	fn run(&mut self, mut data: Self::SystemData) {
		let flags = (
			&*data.entities,
			&data.pos,
			&data.team,
			&data.carrier,
			&data.is_flag,
			&data.lastdrop,
		).join()
			.filter(|(ent, _, _, _, _, _)| {
				data.is_dead.get(*ent).is_none() && data.is_spec.get(*ent).is_none()
			})
			.map(|(ent, pos, team, carrier, _, lastdrop)| (ent, *pos, *team, *carrier, *lastdrop))
			.collect::<Vec<(Entity, Position, Team, FlagCarrier, LastDrop)>>();

		for (f_ent, f_pos, f_team, carrier, lastdrop) in flags {
			if carrier.0.is_some() {
				continue;
			}

			let nearest = (
				&*data.entities,
				&data.pos,
				&data.team,
				&data.is_player,
				&data.plane,
			).join()
				.filter(|(_, _, p_team, _, _)| f_team != **p_team)
				.filter(|(ent, _, _, _, _)| {
					// Check against time-since-drop
					(data.thisframe.0 - lastdrop.time) > *ctfconfig::FLAG_NO_REGRAB_TIME
						// Then check against contained player id
						|| lastdrop.player.map(|x| x != *ent).unwrap_or(false)
				})
				.filter_map(|(p_ent, p_pos, _, _, p_plane)| {
					let rad = ctfconfig::FLAG_RADIUS[&p_plane];
					let dst = (*p_pos - f_pos).length2();

					// Quickly filter out negative distances
					// without doing a sqrt
					if dst > rad * rad {
						None
					} else {
						// Only calculate actual distance if necessary
						Some((p_ent, dst.sqrt() - rad))
					}
				})
				.min_by(|a, b| {
					if a.1 < b.1 {
						Ordering::Less
					} else {
						Ordering::Greater
					}
				});

			if nearest.is_none() {
				continue;
			}

			let nearest = nearest.unwrap().0;
			let team = *data.team.get(nearest).unwrap();

			*data.carrier.get_mut(f_ent).unwrap() = FlagCarrier(Some(nearest));

			let ty = if team == f_team {
				FlagEventType::Return
			} else {
				FlagEventType::PickUp
			};

			data.channel.single_write(FlagEvent {
				ty,
				player: Some(nearest),
				flag: f_ent,
			});
		}
	}
}

use super::LoginUpdateSystem;
use server::systems::PositionUpdate;

impl SystemInfo for PickupFlagSystem {
	type Dependencies = (PositionUpdate, LoginUpdateSystem);

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self {}
	}
}
