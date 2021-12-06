use clap::{value_t, App, Arg};
use drawing::{draw_palette, EmojiColor};
use emoji_store::{EmojiFS, EmojiSpec, EmojiStore};
use std::path::PathBuf;

mod drawing;
mod emoji_store;
// TODO: Scrape skin tone emoji list https://www.unicode.org/emoji/charts/full-emoji-modifiers.html

fn main() {
    let matches = App::new("emoji-palette")
        .author("Joshua O'Connor <joshua@joshuao.com>")
        .about("Creates a color palette based on emojis, based on a TikTok trend")
        .arg(
            Arg::with_name("emoji")
                .help("3 Emojis from which to create the palette")
                .multiple(true)
                .number_of_values(3)
                .required(true),
        )
        .arg(
            Arg::with_name("platform")
                .help("The platform from whose emoji set to create the pallette")
                .default_value("twitter")
                .short("p")
                .long("platform")
                .takes_value(true)
                .possible_values(&[
                    "twitter", "apple", "facebook", "google", "samsung", "windows",
                ]),
        )
        .arg(
            Arg::with_name("root_dir")
                .help("The root dir when using the filesystem emoji storage")
                .default_value(".")
                .short("r")
                .long("root-dir")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("output_filename")
                .help("Filename of output png")
                .default_value("emoji_palette.png")
                .short("o")
                .long("output")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("image_size")
                .help("Size (in pixels) of square output image")
                .short("s")
                .long("size")
                .default_value("1000")
                .takes_value(true),
        )
        .get_matches();
    // TODO: If I add any more backends, create mutually exclusive arguments

    // Safe to unwrap since it has a default, it would be neat if that could be typechecked
    let platform = matches.value_of("platform").unwrap();
    let output_name = matches.value_of("output_filename").unwrap();
    let size = value_t!(matches, "image_size", u32).unwrap();

    let root_dir = matches.value_of("root_dir").unwrap();
    let emoji_store = EmojiFS {
        root: PathBuf::from(root_dir),
    };

    let emoji_colors: Vec<EmojiColor> = matches
        .values_of("emoji")
        .unwrap()
        .map(|s| EmojiSpec {
            code: s.to_owned(),
            platform: platform.to_owned(), // TODO:  Use parameterised version
        })
        .map(|x| emoji_store.emoji_image(x).unwrap())
        .map(EmojiColor::from)
        .collect();

    let canvas = draw_palette(size, &emoji_colors[0], &emoji_colors[1], &emoji_colors[2]);
    canvas.save(output_name).unwrap();
}

// TODO: Create terminal output for terminals with realcolor
// TODO: Option to use font (system font as default), will have to introspect ttf
// TODO: Android app?
// TODO: Create structure at compilation time of image colors to code points based on folder names
// TODO: Create commandline arg to override compiled map and use folders
// TODO: Write some tests
