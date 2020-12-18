use clap::*;
use rddit_framework_2;
fn main() {
    let matches = App::new("r-ddit")
        .version("2.0.0")
        .author("Johannes T. <github.com/JohannesThoren>")
        .about("a wors reddit in the terminal")
        .arg(
            Arg::with_name("subreddit")
                .short("S")
                .long("sub")
                .takes_value(true)
                .value_name("subreddit")
                .help("sets the subreddit you whant to pull data from"),
        )
        .arg(
            Arg::with_name("count")
                .short("c")
                .long("count")
                .takes_value(true)
                .value_name("count")
                .help("amount of images that you want to download"),
        )
        .arg(
            Arg::with_name("limit")
                .short("l")
                .long("limit")
                .takes_value(true)
                .value_name("limit")
                .help("sets a limit for how many posts that you can index"),
        )
        .arg(
            Arg::with_name("sorting")
                .short("s")
                .long("sorting")
                .takes_value(true)
                .value_name("sorting")
                .help("sets the sorting method"),
        )
        .arg(
            Arg::with_name("timespan")
                .short("t")
                .long("timespan")
                .takes_value(true)
                .value_name("timespan")
                .help("sets the timespan for sorting"),
        )
        .arg(
            Arg::with_name("file destination")
                .short("f")
                .long("file_destination")
                .takes_value(true)
                .value_name("file_destination")
                .help("sets the file destination"),
        )
        .arg(
            Arg::with_name("Text")
                .short("T")
                .long("Text")
                .help("gets the so called self text"),
        )
        .arg(
            Arg::with_name("img")
                .short("i")
                .long("img")
                .help("gets post images"),
        )
        .get_matches();

    let start_time = std::time::SystemTime::now();
    let mut settings = rddit_framework_2::url_handler::Settings::new();

    let mut count = 1;
    let mut destination = String::new();

    let mut img = false;
    let mut text = false;

    if matches.is_present("subreddit") {
        settings.subreddit = String::from(matches.value_of("subreddit").unwrap());
    }
    if matches.is_present("count") {
        count = matches.value_of("count").unwrap().parse().unwrap();
    }
    if matches.is_present("limit") {
        settings.limit = matches.value_of("limit").unwrap().parse().unwrap();
    }
    if matches.is_present("sorting") {
        settings.sorting = String::from(matches.value_of("sorting").unwrap());
    }
    if matches.is_present("timespan") {
        settings.timespan = String::from(matches.value_of("timespan").unwrap());
    }
    if matches.is_present("file destination") {
        destination = String::from(matches.value_of("file destination").unwrap());
    }
    if matches.is_present("img") {
        img = true
    }

    if matches.is_present("Text") {
        text = true
    }

    let posts = rddit_framework_2::post_handler::get_all_post_data(&mut settings);

    if img {
        let imgs = rddit_framework_2::download_handler::get_images(count, &posts);
        rddit_framework_2::download_handler::download_imgs(&imgs, &destination);
    }

    if text {
        rddit_framework_2::download_handler::download_text(count, &destination, &posts)
    }

    let elapsed_time = std::time::SystemTime::now()
        .duration_since(start_time)
        .unwrap()
        .as_secs_f64();
    println!("\n\n\nelapsed time : {:.3}s", elapsed_time);
}
