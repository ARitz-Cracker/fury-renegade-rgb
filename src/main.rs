mod headbang;
mod fury_commands;
mod error;

use std::path::PathBuf;

use color_eyre::eyre::Result;
use fury_commands::{PatternStyle, MultiRamController};

use bpaf::Bpaf;

#[derive(Clone, Debug, Bpaf)]
#[bpaf(options, version)]
/// Accept speed and distance, print them
struct FuryControllerOptions {
	#[bpaf(short, long)]
	/// i2c bus to use, e.g. /dev/i2c-1
	bus: PathBuf,
	#[bpaf(short('1'), long)]
	/// Run command on stick 1
	stick_1: bool,
	#[bpaf(short('2'), long)]
	/// Run command on stick 2
	stick_2: bool,
	#[bpaf(short('3'), long)]
	/// Run command on stick 3
	stick_3: bool,
	#[bpaf(short('4'), long)]
	/// Run command on stick 4
	stick_4: bool,
	#[bpaf(external(action))]
	action: Action
}

fn is_percent(number: &u8) -> bool {
	*number <= 100
}
const PERCENT_TOO_HIGH: &str = "Percent values must be equal to or less than 100";

#[derive(Debug, Clone, Bpaf)]
//#[bpaf(options, version)]
pub(crate) enum Action {
	#[bpaf(command("noop"))]
	Noop,
	#[bpaf(command("reset"))]
	Reset,
	#[bpaf(command("sync"))]
	/// Sync timings between the sticks (only tested on 4 sticks)
	Sync,
	#[bpaf(command("colour-brightness"))]
	/// Sets a rgb mask on whatever the pattern does
	ColourBrightness {
		#[bpaf(short, long, guard(is_percent, PERCENT_TOO_HIGH))]
		/// Value between 0 and 100
		red: u8,
		#[bpaf(short, long, guard(is_percent, PERCENT_TOO_HIGH))]
		/// Value between 0 and 100
		green: u8,
		/// Value between 0 and 100
		#[bpaf(short, long, guard(is_percent, PERCENT_TOO_HIGH))]
		blue: u8
	},
	#[bpaf(command("brightness"))]
	/// Sets the overall brightness of the stick
	Brightness {
		#[bpaf(short, long, guard(is_percent, PERCENT_TOO_HIGH))]
		/// Value between 0 and 100
		value: u8
	},
	#[bpaf(command("pattern-start-offset"))]
	/// Sets a delay before starting the pattern. On a syncronized set of sticks, the offset appears to be additive
	PatternStartOffset {
		#[bpaf(short, long)]
		raw_offset: u8
	},
	#[bpaf(command("pattern-repeat-delay"))]
	/// A delay before the pattern repeats, this should be set to the same value on all sticks
	PatternRepeatDelay {
		#[bpaf(short, long)]
		raw_delay: u8
	},
	#[bpaf(command("pattern"))]
	/// Which pattern and style to use
	Pattern {
		#[bpaf(short, long)]
		/// Can be one of: solid, rainbow, scan, breathe, fade, stripe, trail, lightning, countdown, fire, sparkles, fury
		style: PatternStyle,
		#[bpaf(short, long)]
		/// Value form 0 to 255
		red: u8,
		#[bpaf(short, long)]
		/// Value form 0 to 255
		green: u8,
		#[bpaf(short, long)]
		/// Value form 0 to 255
		blue: u8,
		#[bpaf(short, long)]
		/// Cycles between the following colours: custom, green, orange, blue, yellow-sih green, pink, cyan, yellow, bright-punk, bright-cyan, red
		colour_cycle: bool
	},
	#[bpaf(command("pattern-style"))]
	/// Only sets the pattern style, can be one of solid, rainbow, scan, breathe, fade, stripe, trail, lightning, countdown, fire, sparkles, fury
	PatternStyle(
		PatternStyle
	),
	#[bpaf(command("pattern-colour"))]
	/// Only sets the pattern custom colour
	PatternColour {
		#[bpaf(short, long)]
		/// Value form 0 to 255
		red: u8,
		#[bpaf(short, long)]
		/// Value form 0 to 255
		green: u8,
		#[bpaf(short, long)]
		/// Value form 0 to 255
		blue: u8,
	}
}

const STICK_ADDRESS_1: u16 = 0x60;
const STICK_ADDRESS_2: u16 = 0x61;
const STICK_ADDRESS_3: u16 = 0x62;
const STICK_ADDRESS_4: u16 = 0x63;
fn main() -> Result<()> {
	color_eyre::install()?;
	let opts = fury_controller_options().run();
	let mut sticks = Vec::with_capacity(4);
	if opts.stick_1 {
		sticks.push(STICK_ADDRESS_1);
	}
	if opts.stick_2 {
		sticks.push(STICK_ADDRESS_2);
	}
	if opts.stick_3 {
		sticks.push(STICK_ADDRESS_3);
	}
	if opts.stick_4 {
		sticks.push(STICK_ADDRESS_4);
	}
	let mut fury_controller = MultiRamController::new(
		opts.bus,
		sticks
	)?;
	match opts.action {
		Action::Noop => {
			fury_controller.noop()?;
		},
		Action::Reset => {
			fury_controller.reset()?;
		}
		Action::Sync => {
			fury_controller.sync_timings()?;
		},
		Action::ColourBrightness { red, green, blue } => {
			fury_controller.set_rgb_brightness_percent(red, green, blue)?;
		},
		Action::Brightness { value } => {
			fury_controller.set_brightness_percent(value)?;
		},
		Action::PatternStartOffset { raw_offset } => {
			fury_controller.set_pattern_start_offset(raw_offset)?;
		},
		Action::PatternRepeatDelay { raw_delay } => {
			fury_controller.set_pattern_repeat_delay(raw_delay)?;
		},
		Action::Pattern { style, red, green, blue, colour_cycle } => {
			fury_controller.set_pattern(style, red, green, blue, colour_cycle)?;
		},
		Action::PatternStyle(style) => {
			fury_controller.set_pattern_style_only(style)?;
		}
		Action::PatternColour { red, green, blue } => {
			fury_controller.set_pattern_colour_only(red, green, blue)?;
		},
	}
	Ok(())
}
