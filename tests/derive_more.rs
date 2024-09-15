use derive_more::Display;

#[derive(Display)]
#[display("{_variant}")]
enum SortBy {
    Name,
    Date,
}

#[test]
fn display() {
    assert_eq!(SortBy::Date.to_string(), "Date");
}

#[derive(Display)]
struct Outer(SortBy);

#[test]
fn display_outer() {
    let outer = Outer(SortBy::Name);
    assert_eq!(outer.to_string(), "Name");
}
