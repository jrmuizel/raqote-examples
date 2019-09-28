use raqote::*;

fn main() {
    let mut dt = DrawTarget::new(200, 200);

    let mut pb = PathBuilder::new();
    pb.rect(20.0, 20.0, 160.0, 160.0);
    let path = pb.finish();

    let mut gradient = Source::new_two_circle_radial_gradient(
        Gradient {
            stops: vec![
                GradientStop {
                    position: 0.0,
                    color: Color::new(0xff, 0xff, 0xff, 0xff),
                },
                GradientStop {
                    position: 1.0,
                    color: Color::new(0xff, 0, 0, 0),
                },
            ],
        },
        Point::new(100., 100.),
        0.0,
        Point::new(100., 100.),
        80.,
        Spread::Repeat,
    );


/*
    let mut gradient = Source::new_radial_gradient(
        Gradient {
            stops: vec![
                GradientStop {
                    position: 0.0,
                    color: 0xffffffff,
                },
                GradientStop {
                    position: 1.0,
                    color: 0xff000000,
                },
            ],
        },
        Point::new(100., 100.),
        80.,
        Spread::Repeat,
    );
*/
    dt.fill(&path, &gradient, &DrawOptions::new());

    dt.write_png("out.png").unwrap();
}