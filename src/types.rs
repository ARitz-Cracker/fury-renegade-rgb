use std::str::FromStr;

#[derive(Debug, Default, Clone, Copy)]
pub struct Colour {
	pub red: u8,
	pub green: u8,
	pub blue: u8,
}
impl FromStr for Colour {
	type Err = String;

	fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
		// First, try to see if we can parse it as a hex code
		if s.starts_with('#') && (s.len() == 7 || s.len() == 4) {
			let hex = if s.len() == 7 {
				&s[1..]
			} else {
				// Expand shorthand hex code (#RGB -> #RRGGBB)
				let r = &s[1..2];
				let g = &s[2..3];
				let b = &s[3..4];
				&format!("{}{}{}{}{}{}", r, r, g, g, b, b)
			};
			let red = u8::from_str_radix(&hex[0..2], 16).map_err(|_| "Invalid red value in hex".to_string())?;
			let green = u8::from_str_radix(&hex[2..4], 16).map_err(|_| "Invalid green value in hex".to_string())?;
			let blue = u8::from_str_radix(&hex[4..6], 16).map_err(|_| "Invalid blue value in hex".to_string())?;
			return Ok(Colour { red, green, blue });
		}

		let parts: Vec<&str> = s.split(',').collect();
		if parts.len() != 3 {
			return Err("Colour must be in the format R,G,B".to_string());
		}
		let red = parts[0].parse::<u8>().map_err(|_| "Invalid red value".to_string())?;
		let green = parts[1].parse::<u8>().map_err(|_| "Invalid green value".to_string())?;
		let blue = parts[2].parse::<u8>().map_err(|_| "Invalid blue value".to_string())?;
		Ok(Colour { red, green, blue })
	}
}
