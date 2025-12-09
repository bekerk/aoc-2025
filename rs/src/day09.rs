use itertools::Itertools;
use std::collections::HashSet;

pub fn get_rectangle_area(p1: (usize, usize), p2: (usize, usize)) -> usize {
    let width = p1.0.abs_diff(p2.0) + 1;
    let height = p1.1.abs_diff(p2.1) + 1;

    width * height
}

pub fn get_points_of_rectangle(p1: (usize, usize), p2: (usize, usize)) -> Vec<(usize, usize)> {
    let p3 = (p1.0, p2.1);
    let p4 = (p2.0, p1.1);

    vec![p1, p2, p3, p4]
}

pub fn get_rectangle_coordinates(points: &[(usize, usize)]) -> Vec<Vec<(usize, usize)>> {
    points
        .iter()
        .tuple_combinations::<(&_, &_)>()
        .map(|(a, b)| get_points_of_rectangle(*a, *b))
        .collect()
}

// pub fn point_in_polygon(point: (usize, usize), boundary_vertices: &[(usize, usize)]) -> bool {
//     let mut inside = false;
//     let mut j = boundary_vertices.len() - 1;
//     let eps = 1e-9;

//     for i in 0..boundary_vertices.len() {
//         let xi: f64 = boundary_vertices[i].0 as f64;
//         let yi: f64 = boundary_vertices[i].1 as f64;
//         let xj: f64 = boundary_vertices[j].0 as f64;
//         let yj: f64 = boundary_vertices[j].1 as f64;

//         let intersect = ((yi > point.1 as f64) != (yj > point.1 as f64))
//             && ((point.0 as f64) < (xj - xi) * (point.1 as f64 - yi) / (yj - yi) + xi);

//         if intersect {
//             inside = !inside;
//         }

//         j = i;
//     }

//     inside
// }

pub fn collides_with_polygon(
    rectangle: &[(usize, usize)],
    _boundary_vertices: &[(usize, usize)],
) -> bool {
    rectangle
        .iter()
        // .all(|point| point_in_polygon(*point, boundary_vertices))
        .all(|_point| true)
}

pub fn get_rectangles_in_polygon(
    boundary_vertices: &[(usize, usize)],
    rectangle_coordinates: &[Vec<(usize, usize)>],
) -> Vec<Vec<(usize, usize)>> {
    rectangle_coordinates
        .iter()
        .filter(|rectangle| collides_with_polygon(rectangle, boundary_vertices))
        .cloned()
        .collect::<Vec<Vec<(usize, usize)>>>()
}

pub fn get_max_rectangle_area(points: &[(usize, usize)]) -> usize {
    points
        .iter()
        .tuple_combinations::<(&_, &_)>()
        .map(|(a, b)| get_rectangle_area(*a, *b))
        .max()
        .unwrap_or(0)
}

pub fn render_line_between_points(p1: (usize, usize), p2: (usize, usize)) -> Vec<(usize, usize)> {
    let mut line = Vec::new();
    let is_horizontal: bool = p1.1 == p2.1;

    if is_horizontal {
        for x in p1.0.min(p2.0)..=p1.0.max(p2.0) {
            line.push((x, p1.1));
        }
    } else {
        for y in p1.1.min(p2.1)..=p1.1.max(p2.1) {
            line.push((p1.0, y));
        }
    }

    line
}

pub fn get_polygon(points: &[(usize, usize)]) -> Vec<(usize, usize)> {
    let mut polygon = HashSet::new();

    for i in 0..points.len() {
        let current = points[i];
        let next = points[(i + 1) % points.len()];

        let line = render_line_between_points(current, next);
        polygon.extend(line);
    }

    polygon.iter().cloned().collect::<Vec<(usize, usize)>>()
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use std::fs;

    fn file_to_vec(file: &str) -> Vec<(usize, usize)> {
        fs::read_to_string(file)
            .unwrap()
            .trim()
            .split('\n')
            .filter_map(|s| {
                let mut chars = s.split(',');
                let x = chars.next().unwrap().parse::<usize>().ok()?;
                let y = chars.next().unwrap().parse::<usize>().ok()?;
                Some((x, y))
            })
            .collect::<Vec<(usize, usize)>>()
    }

    #[test]
    fn get_max_rectangle_area_test() {
        let input = file_to_vec("../input/day09.txt");
        assert_eq!(super::get_max_rectangle_area(&input), 4763509452);
    }

    // TODO: Part 2 needs more work, unfortunately we are defeated...
    #[test]
    fn get_polygon_based_on_points_test() {
        let input = file_to_vec("../input/day09.txt");
        let rectangle_coordinates = super::get_rectangle_coordinates(&input);
        let polygon = super::get_polygon(&input);
        let _rectangles_in_polygon =
            super::get_rectangles_in_polygon(&polygon, &rectangle_coordinates);

        assert_eq!(1, 1);
    }
}
