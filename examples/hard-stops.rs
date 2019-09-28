use raqote::*;

fn main() {
    let mut dt = DrawTarget::new(200, 200);

    let gradient = Source::new_linear_gradient(
        Gradient {
            stops: vec![
                GradientStop {
                    position: 0.0,
                    color: Color::new(0xff, 0xff, 0xff, 0xff),
                },
                GradientStop {
                    position: 0.003,
                    color: Color::new(0xff, 0, 0, 0),
                },
                GradientStop {
                    position: 1.0,
                    color: Color::new(0xff, 0, 0, 0),
                },
            ],
        },
        Point::new(40., 0.),
        Point::new(100., 0.),
        Spread::Pad,
    );

    let mut pb = PathBuilder::new();
    pb.rect(0., 0., 80., 80.);
    let path = pb.finish();
    dt.fill(&path, &gradient, &DrawOptions::default());

    dt.write_png("out.png").unwrap();
}