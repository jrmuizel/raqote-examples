use raqote::*;

fn main() {
    let mut dt = DrawTarget::new(400, 400);

    let mut pb = PathBuilder::new();
/*    pb.rect(100., 100., 100., 100.);

    let path = pb.finish();

    dt.stroke(
        &path,
        &Source::Solid(SolidSource {
            r: 0x0,
            g: 0x0,
            b: 0x80,
            a: 0x80,
        }),
        &StrokeStyle {
            cap: LineCap::Square,
            join: LineJoin::Round,
            width: 10.,
            miter_limit: 2.,
            dash_array: vec![10., 18.],
            dash_offset: 16.,
        },
        &DrawOptions::new(),
    );*/

    pb.rect(40., 40., 120., 120.);
    dt.stroke(
        &pb.finish(),
        &Source::Solid(SolidSource {
            r: 0,
            g: 0,
            b: 0,
            a: 0xff,
        }),
        &StrokeStyle {
            width: 1.,
            dash_array: vec![30., 10., 60.],
            dash_offset: 0.5,
            ..Default::default()
        },
        &DrawOptions::new(),
    );

    dt.write_png("example.png");
}
