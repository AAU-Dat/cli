fn main() {
    if let Err(e) = aau::get_args().and_then(aau::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
