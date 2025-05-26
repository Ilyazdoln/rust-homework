#[derive(Debug)]
struct Rectangle {
    x: usize,
    y: usize,
    width: usize,
    height: usize,
}

use std::collections::HashSet;

pub fn occupied_area(rects: &[Rectangle]) -> usize {
    let mut points = HashSet::new();

    for rect in rects {
        for i in rect.x..rect.x + rect.width {
            for j in rect.y..rect.y + rect.height {
                points.insert((i, j));
            }
        }
    }
    points.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_area() {
        let rects = vec![
            Rectangle { x: 0, y: 0, width: 3, height: 2 },
            Rectangle { x: 2, y: 1, width: 3, height: 3 },
        ];
        assert_eq!(occupied_area(&rects), 12);
    }
}