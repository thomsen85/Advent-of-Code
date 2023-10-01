use nom::{
    bytes::complete::tag,
    character::complete::newline,
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult,
};

use aoc2022_rust::datastructs::Vec2::Vec2;

fn main() {
    part1();
    part2();
}

fn manhattan_dist(from: &Vec2, to: &Vec2) -> i32 {
    (from.x - to.x).abs() + (from.y - to.y).abs()
}

fn intersects(sb: &Vec<(Vec2, Vec2)>, point: &Vec2) -> bool {
    for (sensor, beacon) in sb {
        let s_dist = manhattan_dist(sensor, point);
        let b_dist = manhattan_dist(beacon, point);
        if s_dist == 0 || b_dist == 0 {
            return false;
        }
    }
    
    for (sensor, beacon) in sb {
        let sb_dist = manhattan_dist(sensor, beacon);
        let p_dist = manhattan_dist(sensor, point);

        if sb_dist >= p_dist {
            return true;
        }
    }
    false
}

fn intersects_w_sb(sb: &Vec<(Vec2, Vec2)>, point: &Vec2) -> bool {
    for (sensor, beacon) in sb {
        let sb_dist = manhattan_dist(sensor, beacon);
        let p_dist = manhattan_dist(sensor, point);

        if sb_dist >= p_dist {
            return true;
        }
    }
    false
}

fn part1() {
    println!("== Part 1 ==");
    let input = aoc2022_rust::utils::string_from_file("inputs/day15.txt");
    let parsed = parse_input(&input).unwrap().1;
    const LINE: i32 = 2_000_000;
    let min_x = parsed
        .iter()
        .flat_map(|v| vec![v.0.x, v.1.x])
        .min()
        .unwrap();
    let max_x = parsed
        .iter()
        .flat_map(|v| vec![v.0.x, v.1.x])
        .max()
        .unwrap();

    let mut counter = 0;
    for x in (min_x - max_x)..=(max_x * 2) {
        if intersects(&parsed, &Vec2::new(x, LINE)) {
            counter += 1;
        }
    }
    println!("\nCounted {counter}");
}

fn part2() {
    println!("== Part 2 ==");
    let input = aoc2022_rust::utils::string_from_file("inputs/day15.txt");
    let parsed = parse_input(&input).unwrap().1;
    let mut finished = false;
    const MAX: i32 = 4_000_000;
    for (sensor, beacon) in &parsed {
        let dist = manhattan_dist(sensor, beacon) + 1;
        let points = get_cirlce_points(dist, sensor)
            .into_iter()
            .filter(|&Vec2 { x, y }| x > 0 && x <= MAX && y > 0 && y <= MAX);
        for point in points {
            if !intersects_w_sb(&parsed, &point) {
                dbg!(point, point.x as i64 * MAX as i64 + point.y as i64);
                finished = true;
                break;
            }
        }
        if finished {
            break;
        }
    }
}

fn get_cirlce_points(radius: i32, center: &Vec2) -> Vec<Vec2> {
    let mut points = Vec::new();

    for height in -radius..=radius {
        //dbg!(radius - height, height);
        let l = *center - Vec2::new(radius - height.abs(), height);
        let r = *center - Vec2::new(-radius + height.abs(), height);
        points.push(l);
        points.push(r);
    }
    points
}

// Sensor at x=3011731, y=1976307: closest beacon is at x=2729595, y=2000000
fn parse_input(input: &str) -> IResult<&str, Vec<(Vec2, Vec2)>> {
    separated_list1(newline, parse_line)(input)
}

fn parse_line(input: &str) -> IResult<&str, (Vec2, Vec2)> {
    separated_pair(parse_sensor, tag(": "), parse_beacon)(input)
}

fn parse_sensor(input: &str) -> IResult<&str, Vec2> {
    let (input, out) = preceded(
        tag("Sensor at x="),
        separated_pair(
            nom::character::complete::i32,
            tag(", y="),
            nom::character::complete::i32,
        ),
    )(input)?;
    Ok((input, out.into()))
}

fn parse_beacon(input: &str) -> IResult<&str, Vec2> {
    let (input, out) = preceded(
        tag("closest beacon is at x="),
        separated_pair(
            nom::character::complete::i32,
            tag(", y="),
            nom::character::complete::i32,
        ),
    )(input)?;
    Ok((input, out.into()))
}
