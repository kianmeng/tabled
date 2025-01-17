//! The example can be run by this command
//! `cargo run --example formatting_settings`

use tabled::{
    formatting_settings::{AlignmentStrategy, TrimStrategy},
    object::Segment,
    Modify, Style, TableIteratorExt,
};

fn main() {
    let some_json = r#"
[
    "foo",
    {
        "bar": 1,
        "baz": [
            2,
            3
        ]
    }
]"#;

    let data = [some_json];
    let table = data.table().with(Style::rounded());

    println!("A default Alignment settings");
    println!("{}", table);

    let table = table.with(Modify::new(Segment::all()).with(AlignmentStrategy::PerLine));

    println!("Per line Alignment strategy");
    println!("{}", table);

    let table = table.with(
        Modify::new(Segment::all())
            .with(AlignmentStrategy::PerCell)
            .with(TrimStrategy::Both),
    );

    println!("A default Alignment; allowing vertical and horizontal trim");
    println!("{}", table);
}
