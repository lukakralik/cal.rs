fn main() {
    if let Err(e) = calrs::get_args().and_then(calrs::run) {
        eprintln!("{}", e);
        std::process::exti(1);
    }
}
