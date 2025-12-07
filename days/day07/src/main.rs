enum Component {
    Empty,
    Splitter,
}

fn tachyon_manifold_row(row: &str) -> (Option<usize>, Vec<Component>) {
    let mut start_index = None;

    let components = row.chars().enumerate()
        .map(|(i, ch)| {
            match ch {
                '.' => Component::Empty,
                '^' => Component::Splitter,
                'S' => {
                    start_index = Some(i);
                    Component::Empty
                }
                _ => panic!("Unknown character in row"),
            }
        })
        .collect::<Vec<Component>>();

    (start_index, components)
}

struct TachyonManifold {
    start_position: (usize, usize),
    rows: Vec<Vec<Component>>,
}

struct TachyonBeam {
    column_position: usize,
    stacked_count: usize,
}

struct TachyonBeams {
    beams: Vec<TachyonBeam>,

    number_of_splits: usize,
    number_of_timelines: usize,
}

impl TachyonManifold {
    fn height(&self) -> usize {
        self.rows.len()
    }

    fn width(&self) -> usize {
        self.rows.first().map_or(0, |row| row.len())
    }

    fn beams(&self) -> TachyonBeams {
        let mut beams = TachyonBeams {
            beams: vec![TachyonBeam { column_position: self.start_position.1, stacked_count: 1 }],
            number_of_splits: 0,
            number_of_timelines: 0,
        };

        beams = self.rows.iter()
            .skip(self.start_position.0)
            .fold(beams, |mut acc, row| {
                let mut new_beams = acc.beams
                    .into_iter()
                    .fold(vec![], |mut new_beams, beam| {
                        let column_position = beam.column_position;

                        match row.get(column_position) {
                            Some(Component::Splitter) => {
                                acc.number_of_splits += 1;

                                // NOTE: Proper input should never cause out-of-bounds here
                                // so we skip the checks
                                new_beams.push(TachyonBeam { column_position: column_position - 1, stacked_count: beam.stacked_count });
                                new_beams.push(TachyonBeam { column_position: column_position + 1, stacked_count: beam.stacked_count });
                            }
                            Some(Component::Empty) => {
                                new_beams.push(beam);
                            }
                            None => {}
                        }
                        new_beams
                    });
                new_beams.sort_by_key(|beam| beam.column_position);
                acc.beams = new_beams
                    .into_iter()
                    .fold(Vec::<TachyonBeam>::new(), |mut deduped_beams, beam| {
                        if let Some(last_beam) = deduped_beams.last_mut() {
                            if last_beam.column_position == beam.column_position {
                                last_beam.stacked_count += beam.stacked_count;
                                return deduped_beams;
                            }
                        }
                        deduped_beams.push(beam);
                        deduped_beams
                    });
                acc
            });

        beams.number_of_timelines = beams.beams.iter().map(|beam| beam.stacked_count).sum();

        return beams;
    }
}

fn tachyon_manifold(input: &str) -> TachyonManifold {
    let mut start_position = None;
    let rows = input.lines()
        .enumerate()
        .map(|(row_index, line)| {
            let (column_index, components) = tachyon_manifold_row(line);
            if let Some(col) = column_index {
                start_position = Some((row_index, col));
            }

            components
        }).collect::<Vec<_>>();

    return TachyonManifold {
        start_position: start_position.expect("start position should exist"),
        rows,
    };
}

const INPUT: &str = include_str!("input.txt");

fn main() {
    let manifold = tachyon_manifold(INPUT.trim());
    let beams = manifold.beams();
    let split_count = beams.number_of_splits;
    println!("Part 1: {}", split_count);

    let timeline_count = beams.number_of_timelines;
    println!("Part 2: {}", timeline_count);
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r#"
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
"#;

    #[test]
    fn test_tachyon_manifold_row() {
        let (start_index, components) = tachyon_manifold_row("..^..S..^..");
        assert_eq!(start_index, Some(5));
        assert_eq!(components.len(), 11);
        assert!(matches!(components[2], Component::Splitter));
        assert!(matches!(components[8], Component::Splitter));
        assert!(matches!(components[5], Component::Empty));
    }

    #[test]
    fn test_tachyon_manifold() {
        let manifold = tachyon_manifold(EXAMPLE_INPUT.trim());
        assert_eq!(manifold.start_position, (0, 7));
        assert_eq!(manifold.height(), 16);
        assert_eq!(manifold.width(), 15);
    }

    #[test]
    fn test_count_splits() {
        let manifold = tachyon_manifold(EXAMPLE_INPUT.trim());
        let split_count = manifold.beams().number_of_splits;
        assert_eq!(split_count, 21);
    }

    #[test]
    fn test_count_timelines() {
        let manifold = tachyon_manifold(EXAMPLE_INPUT.trim());
        let timeline_count = manifold.beams().number_of_timelines;
        assert_eq!(timeline_count, 40);
    }
}