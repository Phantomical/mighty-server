use server::protocol::PlaneType;
use server::*;

use fnv::FnvHashMap;
use std::time::Duration;

lazy_static! {
	pub static ref FLAG_RADIUS: FnvHashMap<Plane, Distance> = {
		let mut map = FnvHashMap::default();

		// These are just random guesses
		// TODO: rev-eng these from official server
		map.insert(
			PlaneType::Predator,
			Distance::new(100.0)
		);

		map.insert(
			PlaneType::Goliath,
			Distance::new(100.0)
		);

		map.insert(
			PlaneType::Tornado,
			Distance::new(100.0)
		);

		map.insert(
			PlaneType::Prowler,
			Distance::new(100.0)
		);

		map.insert(
			PlaneType::Mohawk,
			Distance::new(100.0)
		);

		map
	};
	// TODO: Actually determine this
	/// Distance that the player must be within to cap
	pub static ref CAP_RADIUS: Distance = Distance::new(50.0);

	pub static ref FLAG_POS: FnvHashMap<Team, Position> = {
		let mut map = FnvHashMap::default();

		// Blue team
		map.insert(Team(1), Position::new(
			Distance::new(-9670.0),
			Distance::new(-1470.0))
		);
		// Red team
		map.insert(Team(2), Position::new(
			Distance::new(8600.0),
			Distance::new(-940.0))
		);

		map
	};
	pub static ref FLAG_RETURN_POS: FnvHashMap<Team, Position> = {
		let mut map = FnvHashMap::default();

		// Flags get returned at the opposite base
		map.insert(Team(2), FLAG_POS[&Team(1)]);
		map.insert(Team(1), FLAG_POS[&Team(2)]);

		map
	};

	pub static ref FLAG_NO_REGRAB_TIME: Duration = Duration::from_secs(5);

	pub static ref FLAG_MESSAGE_TEAM: FnvHashMap<Team, &'static str> = {
		let mut map = FnvHashMap::default();

		map.insert(Team(1), "blueflag");
		map.insert(Team(2), "redflag");

		map
	};
}
