use crate::prelude::*;
use std::collections::HashSet;

pub struct Day19 {}

impl Day19 {}

fn parse_scanners(input: &str) -> Vec<Vec<(i64, i64, i64)>> {
    let mut scanners = Vec::new();

    let mut coords = Vec::new();
    for line in input.lines() {
        let line = line.trim();
        if line == "" {
            scanners.push(coords);
            coords = Vec::new();
            continue;
        }
        if &line[0..2] == "--" {
            continue;
        }
        let mut dirs = line.split(",").map(|s| s.parse::<i64>().unwrap());

        coords.push((
            dirs.next().unwrap(),
            dirs.next().unwrap(),
            dirs.next().unwrap(),
        ));
    }
    scanners.push(coords);

    scanners
}

fn rot_x(c: (i64, i64, i64), r: f64) -> (i64, i64, i64) {
    (
        c.0,
        ((c.1 as f64) * r.cos() - (c.2 as f64) * r.sin()).round() as i64,
        ((c.1 as f64) * r.sin() + (c.2 as f64) * r.cos()).round() as i64,
    )
}

fn rot_y(c: (i64, i64, i64), r: f64) -> (i64, i64, i64) {
    (
        ((c.0 as f64) * r.cos() + (c.2 as f64) * r.sin()).round() as i64,
        c.1,
        (-(c.0 as f64) * r.sin() + (c.2 as f64) * r.cos()).round() as i64,
    )
}

fn rot_z(c: (i64, i64, i64), r: f64) -> (i64, i64, i64) {
    (
        ((c.0 as f64) * r.cos() - (c.1 as f64) * r.sin()).round() as i64,
        ((c.0 as f64) * r.sin() + (c.1 as f64) * r.cos()).round() as i64,
        c.2,
    )
}

fn translate(c: (i64, i64, i64), t: (i64, i64, i64)) -> (i64, i64, i64) {
    (c.0 + t.0, c.1 + t.1, c.2 + t.2)
}

fn permute(c: (i64, i64, i64)) -> Vec<(i64, i64, i64)> {
    let mut res = Vec::new();
    for z in [0.0f64, 90.0, 180.0, 270.0] {
        let rot_z = rot_z(c, z.to_radians());
        for x in [0.0f64, 90.0, 180.0, 270.0] {
            let rot_x = rot_x(rot_z, x.to_radians());
            res.push(rot_x);
        }
    }
    for y in [90.0f64, 270.0] {
        let rot_y = rot_y(c, y.to_radians());
        for x in [0.0f64, 90.0, 180.0, 270.0] {
            let rot_x = rot_x(rot_y, x.to_radians());
            res.push(rot_x);
        }
    }

    res
}

fn interleave(v: Vec<Vec<(i64, i64, i64)>>) -> Vec<HashSet<(i64, i64, i64)>> {
    let mut out = Vec::new();
    for i in 0..v[0].len() {
        let mut row = HashSet::new();
        for j in 0..v.len() {
            row.insert(v[j][i]);
        }
        out.push(row);
    }
    out
}

impl Solution for Day19 {
    fn part1(&self, input: &str) -> Box<dyn ToString> {
        let mut scanners = parse_scanners(input);
        let mut root_permutations = interleave(
            scanners
                .remove(0)
                .iter()
                .map(|p| permute(*p))
                .collect::<Vec<_>>(),
        );

        let mut scanners_left = (0..scanners.len()).collect::<Vec<_>>();
        while scanners_left.len() > 0 {
            'a: for i in 0..scanners_left.len() {
                let scanner = &scanners[scanners_left[i]];

                for ref_point in scanner {
                    for root_perm_idx in 0..root_permutations.len() {
                        for root_point in &root_permutations[root_perm_idx] {
                            let diff = (
                                root_point.0 - ref_point.0,
                                root_point.1 - ref_point.1,
                                root_point.2 - ref_point.2,
                            );

                            let translated = scanner
                                .iter()
                                .map(|p| translate(*p, diff))
                                .collect::<HashSet<_>>();

                            if translated
                                .intersection(&root_permutations[root_perm_idx])
                                .count()
                                >= 12
                            {
                                root_permutations = interleave(
                                    translated
                                        .union(&root_permutations[root_perm_idx])
                                        .map(|p| permute(*p))
                                        .collect::<Vec<_>>(),
                                );
                                scanners_left.remove(i);
                                break 'a;
                            }
                        }
                    }
                }
            }
        }

        Box::new(root_permutations[0].len())
    }

    fn part2(&self, input: &str) -> Box<dyn ToString> {
        let mut scanners = parse_scanners(input);
        let mut root_permutations = interleave(
            scanners
                .remove(0)
                .iter()
                .map(|p| permute(*p))
                .collect::<Vec<_>>(),
        );
        let mut scanner_positions = interleave(vec![permute((0, 0, 0))]);

        let mut scanners_left = (0..scanners.len()).collect::<Vec<_>>();
        while scanners_left.len() > 0 {
            'a: for i in 0..scanners_left.len() {
                let scanner = &scanners[scanners_left[i]];

                for ref_point in scanner {
                    for root_perm_idx in 0..root_permutations.len() {
                        for root_point in &root_permutations[root_perm_idx] {
                            let diff = (
                                root_point.0 - ref_point.0,
                                root_point.1 - ref_point.1,
                                root_point.2 - ref_point.2,
                            );

                            let translated = scanner
                                .iter()
                                .map(|p| translate(*p, diff))
                                .collect::<HashSet<_>>();

                            if translated
                                .intersection(&root_permutations[root_perm_idx])
                                .count()
                                >= 12
                            {
                                let relative_scanner_pos = (
                                    root_point.0 + -ref_point.0,
                                    root_point.1 + -ref_point.1,
                                    root_point.2 + -ref_point.2,
                                );

                                let mut _scanner_positions = scanner_positions[root_perm_idx]
                                    .iter()
                                    .map(|p| *p)
                                    .collect::<Vec<_>>();
                                _scanner_positions.push(relative_scanner_pos);

                                scanner_positions = interleave(
                                    _scanner_positions
                                        .iter()
                                        .map(|p| permute(*p))
                                        .collect::<Vec<_>>(),
                                );

                                root_permutations = interleave(
                                    translated
                                        .union(&root_permutations[root_perm_idx])
                                        .map(|p| permute(*p))
                                        .collect::<Vec<_>>(),
                                );

                                scanners_left.remove(i);
                                break 'a;
                            }
                        }
                    }
                }
            }
        }

        // Calculate manhattan distance between two furthest points
        let mut max_dist = 0;
        for p1 in &scanner_positions[0] {
            for p2 in &scanner_positions[0] {
                let dist = (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs() + (p1.2 - p2.2).abs();
                max_dist = max_dist.max(dist);
            }
        }

        Box::new(max_dist)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "--- scanner 0 ---
    404,-588,-901
    528,-643,409
    -838,591,734
    390,-675,-793
    -537,-823,-458
    -485,-357,347
    -345,-311,381
    -661,-816,-575
    -876,649,763
    -618,-824,-621
    553,345,-567
    474,580,667
    -447,-329,318
    -584,868,-557
    544,-627,-890
    564,392,-477
    455,729,728
    -892,524,684
    -689,845,-530
    423,-701,434
    7,-33,-71
    630,319,-379
    443,580,662
    -789,900,-551
    459,-707,401
    
    --- scanner 1 ---
    686,422,578
    605,423,415
    515,917,-361
    -336,658,858
    95,138,22
    -476,619,847
    -340,-569,-846
    567,-361,727
    -460,603,-452
    669,-402,600
    729,430,532
    -500,-761,534
    -322,571,750
    -466,-666,-811
    -429,-592,574
    -355,545,-477
    703,-491,-529
    -328,-685,520
    413,935,-424
    -391,539,-444
    586,-435,557
    -364,-763,-893
    807,-499,-711
    755,-354,-619
    553,889,-390
    
    --- scanner 2 ---
    649,640,665
    682,-795,504
    -784,533,-524
    -644,584,-595
    -588,-843,648
    -30,6,44
    -674,560,763
    500,723,-460
    609,671,-379
    -555,-800,653
    -675,-892,-343
    697,-426,-610
    578,704,681
    493,664,-388
    -671,-858,530
    -667,343,800
    571,-461,-707
    -138,-166,112
    -889,563,-600
    646,-828,498
    640,759,510
    -630,509,768
    -681,-892,-333
    673,-379,-804
    -742,-814,-386
    577,-820,562
    
    --- scanner 3 ---
    -589,542,597
    605,-692,669
    -500,565,-823
    -660,373,557
    -458,-679,-417
    -488,449,543
    -626,468,-788
    338,-750,-386
    528,-832,-391
    562,-778,733
    -938,-730,414
    543,643,-506
    -524,371,-870
    407,773,750
    -104,29,83
    378,-903,-323
    -778,-728,485
    426,699,580
    -438,-605,-362
    -469,-447,-387
    509,732,623
    647,635,-688
    -868,-804,481
    614,-800,639
    595,780,-596
    
    --- scanner 4 ---
    727,592,562
    -293,-554,779
    441,611,-461
    -714,465,-776
    -743,427,-804
    -660,-479,-426
    832,-632,460
    927,-485,-438
    408,393,-506
    466,436,-512
    110,16,151
    -258,-428,682
    -393,719,612
    -211,-452,876
    808,-476,-593
    -575,615,604
    -485,667,467
    -680,325,-822
    -627,-443,-432
    872,-547,-609
    833,512,582
    807,604,487
    839,-516,451
    891,-625,532
    -652,-548,-490
    30,-46,-14";

    const PART1: &str = "79";
    const PART2: &str = "3621";

    #[test]
    fn test_rot_x() {
        assert_eq!(rot_x((1, 2, 3), 0.0f64.to_radians()), (1, 2, 3));
        assert_eq!(rot_x((1, 2, 3), 90.0f64.to_radians()), (1, -3, 2));
        assert_eq!(rot_x((1, 2, 3), 180.0f64.to_radians()), (1, -2, -3));
        assert_eq!(rot_x((1, 2, 3), 270.0f64.to_radians()), (1, 3, -2));
    }

    #[test]
    fn test_rot_y() {
        assert_eq!(rot_y((1, 0, 0), 0.0f64.to_radians()), (1, 0, 0));
        assert_eq!(rot_y((1, 0, 0), 90.0f64.to_radians()), (0, 0, -1));
        assert_eq!(rot_y((1, 0, 0), 180.0f64.to_radians()), (-1, 0, 0));
        assert_eq!(rot_y((1, 0, 0), 270.0f64.to_radians()), (0, 0, 1));
    }

    #[test]
    fn test_rot_z() {
        assert_eq!(rot_z((1, 0, 0), 0.0f64.to_radians()), (1, 0, 0));
        assert_eq!(rot_z((1, 0, 0), 90.0f64.to_radians()), (0, 1, 0));
        assert_eq!(rot_z((1, 0, 0), 180.0f64.to_radians()), (-1, 0, 0));
        assert_eq!(rot_z((1, 0, 0), 270.0f64.to_radians()), (0, -1, 0));
    }

    #[test]
    fn test_part1() {
        let day = Day19 {};
        assert_eq!(day.part1(INPUT).to_string(), PART1);
    }

    #[test]
    fn test_part2() {
        let day = Day19 {};
        assert_eq!(day.part2(INPUT).to_string(), PART2);
    }
}
