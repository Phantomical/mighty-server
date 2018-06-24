
use utils::timer::*;

lazy_static! {
	pub static ref PING_DISPATCH: TimerEventType = register_event_type();
	pub static ref AFK_TIMER: TimerEventType = register_event_type();
	pub static ref SCORE_BOARD: TimerEventType = register_event_type();
}
