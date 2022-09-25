use proconio::input;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct P {
    x: i32,
    y: i32,
}
impl P {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
    fn rotated(&self) -> Self {
        Self {
            x: -self.y,
            y: self.x,
        }
    }
    fn weighted_dist(&self) -> i32 {
        self.x.abs() * self.x.abs() + self.y.abs() * self.y.abs() + 1
    }
}
impl std::ops::Neg for P {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}
impl std::ops::Add for P {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
impl std::ops::Sub for P {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    D,
    DR,
    R,
    UR,
    U,
    UL,
    L,
    DL,
}
impl Direction {
    fn from_p2d(p2d: P) -> Result<Self, String> {
        let dx = p2d.x.signum();
        let dy = p2d.y.signum();
        match (dx, dy) {
            (1, 0) => Ok(Self::D),
            (1, 1) => Ok(Self::DR),
            (0, 1) => Ok(Self::R),
            (-1, 1) => Ok(Self::UR),
            (-1, 0) => Ok(Self::U),
            (-1, -1) => Ok(Self::UL),
            (0, -1) => Ok(Self::L),
            (1, -1) => Ok(Self::DL),
            (0, 0) => Err("(0, 0) cannot be converted into LineDirection".to_owned()),
            _ => unreachable!(),
        }
    }
    fn as_p2d(&self) -> P {
        match self {
            Self::D => P::new(1, 0),
            Self::DR => P::new(1, 1),
            Self::R => P::new(0, 1),
            Self::UR => P::new(-1, 1),
            Self::U => P::new(-1, 0),
            Self::UL => P::new(-1, -1),
            Self::L => P::new(0, -1),
            Self::DL => P::new(1, -1),
        }
    }
    fn from_idx(idx: usize) -> Self {
        [
            Self::D,
            Self::DR,
            Self::R,
            Self::UR,
            Self::U,
            Self::UL,
            Self::L,
            Self::DL,
        ][idx]
    }
    fn as_idx(&self) -> usize {
        match self {
            Self::D => 0,
            Self::DR => 1,
            Self::R => 2,
            Self::UR => 3,
            Self::U => 4,
            Self::UL => 5,
            Self::L => 6,
            Self::DL => 7,
        }
    }
    fn to_vec() -> Vec<Self> {
        (0..=7).map(Self::from_idx).collect()
    }
    fn flipped(self) -> Self {
        Self::from_p2d(-self.as_p2d()).unwrap()
    }
    fn rotated(&self) -> Self {
        let p = self.as_p2d();
        Self::from_p2d(p.rotated()).unwrap()
    }
}

#[derive(Debug, Clone)]
struct DirectedPoint {
    is_point_used: bool,
    are_directions_used: u8,
}
impl DirectedPoint {
    fn new() -> Self {
        Self {
            is_point_used: false,
            are_directions_used: 0,
        }
    }
    fn use_point(&mut self) {
        self.is_point_used |= true;
    }
    fn clear_point(&mut self) {
        self.is_point_used &= false;
    }
    fn use_direction_at(&mut self, dir: Direction) {
        self.are_directions_used |= 1 << dir.as_idx();
    }
    fn is_direction_used_at(&self, dir: Direction) -> bool {
        self.are_directions_used >> dir.as_idx() & 1 == 1
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Rect {
    // CCW p1 -> p2 -> p3 -> p4
    p1: P, // A point currently not used
    p2: P,
    p3: P,
    p4: P,
    d: Direction,
}
impl std::fmt::Display for Rect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {} {} {} {} {} {}",
            self.p1.x, self.p1.y, self.p2.x, self.p2.y, self.p3.x, self.p3.y, self.p4.x, self.p4.y
        )
    }
}

#[derive(Debug, Clone)]
struct State {
    points: Vec<Vec<DirectedPoint>>,
    centroid: P,
    score: i32,
    legal_rects: Vec<Rect>,
    rects_history: Vec<Rect>,
}
impl State {
    fn initialize_legal_rect_by_brute_force(&mut self) {
        let n = self.points.len();
        let mut result = vec![];
        for i in 0..n {
            for j in 0..n {
                let p1 = P::new(i as i32, j as i32);
                for d in Direction::to_vec() {
                    if let Some(rect) = self.get_legal_rect(p1, d) {
                        result.push(rect);
                    }
                }
            }
        }
        self.legal_rects = result;
    }
    fn new(input: &Input) -> Self {
        let n = input.n;
        let mut points = vec![vec![DirectedPoint::new(); n]; n];
        for p in input.ps.iter() {
            points[p.x as usize][p.y as usize].use_point();
        }
        let mut result = Self {
            points,
            centroid: P::new((input.n as i32 - 1) / 2, (input.n as i32 - 1) / 2),
            score: 0,
            legal_rects: vec![],
            rects_history: vec![],
        };
        result.initialize_legal_rect_by_brute_force();
        result
    }
    fn get_legal_rect(&mut self, p1: P, d: Direction) -> Option<Rect> {
        let n = self.points.len();
        let init_i = p1.x as usize;
        let init_j = p1.y as usize;
        if init_i >= n || init_j >= n {
            return None;
        }
        if self.points[init_i][init_j].is_point_used {
            return None;
        }

        let mut cur_d = d;
        let p2 = self.search_point(p1, cur_d, cur_d.rotated())?;
        cur_d = cur_d.rotated();
        let p3 = self.search_point(p2, cur_d, cur_d.rotated())?;
        cur_d = cur_d.rotated();
        let p4 = self.search_point(p3, cur_d, cur_d.rotated())?;
        cur_d = cur_d.rotated();
        self.points[init_i][init_j].use_point();
        if let Some(p1_returned) = self.search_point(p4, cur_d, cur_d.rotated()) {
            self.points[init_i][init_j].clear_point();
            if p1_returned != p1 {
                return None;
            }
            Some(Rect { p1, p2, p3, p4, d })
        } else {
            self.points[init_i][init_j].clear_point();
            None
        }
    }
    fn search_point(&self, p: P, d: Direction, turned_d: Direction) -> Option<P> {
        let n = self.points.len();

        let mut cur_pos = p;
        let flipped_d = d.flipped();
        let d_p2d = d.as_p2d();
        loop {
            cur_pos = cur_pos + d_p2d;
            let i = cur_pos.x as usize;
            let j = cur_pos.y as usize;
            if i >= n || j >= n {
                return None;
            }
            let p = &self.points[i][j];

            // In case the point is not used.
            if !p.is_point_used {
                if p.is_direction_used_at(d) || p.is_direction_used_at(flipped_d) {
                    return None;
                } else {
                    continue;
                }
            }

            // In case the point is used.
            if p.is_direction_used_at(flipped_d) || p.is_direction_used_at(turned_d) {
                return None;
            }
            return Some(cur_pos);
        }
    }
    fn cur_p1_as_next_p2(&mut self, next_p2: P, d: Direction) -> Option<Rect> {
        let p3_cand = self.search_point(next_p2, d.rotated(), d.flipped())?;
        let p4_cand = self.search_point(p3_cand, d.flipped(), d.flipped().rotated())?;
        let p1_cand = next_p2 + (p4_cand - p3_cand);
        self.get_legal_rect(p1_cand, d)
    }
    fn cur_p1_as_next_p3(&mut self, next_p3: P, d: Direction) -> Option<Rect> {
        let p2_cand = self.search_point(next_p3, d.flipped().rotated(), d.flipped())?;
        let p4_cand = self.search_point(next_p3, d.flipped(), d.flipped().rotated())?;
        let p1_cand = p2_cand + (p4_cand - next_p3);
        self.get_legal_rect(p1_cand, d)
    }
    fn cur_p1_as_next_p4(&mut self, next_p4: P, d: Direction) -> Option<Rect> {
        let p3_cand = self.search_point(next_p4, d, d.flipped().rotated())?;
        let p2_cand = self.search_point(p3_cand, d.flipped().rotated(), d.flipped())?;
        let p1_cand = p2_cand + (next_p4 - p3_cand);
        self.get_legal_rect(p1_cand, d)
    }
    fn set_rect(&mut self, rect: &Rect) {
        let init_i = rect.p1.x as usize;
        let init_j = rect.p1.y as usize;

        let mut cur_pos = rect.p1;
        let mut cur_d = rect.d;
        let mut cur_d_p2d = cur_d.as_p2d();
        loop {
            cur_pos = cur_pos + cur_d_p2d;
            let ii = cur_pos.x as usize;
            let jj = cur_pos.y as usize;
            let p = &mut self.points[ii][jj];

            if !(p.is_point_used || ii == init_i && jj == init_j) {
                p.use_direction_at(cur_d);
                p.use_direction_at(cur_d.flipped());
                continue;
            }

            cur_d = cur_d.rotated();
            cur_d_p2d = cur_d_p2d.rotated();
            p.use_direction_at(cur_d);
            p.use_direction_at(cur_d.rotated());

            if cur_pos.x as usize == init_i && cur_pos.y as usize == init_j {
                p.use_point();
                break;
            }
        }
        self.rects_history.push(*rect);
        self.score += (rect.p1 - self.centroid).weighted_dist();

        // Filter out rects which has become illegal by setting the rect
        let mut legal_rects = vec![];
        for r in &self.legal_rects.clone() {
            if let Some(r) = self.get_legal_rect(r.p1, r.d) {
                legal_rects.push(r);
            }
        }
        self.legal_rects = legal_rects;

        // Add rects which has became legal by setting the rect
        for d in Direction::to_vec() {
            // Case 1: rect.p1 becomes `p2` of a new rect
            if let Some(r) = self.cur_p1_as_next_p2(rect.p1, d) {
                self.legal_rects.push(r)
            }
            // Case 2: rect.p1 becomes `p3` of a new rect
            if let Some(r) = self.cur_p1_as_next_p3(rect.p1, d) {
                self.legal_rects.push(r)
            };
            // Case 3: rect.p1 becomes `p4` of a new rect
            if let Some(r) = self.cur_p1_as_next_p4(rect.p1, d) {
                self.legal_rects.push(r)
            };
        }
    }
}

#[derive(Clone, Debug)]
struct Input {
    n: usize,
    ps: Vec<P>, // 0-indexed
}
fn parse_input() -> Input {
    input! {
        n: usize, m: usize,
        ps: [(i32, i32); m],
    }
    let ps = ps.into_iter().map(|(x, y)| P::new(x, y)).collect();
    Input { n, ps }
}

fn main() {
    let input = parse_input();
    let beam_width = 1;

    let mut states = vec![State::new(&input)];
    let mut terminal_states = vec![];

    // beam search
    loop {
        let mut next_states = vec![];
        while let Some(s) = states.pop() {
            if s.legal_rects.is_empty() {
                terminal_states.push(s);
                continue;
            }
            for r in &s.legal_rects {
                let mut ns = s.clone();
                ns.set_rect(r);
                next_states.push(ns);
            }
        }
        if next_states.is_empty() {
            break;
        }
        next_states.sort_unstable_by_key(|s| std::cmp::Reverse(s.score));
        next_states.truncate(beam_width);
        states = next_states;
    }

    terminal_states.sort_unstable_by_key(|s| s.score);

    let best_state = terminal_states.last().unwrap();
    println!("{}", best_state.rects_history.len());
    for r in &best_state.rects_history {
        println!("{}", r);
    }
}
