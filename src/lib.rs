use std::path::Path;

use plist::Value;
use percent_encoding::{percent_decode_str, percent_encode, AsciiSet};
use encoding_rs::ISO_8859_15;

/// Parses the plist from `src`, and write it at `dst` with latin-1 characters
pub fn encode_as_latin1(src: &Path, dst: &Path) {
    log::info!("Parsing the input ({:?})...", src);
    let mut root = plist::Value::from_file(src).unwrap();
    let root_dict = root.as_dictionary_mut().unwrap();
    log::info!("Done");

    log::info!("Converting paths...");
    let plist_ascii_set = percent_encoding::CONTROLS;
    let plist_ascii_set = plist_ascii_set.add(b' ');
    let plist_ascii_set = Box::new(plist_ascii_set);
    let static_plist_ascii_set = Box::leak(plist_ascii_set);

    let mut tracks = root_dict.get_mut("Tracks").unwrap().as_dictionary_mut().unwrap();
    convert_paths(&mut tracks, static_plist_ascii_set);
    log::info!("Done");

    log::info!("Saving to {:?}...", dst);
    root.to_file_xml(dst).unwrap();
    log::info!("Done");
}

fn convert_paths(tracks: &mut plist::Dictionary, plist_ascii_set: &'static AsciiSet) {
    for (_, track) in tracks {
        let track_dict = match track.as_dictionary_mut() {
            Some(dict) => dict,
            None => {
                log::warn!("Unexpected non-dict type");
                continue;
            }
        };

        let track_display_name = track_dict
            .get("Name")
            .and_then(|val| val.as_string())
            .unwrap_or("<invalid track name>")
            .to_string();

        // Grab the location
        // e.g. "file://localhost/C:/Users/J%C3%A9r%C3%B4me/Music/iTunes/iTunes%20Media/Music/..."
        let location_from_xml = match track_dict.get("Location").map(|l| l.as_string()) {
            Some(Some(l)) => l,
            Some(None) => {
                log::warn!("Track {} location is not a string!", track_display_name);
                continue;
            },
            None => {
                log::warn!("Track {} has no valid location!", track_display_name);
                continue;
            },
        };

        // Decode as UTF8
        let location_from_utf8 = match percent_decode_str(location_from_xml).decode_utf8() {
            Err(err) => {
                log::warn!("Track {} has invalid location: {} ({})", track_display_name, location_from_xml, err);
                continue;
            },
            Ok(cow) => cow,
        };

        // Encode to Latin-15
        let (location_as_latin15, _encoding_used, errors) = ISO_8859_15.encode(&location_from_utf8);
        if errors {
            log::warn!("Track {} location ({}) could not be converted to Latin-1", track_display_name, location_from_utf8);
            continue;
        }

        // URL escape and put back in the PLIST
        let location_as_escaped_latin15 = percent_encode(&location_as_latin15, plist_ascii_set).to_string();
        track_dict.insert(String::from("Location"), Value::String(location_as_escaped_latin15));

        // log::debug!("Track {}", track_display_name);
        // log::debug!("  XML input:      {}", location_from_xml);
        // log::debug!("  decoded string: {}", location_from_utf8);
        // log::debug!("  final encoding: {}", location_as_escaped_latin15);
    }
}
