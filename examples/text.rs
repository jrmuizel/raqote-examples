use font_kit::family_name::FamilyName;
use font_kit::properties::{Properties, Weight};
use font_kit::source::SystemSource;
use raqote::*;

fn main() {
    let mut dt = DrawTarget::new(200, 100);

    let font = SystemSource::new()
        .select_best_match(
            &[FamilyName::Title("Roboto".into())],
            &Properties::new().weight(Weight::MEDIUM),
        )
        .unwrap()
        .load()
        .unwrap();
    println!("{:?}", font);

    let mut pb = PathBuilder::new();
    pb.rect(10., 10., 50., 50.);
    dt.fill(
        &pb.finish(),
        &Source::Solid(SolidSource {
            r: 0xff,
            g: 0xff,
            b: 0xff,
            a: 0xff,
        }),
        &DrawOptions::new(),
    );
    dt.draw_text(
        &font,
        24.,
        "Toggle Button",
        Point::new(30., 30.),
        &Source::Solid(SolidSource {
            r: 0,
            g: 0,
            b: 0xff,
            a: 0xff,
        }),
        &DrawOptions::new(),
    );

    dt.write_png("out.png").unwrap();
}
