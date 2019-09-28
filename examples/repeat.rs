use raqote::*;

fn main() {
    let mut dt = DrawTarget::new(200, 200);

    let mut pb = PathBuilder::new();
    pb.rect(20.0, 20.0, 160.0, 160.0);
    let path = pb.finish();

    let mut gradient = Source::new_linear_gradient(
        Gradient {
            stops: vec![
                GradientStop { position: 0.0, color: Color::new(0xff, 0xff, 0xff, 0xff) },
                GradientStop { position: 1.0, color: Color::new(0xff, 0, 0, 0) },
            ],
        },
        Point::new(0.0, 0.0),
        Point::new(0.2, 0.0),
        Spread::Repeat,
    );

    if let raqote::Source::LinearGradient(_, _, ref mut transform) = gradient {
        let ts = Transform::row_major(160.0, 0.0, 0.0, 160.0, 20.0, 20.0).inverse().unwrap();
        *transform = transform.pre_transform(&ts);
    }

    dt.fill(&path, &gradient, &DrawOptions::new());

    dt.write_png("out.png").unwrap();
}