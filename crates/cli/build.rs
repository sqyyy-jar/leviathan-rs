use chrono::Datelike;

fn main() {
    let now = chrono::Utc::now();
    println!(
        "cargo:rustc-env=BUILD_DATE={}-{:02}-{:02}",
        now.year(),
        now.month(),
        now.day()
    );
}
