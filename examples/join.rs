use raqote::*;

fn main() {
    let mut dt = DrawTarget::new(400, 400);

    let mut pb = PathBuilder::new();

    pb.move_to(30.0, 40.);
    pb.cubic_to(16., 137., 171., 45., 100., 90.);
    pb.cubic_to(29., 135., 171., 45., 180., 155.);
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
            cap: LineCap::Round,
            join: LineJoin::Miter,
            width: 5.,
            miter_limit: 4.,
            dash_array: Vec::new(),
            dash_offset: 0.,
        },
        &DrawOptions::new(),
    );

    dt.write_png("example.png");
}
