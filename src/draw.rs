use plotters::coord::types::RangedCoordf32;
use plotters::prelude::*;

pub fn draw_graph(
    mut points: Vec<(f32, f32, &str)>,
    size: (u32, u32),
) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("results/result.png", size).into_drawing_area();

    root.fill(&WHITE)?;

    let root = root.apply_coord_spec(Cartesian2d::<RangedCoordf32, RangedCoordf32>::new(
        0f32..1f32,
        1f32..0f32,
        (0..(size.0 as i32), 0..(size.1 as i32)),
    ));

    let dot_and_label = |x: f32, y: f32, name: &str| {

        return EmptyElement::at((x, y))
            + Circle::new((0, 0), 3, ShapeStyle::from(&BLACK).filled())
            + Text::new(
                format!("({:.2},{:.2})", x, y),
                (10, 0),
                ("sans-serif", 15.0).into_font(),
            ) + Text::new(
                name.to_string(),
                (-5, -15),
                ("sans-serif", 15.0).into_font(),
            );
    };
    let lines = |points: Vec<(f32, f32)>| {
        return PathElement::new(points, ShapeStyle::from(&BLACK).filled());
    };

    let mut points: Vec<(f32, f32, &str)> = points
        .iter_mut()
        .map(|point| {
            return match (point.0, point.1) {
                (0.0, 0.0) => (0.01, 0.01, point.2),
                _ => *point,
            };
        })
        .collect();
    points.sort_by(|a, b| a.0.total_cmp(&b.0).then(a.1.total_cmp(&b.1)));

    points.iter_mut().for_each(|point| {
        let _ = root.draw(&dot_and_label(point.0, point.1, point.2));
    });

    root.draw(&lines(points.into_iter().map(|f| (f.0, f.1)).collect()))?;
    root.present()?;

    Ok(())
}
