fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me");
    let first = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt { part: first };
}
