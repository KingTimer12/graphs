use draw::draw_graph;

mod draw;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    draw_graph(vec![
        (0.0, 0.0, "A"),
        (0.25, 0.33, "B"),
        (0.93, 0.9, "C"),
        (0.5, 0.6, "D"),
        (0.8, 0.8, "E")
    ], (800, 700))?;

    Ok(())
}