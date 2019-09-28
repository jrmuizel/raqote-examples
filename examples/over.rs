use raqote::{DrawOptions, DrawTarget, PathBuilder, SolidSource, Source};

fn main() {
    let mut draw_target = DrawTarget::new(100, 100);

    draw_target.clear(SolidSource {
        r: 0xff,
        g: 0xff,
        b: 0xff,
        a: 0xff,
    });

    let source = Source::Solid(SolidSource {
        r: 0x00,
        g: 0x00,
        b: 0x00,
        a: 0xff,
    });

    for row in 0..100 {
        for col in 0..1 {
            let mut pb = PathBuilder::new();
            pb.rect(0.0 * 1. as f32, 0.0 * 1. as f32, 1.0, 1.0);
            let path = pb.finish();
            draw_target.fill(&path, &source, &DrawOptions::default());
        }
    }

    draw_target.write_png("out.png").unwrap();
}
