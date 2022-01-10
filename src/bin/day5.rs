use std::{
    cmp,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Copy, Clone, PartialEq, Debug)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
struct LineSegment {
    start: Point,
    end: Point,
}

impl LineSegment {
    fn new(start: Point, end: Point) -> Self {
        Self { start, end }
    }

    fn is_vertical(&self) -> bool {
        self.start.x == self.end.x
    }

    fn is_horizontal(&self) -> bool {
        self.start.y == self.end.y
    }

    fn is_diagonal(&self) -> bool {
        !self.is_vertical() && !self.is_horizontal()
    }

    fn points(&self) -> Vec<Point> {
        if self.is_vertical() {
            let x = self.start.x;
            let y = cmp::min(self.start.y, self.end.y);
            let y_end = cmp::max(self.start.y, self.end.y);

            return (y..=y_end).map(|y| Point::new(x, y)).collect();
        } else if self.is_horizontal() {
            let x = cmp::min(self.start.x, self.end.x);
            let x_end = cmp::max(self.start.x, self.end.x);
            let y = self.start.y;

            return (x..=x_end).map(|x| Point::new(x, y)).collect();
        }

        let mut x = self.start.x as isize;
        let mut y = self.start.y as isize;
        let x_step = if self.end.x > self.start.x { 1 } else { -1 };
        let y_step = if self.end.y > self.start.y { 1 } else { -1 };

        let capacity = (self.start.x as isize - self.end.x as isize).abs() as usize + 1;
        let mut points = Vec::with_capacity(capacity);

        while points.len() < capacity {
            points.push(Point::new(x as usize, y as usize));
            x += x_step;
            y += y_step;
        }

        points
    }
}

struct Diagram {
    points: Vec<Vec<usize>>,
}

impl Diagram {
    fn new(width: usize, height: usize) -> Self {
        Self {
            points: vec![vec![0; height]; width],
        }
    }

    fn add_line_segment(&mut self, line_segment: &LineSegment, allow_diagonals: bool) {
        if !allow_diagonals && line_segment.is_diagonal() {
            return;
        }

        // println!("{},{} -> {},{}", line_segment.start.x, line_segment.start.y, line_segment.end.x, line_segment.end.y);

        for point in &line_segment.points() {
            self.points[point.x][point.y] += 1;
        }
    }

    fn count_points_where_lines_overlap(&self) -> usize {
        let mut count = 0;

        for col in &self.points {
            count += col.iter().filter(|&sum| *sum > 1).count();
        }

        count
    }
}

fn main() {
    let input_file = File::open("./data/day5.txt").unwrap();
    let reader = BufReader::new(input_file);
    let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();
    let line_segments = read_line_segments(lines);

    let width = get_max_x_coord(&line_segments) + 1;
    let height = get_max_y_coord(&line_segments) + 1;

    let mut diagram = Diagram::new(width, height);

    for line_segment in &line_segments {
        diagram.add_line_segment(line_segment, false);
    }

    println!(
        "At {} points do at least two vertical or horizontal lines overlap",
        diagram.count_points_where_lines_overlap()
    );

    let mut diagram = Diagram::new(width, height);

    for line_segment in &line_segments {
        diagram.add_line_segment(line_segment, true);
    }

    println!(
        "At {} points do at least two lines overlap",
        diagram.count_points_where_lines_overlap()
    );
}

fn read_line_segments(lines: Vec<String>) -> Vec<LineSegment> {
    lines.iter().map(|line| read_line_segment(line)).collect()
}

fn read_line_segment(line: &str) -> LineSegment {
    // parse '0,9 -> 5,9'
    let mut points = line.split(" -> ");

    let start = points.next().unwrap();
    let mut parts = start.split(',');
    let x: usize = parts.next().unwrap().parse().unwrap();
    let y: usize = parts.next().unwrap().parse().unwrap();
    let start = Point::new(x, y);

    let end = points.next().unwrap();
    let mut parts = end.split(',');
    let x: usize = parts.next().unwrap().parse().unwrap();
    let y: usize = parts.next().unwrap().parse().unwrap();
    let end = Point::new(x, y);

    LineSegment::new(start, end)
}

fn get_max_x_coord(line_segments: &[LineSegment]) -> usize {
    let max_start_x = line_segments.iter().map(|line| line.start.x).max().unwrap();
    let max_end_x = line_segments.iter().map(|line| line.end.x).max().unwrap();

    cmp::max(max_start_x, max_end_x)
}

fn get_max_y_coord(line_segments: &[LineSegment]) -> usize {
    let max_start_y = line_segments.iter().map(|line| line.start.y).max().unwrap();
    let max_end_y = line_segments.iter().map(|line| line.end.y).max().unwrap();

    cmp::max(max_start_y, max_end_y)
}

#[cfg(test)]
mod day5_tests {
    use std::{
        fs::File,
        io::{BufRead, BufReader},
    };

    use super::*;

    #[test]
    fn test_read_line_segment() {
        let input = "0,9 -> 5,9";
        let line = read_line_segment(input);

        assert_eq!(line.start, Point::new(0, 9));
        assert_eq!(line.end, Point::new(5, 9));
    }

    #[test]
    fn test_read_sample_input() {
        let input_file = File::open("./data/sample5.txt").unwrap();
        let reader = BufReader::new(input_file);
        let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

        let line_segments = read_line_segments(lines);
        assert_eq!(10, line_segments.len());

        let line_segment = line_segments[3];
        assert_eq!(line_segment.start, Point::new(2, 2));
        assert_eq!(line_segment.end, Point::new(2, 1));

        let line_segment = line_segments[7];
        assert_eq!(line_segment.start, Point::new(3, 4));
        assert_eq!(line_segment.end, Point::new(1, 4));
    }

    #[test]
    fn test_get_max_x_coord() {
        let input_file = File::open("./data/sample5.txt").unwrap();
        let reader = BufReader::new(input_file);
        let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();
        let line_segments = read_line_segments(lines);

        let max_x = get_max_x_coord(&line_segments);
        assert_eq!(9, max_x);
    }

    #[test]
    fn test_get_max_y_coord() {
        let input_file = File::open("./data/sample5.txt").unwrap();
        let reader = BufReader::new(input_file);
        let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();
        let line_segments = read_line_segments(lines);

        let max_y = get_max_y_coord(&line_segments);
        assert_eq!(9, max_y);
    }

    mod test_diagram {
        use super::*;

        #[test]
        fn test_count_points_where_lines_overlap() {
            let input_file = File::open("./data/sample5.txt").unwrap();
            let reader = BufReader::new(input_file);
            let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();
            let line_segments = read_line_segments(lines);

            let width = get_max_x_coord(&line_segments) + 1;
            let height = get_max_y_coord(&line_segments) + 1;
            let mut diagram = Diagram::new(width, height);

            for line_segment in &line_segments {
                diagram.add_line_segment(&line_segment, false);
            }

            assert_eq!(5, diagram.count_points_where_lines_overlap());
        }

        #[test]
        fn test_count_points_where_lines_overlap_including_diagonal() {
            let input_file = File::open("./data/sample5.txt").unwrap();
            let reader = BufReader::new(input_file);
            let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();
            let line_segments = read_line_segments(lines);

            let width = get_max_x_coord(&line_segments) + 1;
            let height = get_max_y_coord(&line_segments) + 1;
            let mut diagram = Diagram::new(width, height);

            for line_segment in &line_segments {
                diagram.add_line_segment(&line_segment, true);
            }

            assert_eq!(12, diagram.count_points_where_lines_overlap());
        }
    }

    mod test_line_segment {
        use super::*;

        #[test]
        fn test_points_horizontal() {
            let line = LineSegment::new(Point::new(0, 9), Point::new(2, 9));
            let points = line.points();

            assert_eq!(3, points.len());
            assert_eq!(Point::new(0, 9), points[0]);
            assert_eq!(Point::new(1, 9), points[1]);
            assert_eq!(Point::new(2, 9), points[2]);
        }

        #[test]
        fn test_points_vertical() {
            let line = LineSegment::new(Point::new(3, 0), Point::new(3, 3));
            let points = line.points();

            assert_eq!(4, points.len());
            assert_eq!(Point::new(3, 0), points[0]);
            assert_eq!(Point::new(3, 1), points[1]);
            assert_eq!(Point::new(3, 2), points[2]);
            assert_eq!(Point::new(3, 3), points[3]);
        }

        #[test]
        fn test_points_diagonal() {
            let line = LineSegment::new(Point::new(0, 0), Point::new(3, 3));
            let points = line.points();

            assert_eq!(4, points.len());
            assert_eq!(Point::new(0, 0), points[0]);
            assert_eq!(Point::new(1, 1), points[1]);
            assert_eq!(Point::new(2, 2), points[2]);
            assert_eq!(Point::new(3, 3), points[3]);

            let line = LineSegment::new(Point::new(3, 3), Point::new(0, 0));
            let points = line.points();

            assert_eq!(4, points.len());
            assert_eq!(Point::new(3, 3), points[0]);
            assert_eq!(Point::new(2, 2), points[1]);
            assert_eq!(Point::new(1, 1), points[2]);
            assert_eq!(Point::new(0, 0), points[3]);

            let line = LineSegment::new(Point::new(0, 3), Point::new(3, 0));
            let points = line.points();

            assert_eq!(4, points.len());
            assert_eq!(Point::new(0, 3), points[0]);
            assert_eq!(Point::new(1, 2), points[1]);
            assert_eq!(Point::new(2, 1), points[2]);
            assert_eq!(Point::new(3, 0), points[3]);

            let line = LineSegment::new(Point::new(3, 0), Point::new(0, 3));
            let points = line.points();

            assert_eq!(4, points.len());
            assert_eq!(Point::new(3, 0), points[0]);
            assert_eq!(Point::new(2, 1), points[1]);
            assert_eq!(Point::new(1, 2), points[2]);
            assert_eq!(Point::new(0, 3), points[3]);
        }
    }
}
