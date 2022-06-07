use std::fmt::Display;

#[allow(dead_code)]
// TODO library links for docs
/// An enum to indicate the source of data

#[derive(Clone, Debug)]
pub enum Provider {
	/// Unknown, mostly a placeholder
	Unknown,
	/// Data from the system, mostly accessed/modified/created times
	System,
	/// Data from the `id3` library
	Id3,
	/// Data from the `lofty` library
	Lofty,
	/// Data from the `rexif` library
	Rexif,
	/// Data from the `kamadak-exif` library
	Kamadak,
}

impl PartialEq for Provider {
	fn eq(&self, other: &Self) -> bool {
		match (self, other) {
			(Provider::System, Provider::System) => true,
			(Provider::Id3, Provider::Id3) => true,
			(Provider::Lofty, Provider::Lofty) => true,
			(Provider::Rexif, Provider::Rexif) => true,
			(Provider::Kamadak, Provider::Kamadak) => true,
			(Provider::Unknown, Provider::Unknown) => true,
			// If you want to add a custom provider, add `(Provider::a, Provider::a) => true,`
			_ => false,
		}
	}
}

impl Display for Provider {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{}",
			match self {
				Provider::Unknown => "Unknown",
				Provider::System => "System",
				Provider::Id3 => "Id3",
				Provider::Lofty => "Lofty",
				Provider::Rexif => "Rexif",
				Provider::Kamadak => "Kamadak",
			}
		)
	}
}
