use error::ValueError;
use model::{GameVersion, MainSong, RawObject};
use model::de;
use std;
use std::convert::From;
use std::convert::TryFrom;
use std::fmt::{Display, Error, Formatter};
use std::num::ParseIntError;
use std::str::FromStr;

/// Enum representing the possible level lengths known to GDCF
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "deser", derive(Serialize, Deserialize))]
pub enum LevelLength {
    /// Tiny
    ///
    /// ### GD Internals:
    /// This variant is represented by the value `0` in both requests and responses
    Tiny,

    /// Short
    ///
    /// ### GD Internals:
    /// This variant is represented by the value `1` in both requests and responses
    Short,

    /// Medium
    ///
    /// ### GD Internals:
    /// This variant is represented by the value `2` in both requests and responses
    Medium,

    /// Long
    ///
    /// ### GD Internals:
    /// This variant is represented by the value `3` in both requests and responses
    Long,

    /// Extra Long, sometime referred to as `XL`
    ///
    /// ### GD Internals:
    /// This variant is represented by the value `4` in both requests and responses
    ExtraLong,

    /// Enum variant that's used by the `From<i32>` impl for when an unrecognized value is passed
    Unknown,
}


/// Enum representing the possible level ratings
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "deser", derive(Serialize, Deserialize))]
pub enum LevelRating {
    /// Auto rating. This variant is only used when making requests. Use the [is_auto](struct.PartialLevel.html#structfield.is_auto)
    /// field to check if a level is auto instead.
    ///
    /// ### GD Internals:
    /// This variant is represented by the value `-3` in requests, and not included in responses.
    Auto,

    /// Demon rating.
    ///
    /// ### GD Internals:
    /// This variant is represented by the value `-2` in requests. In responses, you will have to
    /// first check the provided level is a demon and then interpret the provided `rating` value as a
    /// [DemonRating](struct.DemonRating.html)
    Demon(DemonRating),

    /// Not Available, sometimes referred to as `N/A` or `NA`
    ///
    /// ### GD Internals:
    /// This variant is represented by the value `-1` in requests and by the value `0` in responses
    NotAvailable,

    /// Easy rating
    ///
    /// ### GD Internals:
    /// This variant is represented by the value `1` in requests and by the value `10` in responses
    Easy,

    /// Normal rating
    ///
    /// ### GD Internals:
    /// This variant is represented by the value `2` in requests and by the value `20` in responses
    Normal,

    /// Hard rating
    ///
    /// ### GD Internals:
    /// This variant is represented by the value `3` in requests and by the value `30` in responses
    Hard,

    /// Harder rating
    ///
    /// ### GD Internals:
    /// This variant is represented by the value `4` in requests and by the value `40` in responses
    Harder,

    /// Insane rating
    ///
    /// ### GD Internals:
    /// This variant is represented by the value `5` in requests and by the value `50` in responses
    Insane,

    /// Enum variant that's used by the `From<i32>`impl for when an unrecognized value is passed
    Unknown,
}

/// Enum representing the possible demon difficulties
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "deser", derive(Serialize, Deserialize))]
pub enum DemonRating {
    /// Easy demon
    ///
    /// ### GD Internals:
    /// This variant is represented by the value `1` in requests and by the value `10` in responses
    Easy,

    /// Medium demon
    ///
    /// ### GD Internals:
    /// This variant is represented by the value `2` in requests and by the value `20` in responses
    Medium,

    /// Hard demon
    ///
    /// ### GD Internals:
    /// This variant is represented by the value `3` in requests and by the value `30` in responses
    Hard,

    /// Insane demon
    ///
    /// ### GD Internals:
    /// This variant is represented by the value `4` in requests and by the value `40` in responses
    Insane,

    /// Extreme demon
    ///
    /// ### GD Internals:
    /// This variant is represented by the value `5` in requests and by the value `50` in responses
    Extreme,

    /// Enum variant that's used by the `From<i32>` impl for when an unrecognized value is passed
    Unknown,
}

/// Enum representing a levels featured state
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "deser", derive(Serialize, Deserialize))]
pub enum Featured {
    /// The level isn't featured, and has never been featured before
    NotFeatured,

    /// The level isn't featured, but used to be (it either got unrated, or unfeatured, like Sonic Wave)
    Unfeatured,

    /// The level is featured, and has the contained value as its featured weight.
    ///
    /// The featured weight determines how high on the featured pages the level appear, where a
    /// higher value means a higher position.
    Featured(u32),
}

/// Struct representing partial levels. These are returned to [LevelsRequest](../../api/request/level/struct.LevelsRequest.html)s
/// and only contain metadata on the level.
///
/// ## GD Internals:
/// The Geometry Dash servers provided lists of partial levels via the `getGJLevels` endpoint.
///
/// ### Unmapped values:
/// + Index `8`: Index 8 is a boolean value indicating whether the level has a difficulty rating that isn't N/A.
/// This is equivalent to checking if [difficulty](struct.PartialLevel.html#structfield.difficulty)
/// is unequal to [NotAvailable](enum.LevelRating.html#variant.NotAvailable)
/// + Index `17`: Index 17 is a boolean value indicating whether the level is a demon level.
/// This is equivalent to checking if [difficulty](struct.PartialLevel.html#structfield.difficulty)
/// if the [Demon](enum.LevelRating.html#variant.Demon) variant.
/// + Index `25`: Index 25 is a boolean value indicating whether the level is an auto level.
/// This is equivalent to checking if [difficulty](struct.PartialLevel.html#structfield.difficulty)
/// is equal to [Demon](enum.LevelRating.html#variant.Auto).
///
/// ### Unprovided values:
/// These values are not provided for by the `getGJLevels` endpoint and are thus only modelled in the
/// [Level](struct.Level.html) struct: `4`, `27`, `28`, `29`, `36`
///
/// ### Unused indices:
/// The following indices arent used by the Geometry Dash servers: `11`, `16`, `17`, `20`, `21`,
/// `22`, `23`, `24`, `26`, `31`, `32`, `33`, `34`, `40`, `41`, `44`
#[derive(Debug, FromRawObject)]
#[cfg_attr(feature = "deser", derive(Serialize, Deserialize))]
pub struct PartialLevel {
    /// The `Level`'s unique level id
    ///
    /// ## GD Internals:
    /// This value is provided at index `1`.
    #[raw_data(index = 1)]
    pub level_id: u64,

    /// The `Level`'s name
    ///
    /// ## GD Internals:
    /// This value is provided at index `2`.
    #[raw_data(index = 2)]
    pub name: String,

    /// The `Level`'s description. Is `None` if the creator didn't put any description.
    ///
    /// ## GD Internals:
    /// This value is provided at index `3` and encoded using urlsafe base 64.
    #[raw_data(index = 3, deserialize_with = "de::into_option", default)]
    pub description: Option<String>,

    /// The `PartialLevel`'s version. The version get incremented every time the level is updated,
    /// and the initial version is always version 1.
    ///
    /// ## GD Internals:
    /// This value is provided at index `5`.
    #[raw_data(index = 5)]
    pub version: u32,

    /// The ID of the `Level`'s creator
    ///
    /// ## GD Internals:
    /// This value is provided at index `6`.
    #[raw_data(index = 6)]
    pub creator_id: u64,

    /// The difficulty of this `PartialLevel`
    ///
    /// ## GD Internals:
    /// This value is a construct from the value at the indices `9`, `17` and `25`, whereas index 9
    /// is an integer representation of either the [LevelRating](struct.LevelRating.html) or the
    /// [DemonRating](struct.DemonRating.html) struct, depending on the value of index 17.
    ///
    /// If index 25 is set to true, the level is an auto level and the value at index 9 is some nonsense,
    /// in which case it is ignored.
    #[raw_data(custom = "de::level_rating")]
    pub difficulty: LevelRating,

    #[raw_data(index = 10)]
    /// The amount of downloads
    ///
    /// ## GD Internals:
    /// This value is provided at index `10`
    pub downloads: u32,

    /// The `MainSong` the level uses, if any.
    ///
    /// ## GD Internals:
    /// This value is provided at index `12`. Interpretation is additionally dependant on the value
    /// at index `35` (the custom song id), as without that information, a value of `0` for this
    /// field could either mean the level uses `Stereo Madness` or no main song.
    #[raw_data(custom = "de::main_song")]
    pub main_song: Option<&'static MainSong>,

    /// The gd version the request was uploaded/last updated in.
    ///
    /// ## GD Internals:
    /// This value is provided at index `13`
    #[raw_data(index = 13)]
    pub gd_version: GameVersion,

    /// The amount of likes this `PartialLevel` has received
    ///
    /// ## GD Internals:
    /// This value is provided at index `14`
    #[raw_data(index = 14)]
    pub likes: i32,

    /// The length of this `PartialLevel`
    ///
    /// ## GD Internals:
    /// This value is provided as an integer representation of the [LevelLength](struct.LevelLength.html)
    /// struct at index `15`
    #[raw_data(index = 15)]
    pub length: LevelLength,

    /// The amount of stars completion of this `PartialLevel` awards
    ///
    /// ## GD Internals:
    /// This value is provided at index `18`
    #[raw_data(index = 18)]
    pub stars: u8,

    /// This `PartialLevel`s featured state
    ///
    /// ## GD Internals:
    /// This value is provided at index `19`
    #[raw_data(index = 19)]
    pub featured: Featured,

    /// The ID of the level this `PartialLevel` is a copy of, or `None`, if this `PartialLevel`
    /// isn't a copy.
    ///
    /// ## GD Internals:
    /// This value is provided at index `30`
    #[raw_data(index = 30, deserialize_with = "de::default_to_none")]
    pub copy_of: Option<u64>,

    /// The id of the newgrounds song this `PartialLevel` uses, or `None` if it useds a main song.
    ///
    /// ## GD Internals:
    /// This value is provided at index `35`, and a value of `0` means, that no custom song is used.
    #[raw_data(index = 35, deserialize_with = "de::default_to_none")]
    pub custom_song_id: Option<u64>,

    /// The amount of coints in this `PartialLevel`
    ///
    /// ## GD Internals:
    /// This value is provided at index `37`
    #[raw_data(index = 37)]
    pub coin_amount: u8,

    #[raw_data(index = 38)]
    pub index_38: String,

    /// The amount of stars the level creator has requested when uploading this `PartialLevel`,
    /// or `None` if no stars were requested.
    ///
    /// ## GD Internals:
    /// This value is provided at index `39`, and a value of `0` means no stars were requested
    #[raw_data(index = 39, deserialize_with = "de::default_to_none")]
    pub stars_requested: Option<u8>,

    /// Value indicating whether this `PartialLevel` is epic
    ///
    /// ## GD Internals:
    /// This value is provided at index `42`, as an integer
    #[raw_data(index = 42, deserialize_with = "de::int_to_bool")]
    pub is_epic: bool,

    #[raw_data(index = 43)]
    pub index_43: String,

    /// The amount of objects in this `PartialLevel`
    ///
    /// ## GD Internals:-
    /// This value is provided at index `45`, although only for levels uploaded in version
    /// 2.1 or later. For all older levels this is always `0`
    #[raw_data(index = 45)]
    pub object_amount: u32,

    #[raw_data(index = 46, default)]
    pub index_46: String,

    #[raw_data(index = 47, default)]
    pub index_47: String,
}

#[derive(Debug, FromRawObject)]
#[cfg_attr(feature = "deser", derive(Serialize, Deserialize))]
pub struct Level {
    /// The `PartialLevel` this `Level` instance supplements
    #[raw_data(flatten)]
    pub base: PartialLevel,

    /// The raw level data
    ///
    /// ## GD Internals:
    /// This value is provided at index `4`, and is urlsafe base64 encoded and DEFLATE
    /// compressed
    #[raw_data(index = 4)]
    pub level_data: String,

    /// The request's password
    ///
    /// ## GD Internals:
    /// This value is provided at index `27`, and "encrypted" using robtop's XOR routine with key <TODO: key>
    #[raw_data(index = 27)]
    pub password: String,

    /// The time passed since the `Level` was uploaded
    ///
    /// ## GD Internals:
    /// This value is provided at index `28`
    #[raw_data(index = 28)]
    pub time_since_upload: String,

    /// The time passed since this `Level` was last updated
    ///
    /// ## GD Internals:
    /// This value is provided at index `29`
    #[raw_data(index = 29)]
    pub time_since_update: String,

    #[raw_data(index = 36, default)]
    pub index_36: String,
}

impl Display for PartialLevel {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "PartialLevel({}, {})", self.level_id, self.name)
    }
}

impl Display for Level {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "Level({}, {})", self.base.level_id, self.base.name)
    }
}