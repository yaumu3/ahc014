use proconio::input;

#[derive(Debug, Clone, Copy)]
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

#[derive(Debug, Clone, Copy)]
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
    fn clear_direction_at(&mut self, dir: Direction) {
        self.are_directions_used &= !(1 << dir.as_idx());
    }
    fn is_direction_used_at(&self, dir: Direction) -> bool {
        self.are_directions_used >> dir.as_idx() & 1 == 1
    }
}

#[derive(Debug, Clone, Copy)]
struct Rect {
    p1: P,
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
    n: usize,
    points: Vec<Vec<DirectedPoint>>,
    rects_history: Vec<Rect>,
}
impl State {
    fn new(input: &Input) -> Self {
        let n = input.n;
        let mut points = vec![vec![DirectedPoint::new(); n]; n];
        for p in input.ps.iter() {
            points[p.x as usize][p.y as usize].use_point();
        }
        Self {
            n,
            points,
            rects_history: vec![],
        }
    }
    fn get_legal_rects(&self) -> Vec<Rect> {
        let mut result = vec![];
        for (i, row) in self.points.iter().enumerate() {
            for (j, p) in row.iter().enumerate() {
                if p.is_point_used {
                    continue;
                }
                for d in Direction::to_vec() {
                    let mut cand = vec![];
                    let mut cur_pos = P::new(i as i32, j as i32);
                    let mut cur_d = d;
                    let mut cur_d_p2d = cur_d.as_p2d();
                    loop {
                        cur_pos = cur_pos + cur_d_p2d;
                        if cur_pos.x < 0
                            || cur_pos.x >= self.n as i32
                            || cur_pos.y < 0
                            || cur_pos.y >= self.n as i32
                        {
                            break;
                        }
                        let ii = cur_pos.x as usize;
                        let jj = cur_pos.y as usize;
                        let p = &self.points[ii][jj];

                        // In case the point is not occupied.
                        if !(p.is_point_used || ii == i && jj == j) {
                            if p.is_direction_used_at(cur_d)
                                || p.is_direction_used_at(cur_d.flipped())
                            {
                                break;
                            } else {
                                continue;
                            }
                        }

                        // In case the point is occupied.
                        cur_d = cur_d.rotated();
                        cur_d_p2d = cur_d_p2d.rotated();
                        if p.is_direction_used_at(cur_d) || p.is_direction_used_at(cur_d.rotated())
                        {
                            break;
                        }
                        cand.push(cur_pos);
                        if cand.len() > 4 {
                            break;
                        }

                        // If it came back to the original position
                        if cur_pos.x as usize == i && cur_pos.y as usize == j {
                            let p1 = cand.pop().unwrap();
                            let p2 = cand.pop().unwrap();
                            let p3 = cand.pop().unwrap();
                            let p4 = cand.pop().unwrap();
                            result.push(Rect { p1, p2, p3, p4, d });
                            break;
                        }
                    }
                }
            }
        }
        result
    }
    fn apply_rect(
        &mut self,
        rect: &Rect,
        f: fn(&mut DirectedPoint, Direction),
        g: fn(&mut DirectedPoint),
    ) {
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
                f(p, cur_d);
                f(p, cur_d.flipped());
                continue;
            }

            cur_d = cur_d.rotated();
            cur_d_p2d = cur_d_p2d.rotated();
            f(p, cur_d);
            f(p, cur_d.rotated());

            if cur_pos.x as usize == init_i && cur_pos.y as usize == init_j {
                g(p);
                break;
            }
        }
    }
    fn set_rect(&mut self, rect: &Rect) {
        self.apply_rect(
            rect,
            |p: &mut DirectedPoint, d: Direction| p.use_direction_at(d),
            |p: &mut DirectedPoint| p.use_point(),
        );
        self.rects_history.push(*rect);
    }
    fn undo_rect(&mut self) -> Result<Rect, String> {
        if self.rects_history.is_empty() {
            return Err("No rects used".to_owned());
        }
        let last_rect = self.rects_history.pop().unwrap();
        self.apply_rect(
            &last_rect,
            |p: &mut DirectedPoint, d: Direction| p.clear_direction_at(d),
            |p: &mut DirectedPoint| p.clear_point(),
        );
        Ok(last_rect)
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
    let centroid = P::new((input.n as i32 - 1) / 2, (input.n as i32 - 1) / 2);
    let mut state = State::new(&input);

    loop {
        let mut rects = state.get_legal_rects();
        rects.sort_unstable_by_key(|r| {
            let dist = r.p1 - centroid;
            dist.x.abs() + dist.y.abs()
        });
        if let Some(r) = rects.pop() {
            state.set_rect(&r);
        } else {
            break;
        }
    }

    println!("{}", state.rects_history.len());
    for r in &state.rects_history {
        println!("{}", r);
    }

    // while state.undo_rect().is_ok() {}
    // dbg!(state.rects_history);
}
