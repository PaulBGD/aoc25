use geo::{coord, Contains, LineString, Polygon, Rect};
use itertools::Itertools;

#[aoc::main(09)]
fn main(input: &str) -> (usize, usize) {
    let xs = input
        .split('\n')
        .map(|l| {
            let (one, two) = l.split_once(',').unwrap();
            return (one.parse::<f64>().unwrap(), two.parse::<f64>().unwrap());
        })
        .collect::<Vec<_>>();

    let largest = xs
        .iter()
        .combinations(2)
        .map(|vec| {
            let (x1, y1) = *vec[0];
            let (x2, y2) = *vec[1];

            let size = (((x1.max(x2)) - x1.min(x2)) + 1.0) * ((y1.max(y2) - y1.min(y2)) + 1.0);

            return size;
        })
        .sorted_by(|a, b| a.partial_cmp(b).unwrap())
        .rev()
        .nth(0)
        .unwrap();

    let line_str = LineString::new(
        xs.iter()
            .enumerate()
            .flat_map(|(index, coords)| {
                let next_coords = if index == xs.len() - 1 {
                    xs[0]
                } else {
                    xs[index + 1]
                };

                return vec![
                    coord! { x: coords.0, y: coords.1 },
                    coord! { x: next_coords.0, y: next_coords.1 },
                ];
            })
            .collect_vec(),
    );

    let polygon = Polygon::new(line_str, vec![]);

    let largest_p2 = xs
        .iter()
        .combinations(2)
        .map(|vec| {
            let (x1, y1) = *vec[0];
            let (x2, y2) = *vec[1];

            let rect = Rect::new(
                coord! { x: x1, y: y1 },
                coord! { x: x2, y: y2 },
            );

            if !polygon.contains(&rect) {
                return 0;
            }

            return (rect.width() as i64 + 1) * (rect.height() as i64 + 1);
        })
        .sorted()
        .rev()
        .nth(0)
        .unwrap();

    (largest as usize, largest_p2 as usize)
}
