use crate::{
    convert::{Base64BytesConverter, Base64Converter, RobtopFrom, RobtopInto},
    error::ValueError,
    Parse,
};
use gdcf_model::{
    level::{DemonRating, Level, LevelRating, PartialLevel},
    song::{MainSong, MAIN_SONGS, UNKNOWN},
};

pub mod data;
pub mod object;

pub fn process_difficulty(rating: &str, is_auto: bool, is_demon: bool) -> LevelRating {
    if is_demon {
        LevelRating::Demon(DemonRating::robtop_from(rating).unwrap()) // FIXME: make custom
                                                                      // functions return result
    } else if is_auto {
        LevelRating::Auto
    } else {
        LevelRating::robtop_from(rating).unwrap()
    }
}

pub fn process_song(main_song: usize, custom_song: &Option<u64>) -> Option<&'static MainSong> {
    if custom_song.is_none() {
        Some(MAIN_SONGS.get(main_song).unwrap_or(&UNKNOWN))
    } else {
        None
    }
}

parser! {
    PartialLevel<Option<u64>, u64> => {
        level_id(index = 1),
        name(index = 2),
        description(index = 3, parse_infallible = Base64Converter, default),
        version(index = 5),
        creator(index = 6),
        difficulty(custom = process_difficulty[rating, is_auto, is_demon]),
        downloads(index = 10),
        main_song(custom = process_song[main_song_id, &custom_song]),
        gd_version(index = 13),
        likes(index = 14),
        length(index = 15),
        stars(index = 18),
        featured(index = 19),
        copy_of(index = 30),
        index_31(index = 31),
        custom_song(index = 35),
        coin_amount(index = 37),
        coins_verified(index = 38),
        stars_requested(index = 39),
        index_40(index = 40, optional),
        is_epic(index = 42),
        index_43(index = 43),
        object_amount(index = 45),
        index_46(index = 46, default),
        index_47(index = 47, default),
    },
    main_song_id(index = 12, extract = extract_main_song_id[main_song], default),
    rating(index = 9, extract = extract_rating[difficulty]),
    is_demon(index = 17, extract = extract_is_demon[difficulty], default),
    is_auto(index = 25, extract = extract_is_auto[difficulty], default),
    is_na(index = 8, ignore, extract = extract_is_na[difficulty]),
}

fn extract_main_song_id(main_song: Option<&'static MainSong>) -> String {
    main_song.map(|s| s.main_song_id).unwrap_or_default().robtop_into()
}

fn extract_rating(rating: LevelRating) -> String {
    rating.robtop_into()
}

fn extract_is_demon(rating: LevelRating) -> String {
    RobtopInto::<bool, String>::robtop_into(match rating {
        LevelRating::Demon(_) => true,
        _ => false,
    })
}

fn extract_is_auto(rating: LevelRating) -> String {
    RobtopInto::<bool, String>::robtop_into(rating == LevelRating::Auto)
}

fn extract_is_na(rating: LevelRating) -> String {
    RobtopInto::<bool, String>::robtop_into(rating == LevelRating::NotAvailable)
}

parser! {
    Level<Option<u64>, u64> => {
        base(delegate),
        level_data(index = 4, parse = Base64BytesConverter),
        password(index = 27),
        time_since_upload(index = 28),
        time_since_update(index = 29),
        index_36(index = 36, default),
    }
}