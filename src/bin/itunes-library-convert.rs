use std::path::PathBuf;

use plist_tools::encode_as_latin1;

fn main() {
    env_logger::init();

    let mut args = std::env::args();
    let prog_name = args.next();
    let source_path = args.next().map(|s| PathBuf::from(s));
    let dest_path = args.next().map(|s| PathBuf::from(s));

    if let (Some(src), Some(dst)) = (source_path, dest_path) {
        encode_as_latin1(&src, &dst);
    } else {
        println!("Usage: {} <your iTunes Library.xml> <converted iTunes Library.xml>",
            prog_name.unwrap_or(String::from("<prog_name>"))
        );
        println!("");
        println!("  This tool converts your XML iTunes Library into a format");
        println!("  where Traktor understands special chars. This works around");
        println!("  a bug discussed at");
        println!("  https://community.native-instruments.com/discussion/5526/bug-in-tp3-3-6-0-325-duplicate-entries-in-collection/p4");
    }
}
