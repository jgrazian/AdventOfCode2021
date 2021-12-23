use crate::prelude::*;

pub struct Day23 {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Bug {
    A,
    B,
    C,
    D,
}

impl Bug {
    fn target(&self) -> [usize; 4] {
        match self {
            Self::A => [11, 15, 19, 23],
            Self::B => [12, 16, 20, 24],
            Self::C => [13, 17, 21, 25],
            Self::D => [14, 18, 22, 26],
        }
    }

    fn h0(&self) -> usize {
        match self {
            Self::A => 2,
            Self::B => 4,
            Self::C => 6,
            Self::D => 8,
        }
    }

    fn cost(&self) -> u32 {
        match self {
            Self::A => 1,
            Self::B => 10,
            Self::C => 100,
            Self::D => 1000,
        }
    }

    fn to_char(&self) -> char {
        match self {
            Self::A => 'A',
            Self::B => 'B',
            Self::C => 'C',
            Self::D => 'D',
        }
    }
}

// #############   #########################
// #...........#   # 0 1 2 3 4 5 6 7 8 9 10#
// ###B#C#B#D###   #####11##12##13##14######
//   #A#D#C#A#         #15##16##17##18#
//   #########         ################
fn parse(input: &str) -> [Option<Bug>; 19] {
    let mut map = [None; 19];
    let mut i = 0;
    for c in input.chars() {
        if c.is_alphabetic() {
            let bug = match c {
                'A' => Bug::A,
                'B' => Bug::B,
                'C' => Bug::C,
                'D' => Bug::D,
                _ => unreachable!(),
            };
            map[i + 11] = Some(bug);
            i += 1;
        }
    }

    map
}

fn parse_part2(input: &str) -> [Option<Bug>; 27] {
    let mut map = [None; 27];
    let mut i = 0;
    for c in input.chars() {
        if c.is_alphabetic() {
            let bug = match c {
                'A' => Bug::A,
                'B' => Bug::B,
                'C' => Bug::C,
                'D' => Bug::D,
                _ => unreachable!(),
            };
            map[i + 11] = Some(bug);
            i += 1;
        }
    }

    map
}

fn try_move_down(map: &[Option<Bug>; 19]) -> impl Iterator<Item = ([Option<Bug>; 19], u32)> + '_ {
    (11..=14).filter_map(move |top_room| match (map[top_room], map[top_room + 4]) {
        (None, _) => None,
        (_, Some(_)) => None,
        (Some(bug), None) => {
            if top_room == bug.target()[0] {
                let mut out = map.clone();
                out[top_room] = None;
                out[top_room + 4] = Some(bug);
                Some((out, bug.cost()))
            } else {
                None
            }
        }
    })
}

fn try_move_up(map: &[Option<Bug>; 19]) -> impl Iterator<Item = ([Option<Bug>; 19], u32)> + '_ {
    (11..=14).filter_map(move |top_room| match (map[top_room], map[top_room + 4]) {
        (Some(_), _) => None,
        (_, None) => None,
        (None, Some(bug)) => {
            if top_room + 4 != bug.target()[1] {
                let mut out = map.clone();
                out[top_room] = Some(bug);
                out[top_room + 4] = None;
                Some((out, bug.cost()))
            } else {
                None
            }
        }
    })
}

fn try_move_to_room(
    map: &[Option<Bug>; 19],
) -> impl Iterator<Item = ([Option<Bug>; 19], u32)> + '_ {
    (0..=10).filter_map(move |hall| {
        if let Some(bug) = map[hall] {
            let target = bug.target();
            match (map[target[0]], map[target[1]]) {
                (Some(_), _) => None,
                (None, Some(b)) if b != bug => None,
                (None, _) => {
                    let h0 = bug.h0();
                    if bug.h0() > hall && (hall + 1..h0).all(|h| map[h] == None) {
                        let mut out = map.clone();
                        out[hall] = None;
                        out[target[0]] = Some(bug);
                        Some((out, bug.cost() * (h0 - hall + 1) as u32))
                    } else if bug.h0() < hall && (h0 - 1..hall).all(|h| map[h] == None) {
                        let mut out = map.clone();
                        out[hall] = None;
                        out[target[0]] = Some(bug);
                        Some((out, bug.cost() * (hall - h0 + 1) as u32))
                    } else {
                        None
                    }
                }
            }
        } else {
            None
        }
    })
}

fn try_move_out_room(
    map: &[Option<Bug>; 19],
) -> impl Iterator<Item = ([Option<Bug>; 19], u32)> + '_ {
    (11..=14)
        .filter_map(move |top_room| match (map[top_room], map[top_room + 4]) {
            (None, _) => None,
            (Some(bug), None) if bug.target()[0] == top_room => None,
            (Some(b1), Some(b2))
                if b1.target()[0] == top_room && b2.target()[1] == top_room + 4 =>
            {
                None
            }
            (Some(bug), _) => {
                let h0 = (top_room - 9) + (top_room - 11);
                let mut outs = Vec::new();
                for h in (0..h0).rev() {
                    if h == 2 || h == 4 || h == 6 || h == 8 {
                        continue;
                    }
                    if map[h] != None {
                        break;
                    }

                    let mut out = map.clone();
                    out[top_room] = None;
                    out[h] = Some(bug);
                    outs.push(Some((out, bug.cost() * (h0 - h + 1) as u32)))
                }
                for h in h0 + 1..11 {
                    if h == 2 || h == 4 || h == 6 || h == 8 {
                        continue;
                    }
                    if map[h] != None {
                        break;
                    }

                    let mut out = map.clone();
                    out[top_room] = None;
                    out[h] = Some(bug);
                    outs.push(Some((out, bug.cost() * (h - h0 + 1) as u32)))
                }
                Some(outs)
            }
        })
        .flatten()
        .filter_map(|v| v)
}

// #########################
// # 0 1 2 3 4 5 6 7 8 9 10#
// #####11##12##13##14######
//     #15##16##17##18#
//     #19##20##21##22#
//     #23##24##25##26#
//     ################
fn try_move_down_part2(
    map: &[Option<Bug>; 27],
) -> impl Iterator<Item = ([Option<Bug>; 27], u32)> + '_ {
    (11..=14).filter_map(move |tr| {
        if let Some(bug) = map[tr] {
            let cond = [4, 8, 12].iter().all(|i| {
                if let Some(b) = map[tr + i] {
                    if b.target()[0] == tr {
                        true
                    } else {
                        false
                    }
                } else {
                    true
                }
            });

            if bug.target()[0] == tr && cond {
                let mut out = map.clone();
                out[tr] = None;
                match (map[tr + 4], map[tr + 8], map[tr + 12]) {
                    (Some(_), _, _) => None,
                    (None, Some(_), _) => {
                        out[tr + 4] = Some(bug);
                        Some((out, bug.cost()))
                    }
                    (None, None, Some(_)) => {
                        out[tr + 8] = Some(bug);
                        Some((out, bug.cost() * 2))
                    }
                    (None, None, None) => {
                        out[tr + 12] = Some(bug);
                        Some((out, bug.cost() * 3))
                    }
                }
            } else {
                None
            }
        } else {
            None
        }
    })
}

fn try_move_up_part2(
    map: &[Option<Bug>; 27],
) -> impl Iterator<Item = ([Option<Bug>; 27], u32)> + '_ {
    (11..=14).filter_map(move |tr| {
        let cond = [4, 8, 12].iter().any(|i| {
            if let Some(b) = map[tr + i] {
                if b.target()[0] != tr {
                    true
                } else {
                    false
                }
            } else {
                false
            }
        });

        if map[tr] == None && cond {
            let mut out = map.clone();
            match (map[tr + 4], map[tr + 8], map[tr + 12]) {
                (Some(b), _, _) => {
                    out[tr] = Some(b);
                    out[tr + 4] = None;
                    Some((out, b.cost()))
                }
                (None, Some(b), _) => {
                    out[tr] = Some(b);
                    out[tr + 8] = None;
                    Some((out, b.cost() * 2))
                }
                (None, None, Some(b)) => {
                    out[tr] = Some(b);
                    out[tr + 12] = None;
                    Some((out, b.cost() * 3))
                }
                (None, None, None) => unreachable!(),
            }
        } else {
            None
        }
    })
}

fn try_move_to_room_part2(
    map: &[Option<Bug>; 27],
) -> impl Iterator<Item = ([Option<Bug>; 27], u32)> + '_ {
    (0..=10).filter_map(move |hall| {
        if let Some(bug) = map[hall] {
            let target = bug.target();
            match (
                map[target[0]],
                map[target[1]],
                map[target[2]],
                map[target[3]],
            ) {
                (Some(_), _, _, _) => None,
                (_, Some(b), _, _) if b != bug => None,
                (_, _, Some(b), _) if b != bug => None,
                (_, _, _, Some(b)) if b != bug => None,
                (None, _, _, _) => {
                    let h0 = bug.h0();
                    if bug.h0() > hall && (hall + 1..h0).all(|h| map[h] == None) {
                        let mut out = map.clone();
                        out[hall] = None;
                        out[target[0]] = Some(bug);
                        Some((out, bug.cost() * (h0 - hall + 1) as u32))
                    } else if bug.h0() < hall && (h0 - 1..hall).all(|h| map[h] == None) {
                        let mut out = map.clone();
                        out[hall] = None;
                        out[target[0]] = Some(bug);
                        Some((out, bug.cost() * (hall - h0 + 1) as u32))
                    } else {
                        None
                    }
                }
            }
        } else {
            None
        }
    })
}

fn try_move_out_room_part2(
    map: &[Option<Bug>; 27],
) -> impl Iterator<Item = ([Option<Bug>; 27], u32)> + '_ {
    (11..=14)
        .filter_map(move |tr| {
            if let Some(bug) = map[tr] {
                let cond = [4, 8, 12].iter().all(|i| {
                    if let Some(b) = map[tr + i] {
                        if b.target()[0] == tr {
                            true
                        } else {
                            false
                        }
                    } else {
                        true
                    }
                });

                if bug.target()[0] == tr && cond {
                    None
                } else {
                    let h0 = (tr - 9) + (tr - 11);
                    let mut outs = Vec::new();
                    for h in (0..h0).rev() {
                        if h == 2 || h == 4 || h == 6 || h == 8 {
                            continue;
                        }
                        if map[h] != None {
                            break;
                        }

                        let mut out = map.clone();
                        out[tr] = None;
                        out[h] = Some(bug);
                        outs.push(Some((out, bug.cost() * (h0 - h + 1) as u32)))
                    }
                    for h in h0 + 1..11 {
                        if h == 2 || h == 4 || h == 6 || h == 8 {
                            continue;
                        }
                        if map[h] != None {
                            break;
                        }

                        let mut out = map.clone();
                        out[tr] = None;
                        out[h] = Some(bug);
                        outs.push(Some((out, bug.cost() * (h - h0 + 1) as u32)))
                    }
                    Some(outs)
                }
            } else {
                None
            }
        })
        .flatten()
        .filter_map(|v| v)
}

fn part2_from_shorthand(input: &str) -> [Option<Bug>; 27] {
    let mut out = [None; 27];
    let mut i = 0;
    for c in input.chars() {
        match c {
            '.' => (),
            'A' => out[i] = Some(Bug::A),
            'B' => out[i] = Some(Bug::B),
            'C' => out[i] = Some(Bug::C),
            'D' => out[i] = Some(Bug::D),
            _ => (),
        }
        i += 1;
    }
    out
}

fn print_part2(map: &[Option<Bug>; 27]) {
    print!(
        "
    #############
    #{}{}{}{}{}{}{}{}{}{}{}#
    ###{}#{}#{}#{}###
      #{}#{}#{}#{}#
      #{}#{}#{}#{}#
      #{}#{}#{}#{}#
      #########
    ",
        map[0].map_or('.', |v| v.to_char()),
        map[1].map_or('.', |v| v.to_char()),
        map[2].map_or('.', |v| v.to_char()),
        map[3].map_or('.', |v| v.to_char()),
        map[4].map_or('.', |v| v.to_char()),
        map[5].map_or('.', |v| v.to_char()),
        map[6].map_or('.', |v| v.to_char()),
        map[7].map_or('.', |v| v.to_char()),
        map[8].map_or('.', |v| v.to_char()),
        map[9].map_or('.', |v| v.to_char()),
        map[10].map_or('.', |v| v.to_char()),
        map[11].map_or('.', |v| v.to_char()),
        map[12].map_or('.', |v| v.to_char()),
        map[13].map_or('.', |v| v.to_char()),
        map[14].map_or('.', |v| v.to_char()),
        map[15].map_or('.', |v| v.to_char()),
        map[16].map_or('.', |v| v.to_char()),
        map[17].map_or('.', |v| v.to_char()),
        map[18].map_or('.', |v| v.to_char()),
        map[19].map_or('.', |v| v.to_char()),
        map[20].map_or('.', |v| v.to_char()),
        map[21].map_or('.', |v| v.to_char()),
        map[22].map_or('.', |v| v.to_char()),
        map[23].map_or('.', |v| v.to_char()),
        map[24].map_or('.', |v| v.to_char()),
        map[25].map_or('.', |v| v.to_char()),
        map[26].map_or('.', |v| v.to_char()),
    )
}

impl Solution for Day23 {
    fn part1(&self, input: &str) -> Box<dyn ToString> {
        let map = parse(input);

        let mut best_states = HashMap::from([(map, 0)]);
        let mut queue = vec![map];
        while let Some(state) = queue.pop() {
            let current_cost = *best_states.get(&state).unwrap();

            for (next_state, cost) in try_move_down(&state)
                .chain(try_move_up(&state))
                .chain(try_move_to_room(&state))
                .chain(try_move_out_room(&state))
            {
                let best_cost = best_states.entry(next_state).or_insert(99999);
                if *best_cost <= current_cost + cost {
                    continue;
                }
                *best_cost = (*best_cost).min(current_cost + cost);
                queue.push(next_state);
            }
        }
        let solved = "#A#B#C#D#
                           #A#B#C#D#";
        let best = *best_states.get(&parse(solved)).unwrap();

        Box::new(best)
    }

    fn part2(&self, input: &str) -> Box<dyn ToString> {
        let mut new = input.to_owned();
        for i in 0..input.len() {
            if input.chars().nth(i).unwrap().is_alphabetic() && &input[i + 1..i + 4] == "###" {
                new.insert_str(i + 4, "#D#C#B#A##D#B#A#C#");
                break;
            }
        }
        let map = parse_part2(&new);

        let mut best_states = HashMap::from([(map, (0, [None; 27]))]);
        let mut queue = vec![map];
        while let Some(state) = queue.pop() {
            let (current_cost, _prev) = *best_states.get(&state).unwrap();

            for (next_state, next_cost) in try_move_down_part2(&state)
                .chain(try_move_up_part2(&state))
                .chain(try_move_to_room_part2(&state))
                .chain(try_move_out_room_part2(&state))
            {
                if let Some(best_cost) = best_states.get(&next_state) {
                    if (*best_cost).0 <= current_cost + next_cost {
                        continue;
                    }
                }

                best_states.insert(next_state, (current_cost + next_cost, state));
                queue.push(next_state);
            }
        }

        let states = [
            "...........BCBDDCBADBACADCA",
            "..........DBCB.DCBADBACADCA",
            "A.........DBCB.DCB.DBACADCA",
            "A........BDBC..DCB.DBACADCA",
            "A......B.BDBC..DC..DBACADCA",
            "AA.....B.BDBC..DC..DB.CADCA",
            "AA.....B.BDB...DC..DBCCADCA",
            "AA.....B.BDB...D.C.DBCCADCA",
            "AA...B.B.BDB...D.C.D.CCADCA",
            "AA.D.B.B.BDB...D.C.D.CCA.CA",
            "AA.D...B.BDB...D.C.D.CCABCA",
            "AA.D.....BDB...D.C.DBCCABCA",
        ];

        let mut prev = part2_from_shorthand(states[0]);
        for s in states.map(|s| part2_from_shorthand(s)) {
            if best_states.get(&s) == None {
                println!("Can't find:");
                print_part2(&s);
                break;
            }
            prev = s;
        }

        println!("Starting with:");
        print_part2(&prev);
        for (next_state, next_cost) in try_move_down_part2(&prev)
            .chain(try_move_up_part2(&prev))
            .chain(try_move_to_room_part2(&prev))
            .chain(try_move_out_room_part2(&prev))
        {
            print_part2(&next_state);
        }

        let solved = "#A#B#C#D#
        #A#B#C#D#
        #A#B#C#D#
        #A#B#C#D#";
        let finished_state = parse_part2(solved);
        let best = *best_states.get(&finished_state).unwrap();

        let mut _search = finished_state.clone();
        _search[10] = Some(Bug::D);
        _search[14] = None;
        let (c, s) = best_states.get(&_search).unwrap();
        println!("Cost:{}", c);
        print_part2(&s);

        // debug
        let mut path = vec![(best.0, finished_state)];
        while let Some((c, s)) = best_states.get(&(path.last().unwrap()).1) {
            path.push((*c, *s));
            if s == &map {
                break;
            }
        }
        path.iter().rev().for_each(|p| {
            //print_part2(&p.1);
        });

        Box::new(best.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "#############
    #...........#
    ###B#C#B#D###
      #A#D#C#A#
      #########";

    const PART1: &str = "12521";
    const PART2: &str = "44169";

    #[test]
    fn test_try_move_down() {
        let _map = [None; 19];

        let mut map = _map.clone();
        map[11] = Some(Bug::A);
        let mut out = _map.clone();
        out[15] = Some(Bug::A);
        assert_eq!(try_move_down(&map).next(), Some((out, Bug::A.cost())));

        let mut map = _map.clone();
        map[11] = Some(Bug::B);
        assert_eq!(try_move_down(&map).next(), None);

        let mut map = _map.clone();
        map[11] = Some(Bug::A);
        map[15] = Some(Bug::B);
        assert_eq!(try_move_down(&map).next(), None);
    }

    #[test]
    fn test_try_move_up() {
        let _map = [None; 19];

        let mut map = _map.clone();
        map[15] = Some(Bug::B);
        let mut out = _map.clone();
        out[11] = Some(Bug::B);
        assert_eq!(try_move_up(&map).next(), Some((out, Bug::B.cost())));

        let mut map = _map.clone();
        map[15] = Some(Bug::A);
        assert_eq!(try_move_up(&map).next(), None);

        let mut map = _map.clone();
        map[11] = Some(Bug::A);
        map[15] = Some(Bug::B);
        assert_eq!(try_move_up(&map).next(), None);
    }

    #[test]
    fn test_try_move_to_room() {
        let _map = [None; 19];

        let mut map = _map.clone();
        map[9] = Some(Bug::A);
        let mut out = _map.clone();
        out[11] = Some(Bug::A);
        assert_eq!(
            try_move_to_room(&map).next(),
            Some((out, Bug::A.cost() * 8))
        );

        let mut map = _map.clone();
        map[1] = Some(Bug::C);
        let mut out = _map.clone();
        out[13] = Some(Bug::C);
        assert_eq!(
            try_move_to_room(&map).next(),
            Some((out, Bug::C.cost() * 6))
        );

        let mut map = _map.clone();
        map[9] = Some(Bug::A);
        map[15] = Some(Bug::A);
        let mut out = _map.clone();
        out[11] = Some(Bug::A);
        out[15] = Some(Bug::A);
        assert_eq!(
            try_move_to_room(&map).next(),
            Some((out, Bug::A.cost() * 8))
        );

        let mut map = _map.clone();
        map[9] = Some(Bug::A);
        map[15] = Some(Bug::B);
        assert_eq!(try_move_to_room(&map).next(), None);
    }

    #[test]
    fn test_try_move_out_room() {
        let _map = [None; 19];

        let mut map = _map.clone();
        map[11] = Some(Bug::A);
        map[15] = Some(Bug::B);
        assert_eq!(try_move_out_room(&map).count(), 7);

        let mut map = _map.clone();
        map[5] = Some(Bug::C);
        map[11] = Some(Bug::A);
        map[15] = Some(Bug::B);
        assert_eq!(try_move_out_room(&map).count(), 3);

        let mut map = _map.clone();
        map[1] = Some(Bug::C);
        map[11] = Some(Bug::A);
        map[15] = Some(Bug::B);
        assert_eq!(try_move_out_room(&map).count(), 5);

        let mut map = _map.clone();
        map[11] = Some(Bug::A);
        map[15] = Some(Bug::B);
        map[12] = Some(Bug::C);
        assert_eq!(try_move_out_room(&map).count(), 14);
    }

    // ----------------------------------------
    #[test]
    fn test_try_move_down_part2() {
        let _map = [None; 27];

        let mut map = _map.clone();
        map[11] = Some(Bug::A);
        let mut out = _map.clone();
        out[23] = Some(Bug::A);
        assert_eq!(
            try_move_down_part2(&map).next(),
            Some((out, Bug::A.cost() * 3))
        );

        let mut map = _map.clone();
        map[11] = Some(Bug::A);
        map[23] = Some(Bug::A);
        let mut out = _map.clone();
        out[19] = Some(Bug::A);
        out[23] = Some(Bug::A);
        assert_eq!(
            try_move_down_part2(&map).next(),
            Some((out, Bug::A.cost() * 2))
        );

        let mut map = _map.clone();
        map[11] = Some(Bug::A);
        map[19] = Some(Bug::A);
        map[23] = Some(Bug::A);
        let mut out = _map.clone();
        out[15] = Some(Bug::A);
        out[19] = Some(Bug::A);
        out[23] = Some(Bug::A);
        assert_eq!(
            try_move_down_part2(&map).next(),
            Some((out, Bug::A.cost() * 1))
        );

        let mut map = _map.clone();
        map[11] = Some(Bug::A);
        map[23] = Some(Bug::B);
        assert_eq!(try_move_down_part2(&map).next(), None);

        let mut map = _map.clone();
        map[11] = Some(Bug::A);
        map[19] = Some(Bug::B);
        map[23] = Some(Bug::A);
        assert_eq!(try_move_down_part2(&map).next(), None);

        let mut map = _map.clone();
        map[19] = Some(Bug::A);
        map[23] = Some(Bug::A);
        assert_eq!(try_move_down_part2(&map).next(), None);
    }

    #[test]
    fn test_try_move_up_part2() {
        let _map = [None; 27];

        let mut map = _map.clone();
        map[23] = Some(Bug::B);
        let mut out = _map.clone();
        out[11] = Some(Bug::B);
        assert_eq!(
            try_move_up_part2(&map).next(),
            Some((out, Bug::B.cost() * 3))
        );

        let mut map = _map.clone();
        map[19] = Some(Bug::B);
        map[23] = Some(Bug::B);
        let mut out = _map.clone();
        out[11] = Some(Bug::B);
        out[23] = Some(Bug::B);
        assert_eq!(
            try_move_up_part2(&map).next(),
            Some((out, Bug::B.cost() * 2))
        );

        let mut map = _map.clone();
        map[15] = Some(Bug::B);
        map[19] = Some(Bug::B);
        map[23] = Some(Bug::B);
        let mut out = _map.clone();
        out[11] = Some(Bug::B);
        out[19] = Some(Bug::B);
        out[23] = Some(Bug::B);
        assert_eq!(
            try_move_up_part2(&map).next(),
            Some((out, Bug::B.cost() * 1))
        );

        let mut map = _map.clone();
        map[15] = Some(Bug::A);
        map[19] = Some(Bug::B);
        map[23] = Some(Bug::B);
        let mut out = _map.clone();
        out[11] = Some(Bug::A);
        out[19] = Some(Bug::B);
        out[23] = Some(Bug::B);
        assert_eq!(
            try_move_up_part2(&map).next(),
            Some((out, Bug::A.cost() * 1))
        );

        let mut map = _map.clone();
        map[15] = Some(Bug::B);
        map[19] = Some(Bug::A);
        map[23] = Some(Bug::B);
        let mut out = _map.clone();
        out[11] = Some(Bug::B);
        out[19] = Some(Bug::A);
        out[23] = Some(Bug::B);
        assert_eq!(
            try_move_up_part2(&map).next(),
            Some((out, Bug::B.cost() * 1))
        );

        let mut map = _map.clone();
        map[23] = Some(Bug::A);
        assert_eq!(try_move_up_part2(&map).next(), None);

        let mut map = _map.clone();
        map[19] = Some(Bug::A);
        map[23] = Some(Bug::A);
        assert_eq!(try_move_up_part2(&map).next(), None);

        let mut map = _map.clone();
        map[15] = Some(Bug::A);
        map[19] = Some(Bug::A);
        map[23] = Some(Bug::A);
        assert_eq!(try_move_up_part2(&map).next(), None);
    }

    #[test]
    fn test_try_move_to_room_part2() {
        let _map = [None; 27];

        let mut map = _map.clone();
        map[9] = Some(Bug::A);
        let mut out = _map.clone();
        out[11] = Some(Bug::A);
        assert_eq!(
            try_move_to_room_part2(&map).next(),
            Some((out, Bug::A.cost() * 8))
        );

        let mut map = _map.clone();
        map[1] = Some(Bug::C);
        let mut out = _map.clone();
        out[13] = Some(Bug::C);
        assert_eq!(
            try_move_to_room_part2(&map).next(),
            Some((out, Bug::C.cost() * 6))
        );

        let mut map = _map.clone();
        map[9] = Some(Bug::A);
        map[15] = Some(Bug::A);
        let mut out = _map.clone();
        out[11] = Some(Bug::A);
        out[15] = Some(Bug::A);
        assert_eq!(
            try_move_to_room_part2(&map).next(),
            Some((out, Bug::A.cost() * 8))
        );

        let mut map = _map.clone();
        map[9] = Some(Bug::A);
        map[19] = Some(Bug::A);
        let mut out = _map.clone();
        out[11] = Some(Bug::A);
        out[19] = Some(Bug::A);
        assert_eq!(
            try_move_to_room_part2(&map).next(),
            Some((out, Bug::A.cost() * 8))
        );

        let mut map = _map.clone();
        map[9] = Some(Bug::A);
        map[15] = Some(Bug::B);
        assert_eq!(try_move_to_room_part2(&map).next(), None);

        let mut map = _map.clone();
        map[9] = Some(Bug::A);
        map[19] = Some(Bug::B);
        assert_eq!(try_move_to_room_part2(&map).next(), None);
    }

    #[test]
    fn test_try_move_out_room_part2() {
        let _map = [None; 27];

        let mut map = _map.clone();
        map[11] = Some(Bug::A);
        map[15] = Some(Bug::B);
        assert_eq!(try_move_out_room_part2(&map).count(), 7);

        let mut map = _map.clone();
        map[11] = Some(Bug::A);
        map[23] = Some(Bug::B);
        assert_eq!(try_move_out_room_part2(&map).count(), 7);

        let mut map = _map.clone();
        map[5] = Some(Bug::C);
        map[11] = Some(Bug::A);
        map[15] = Some(Bug::B);
        assert_eq!(try_move_out_room_part2(&map).count(), 3);

        let mut map = _map.clone();
        map[5] = Some(Bug::C);
        map[11] = Some(Bug::A);
        map[23] = Some(Bug::B);
        assert_eq!(try_move_out_room_part2(&map).count(), 3);

        let mut map = _map.clone();
        map[1] = Some(Bug::C);
        map[11] = Some(Bug::A);
        map[15] = Some(Bug::B);
        assert_eq!(try_move_out_room_part2(&map).count(), 5);

        let mut map = _map.clone();
        map[1] = Some(Bug::C);
        map[11] = Some(Bug::A);
        map[23] = Some(Bug::B);
        assert_eq!(try_move_out_room_part2(&map).count(), 5);

        let mut map = _map.clone();
        map[11] = Some(Bug::A);
        map[15] = Some(Bug::B);
        map[12] = Some(Bug::C);
        assert_eq!(try_move_out_room_part2(&map).count(), 14);
    }

    #[test]
    fn test_part1() {
        let day = Day23 {};
        assert_eq!(day.part1(INPUT).to_string(), PART1);
    }

    #[test]
    fn test_part2() {
        let day = Day23 {};
        assert_eq!(day.part2(INPUT).to_string(), PART2);
    }
}
