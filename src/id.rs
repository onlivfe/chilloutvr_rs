//! Wrappers for CVR IDs.
//!
//! Wrapping them IDs in newtypes makes sure you aren't trying to accidentally
//! compare different types of CVR IDs with each other like so:
//!
//! ```compile_fail,E0308
//! let user_id = chilloutvr::id::User::try_from("totally-legit-id").unwrap();
//! let record_id = chilloutvr::id::Instance::try_from("totally-legit-id").unwrap();
//! assert!(user_id != record_id, "can't compare different types of IDs")
//! ```
//!
//! The deserializer implementations also check that the strings start with the
//! correct ID prefix.
//!
//! Note that the IDs seem to be handled as case-sensitive, so any normalized
//! versions are represented as strings instead of IDs.

use serde::de::{self, Deserializer, Visitor};
use serde::{Deserialize, Serialize};

macro_rules! add_id {
	(
		$(#[$meta:meta])*
		$name:ident
	) => {
		#[doc = concat!("An ID of a CVR ", stringify!($name))]
		///
		/// # Example usage
		///
		/// ```
		#[doc = concat!("use chilloutvr::id::", stringify!($name), ";")]
		#[doc = concat!("let id1 = ", stringify!($name), "::try_from(\"totally-legit-id\").unwrap();")]
		#[doc = concat!("let id2 = ", stringify!($name), "::try_from(\"tother-legit-id\").unwrap();")]
		/// assert!(id1 != id2);
		/// ```
		#[cfg(any(feature = "http", feature = "ws"))]
		#[derive(Clone, Debug, PartialEq, Eq, Serialize, Hash)]
		#[serde(transparent)]
		$(#[$meta])*
		pub struct $name(String);

		#[cfg(any(feature = "http", feature = "ws"))]
		impl AsRef<str> for $name {
			/// Extracts a string slice containing the entire inner String.
			#[must_use]
			fn as_ref(&self) -> &str {
				&self.0
			}
		}

		#[cfg(any(feature = "http", feature = "ws"))]
		impl std::fmt::Display for $name {
			fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
				write!(f, "{}", self.0)
			}
		}

		#[cfg(any(feature = "http", feature = "ws"))]
		impl TryFrom<String> for $name {
			type Error = &'static str;
			fn try_from(v: String) -> Result<Self, Self::Error> {
				Ok($name(v))
			}
		}

		/// For easier scripting, should use String otherwise.
		#[cfg(any(feature = "http", feature = "ws"))]
		impl TryFrom<&'static str> for $name {
			type Error = &'static str;
			fn try_from(v: &'static str) -> Result<Self, Self::Error> {
				Self::try_from(v.to_owned())
			}
		}

		#[cfg(any(feature = "http", feature = "ws"))]
		impl From<$name> for String {
			fn from(id: $name) -> String {
				id.0
			}
		}

		#[cfg(any(feature = "http", feature = "ws"))]
		impl From<$name> for Any {
			fn from(id: $name) -> Any {
				Any::$name(id)
			}
		}

		#[cfg(any(feature = "http", feature = "ws"))]
		impl<'de> serde::de::Deserialize<'de> for $name {
			fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
			where
				D: Deserializer<'de>,
			{
				struct IdVisitor;

				impl<'de> Visitor<'de> for IdVisitor {
					type Value = $name;

					fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
						formatter
							.write_str("an UUID resembling string")
					}

					fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
					where
						E: de::Error,
					{
						$name::try_from(v.to_owned()).map_err(|err| {
							de::Error::custom(err)
						})
					}
				}

				deserializer.deserialize_str(IdVisitor)
			}
		}

	};
}

add_id!(User);
add_id!(Instance);
add_id!(Invite);
add_id!(Asset);
add_id!(
	/// These IDs are plain, unlike others which seem to be UUIDs.
	Category
);
add_id!(File);

/// Any of the CVR IDs
///
/// # Example usage
///
/// ```
/// let id1 = chilloutvr::id::User::try_from("totally-legit-id").unwrap();
/// let id1: chilloutvr::id::Any = id1.into();
/// let id2 = chilloutvr::id::Instance::try_from("totally-legit-id").unwrap();
/// let id2: chilloutvr::id::Any = id2.into();
/// assert!(id1 != id2);
/// ```
#[cfg(any(feature = "http", feature = "ws"))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Any {
	/// An user ID
	User(User),
	/// An instance ID
	Instance(Instance),
	/// An asset ID
	Asset(Asset),
	/// An invite ID
	Invite(Invite),
	/// A file's ID
	File(File),
	/// A category's ID
	Category(Category),
}

#[cfg(any(feature = "http", feature = "ws"))]
impl AsRef<str> for Any {
	/// Extracts a string slice containing the entire inner String.
	#[must_use]
	fn as_ref(&self) -> &str {
		match self {
			Self::User(v) => v.as_ref(),
			Self::Instance(v) => v.as_ref(),
			Self::Asset(v) => v.as_ref(),
			Self::Invite(v) => v.as_ref(),
			Self::File(v) => v.as_ref(),
			Self::Category(v) => v.as_ref(),
		}
	}
}

#[cfg(any(feature = "http", feature = "ws"))]
impl std::fmt::Display for Any {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "{}", self.as_ref())
	}
}
