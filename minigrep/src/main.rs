use minigrep::{ Config, run, fatal };

fn main() {
    let config = Config::from_args().unwrap_or_else(|err| {
        fatal(err);
    });

    if let Err(err) = run(config) {
        fatal(&err.to_string());
    }
}
