use rayon::prelude::*;

const SIZE: usize = 141;

fn parse_input(fname: &str) -> ([[char; SIZE]; SIZE], (usize, usize)) {
    let content = std::fs::read_to_string(fname).expect("Something went wrong reading the file");

    let mut track = [[' '; SIZE]; SIZE];
    let mut start = (0, 0);

    for (i, line) in content.lines().enumerate() {
        for (j, ch) in line.chars().enumerate() {
            track[i][j] = ch;
            if ch == 'S' {
                start = (i, j);
            }
        }
    }

    (track, start)
}

fn get_next(
    track: &[[char; SIZE]; SIZE],
    current: (usize, usize),
    prev: (usize, usize),
) -> Option<(usize, usize)> {
    let (i, j) = current;
    let (prev_i, prev_j) = prev;
    let (prev_dir_i, prev_dir_j) = (i as isize - prev_i as isize, j as isize - prev_j as isize);

    for (di, dj) in [(0, 1), (0, -1), (1, 0), (-1, 0)].iter() {
        // HUGE time save on this, instead of storing a visited, we just block the previous direction
        if prev_dir_i == -*di && prev_dir_j == -*dj {
            continue;
        }

        let next_i = (i as isize + *di) as usize;
        let next_j = (j as isize + *dj) as usize;

        if track[next_i][next_j] != '#' {
            return Some((next_i, next_j));
        }
    }
    None
}

fn get_track_list(track: &[[char; SIZE]; SIZE], start: (usize, usize)) -> Vec<(usize, usize)> {
    let mut track_list = Vec::with_capacity(SIZE * SIZE);
    let mut prev = start;
    let mut current = start;

    track_list.push(current);

    while let Some(next) = get_next(track, current, prev) {
        track_list.push(next);
        prev = current;
        current = next;
    }

    track_list
}

fn get_track_scores(track_list: &[(usize, usize)]) -> [[i32; SIZE]; SIZE] {
    let mut track_scores = [[-1; SIZE]; SIZE];

    for (i, (x, y)) in track_list.iter().enumerate() {
        track_scores[*x][*y] = 1 + i as i32;
    }

    track_scores
}

fn manhattan_deltas(radius: i32) -> Vec<(i32, i32)> {
    let mut points = Vec::with_capacity(841);

    for x in 0..21 {
        for y in 0..(21 - x) {
            if (x + y) <= radius {
                if x == 0 || y == 0 {
                    points.push((x, y));
                    if x != 0 {
                        points.push((-x, y));
                    }
                    if y != 0 {
                        points.push((x, -y));
                    }
                } else {
                    points.push((x, y));
                    points.push((-x, y));
                    points.push((x, -y));
                    points.push((-x, -y));
                }
            }
        }
    }
    points
}

fn find_saves(
    track_list: &[(usize, usize)],
    track_scores: &[[i32; SIZE]; SIZE],
    dist_1: i32,
    dist_2: i32,
) -> (i32, i32) {
    let manhattan_deltas = manhattan_deltas(dist_2);

    let (saves_1, saves_2) = track_list[0..(track_list.len() - 100)]
        .into_par_iter()
        .map(|&(p1_x, p1_y)| {
            let mut local_saves_1 = 0;
            let mut local_saves_2 = 0;

            let p1_score = track_scores[p1_x][p1_y];

            for &(dx, dy) in &manhattan_deltas {
                let (p2_x, p2_y) = (p1_x as i32 + dx, p1_y as i32 + dy);

                if p2_x >= 0 && p2_x < SIZE as i32 && p2_y >= 0 && p2_y < SIZE as i32 {
                    let p2_score = track_scores[p2_x as usize][p2_y as usize];
                    let m_dist = (p1_x as i32 - p2_x).abs() + (p1_y as i32 - p2_y).abs();

                    if m_dist <= dist_1 && (p2_score - p1_score - m_dist >= 100) {
                        local_saves_1 += 1;
                    }

                    if p2_score - p1_score - m_dist >= 100 {
                        local_saves_2 += 1;
                    }
                }
            }

            (local_saves_1, local_saves_2)
        })
        .reduce(|| (0, 0), |a, b| (a.0 + b.0, a.1 + b.1));

    (saves_1, saves_2)
}

fn main() {
    let (track, start) = parse_input("D");

    let track_list = get_track_list(&track, start);
    let track_scores = get_track_scores(&track_list);

    let (saves_2, saves_20) = find_saves(&track_list, &track_scores, 2, 20);

    println!("Part 1: {}, Part 2: {}", saves_2, saves_20);
}
