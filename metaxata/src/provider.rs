#[allow(dead_code)]
// TODO library links for docs
/// An enum to indicate the source of data

#[derive(Clone)]
pub enum Provider {
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
			// If you want to add a custom provider, add `(Provider::a, Provider::a) => true,`
			_ => false
		}
	}
}
