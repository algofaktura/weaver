pub fn spin_and_color_yarn_s(z_adj: ZAdjacency) -> Spool {
    let max_xyz = z_adj.keys().max().unwrap()[0].abs();
    let mut spindle = spinner(z_adj.len(), max_xyz);
    let blue: Yarn = Yarn::from(spindle.drain(..).collect::<Vec<_>>());
    let red: Yarn = blue.dot(&arr2(&[[-1, 0], [0, -1]])) + arr2(&[[0, 2]]);
    Spool::from([(3, blue), (1, red)])
}

/// new spin function doesn't use adjacency. unfortunately still slower...
/// next refactoring involves using matrix operations to manipulate an already formed sequence, if it's any faster. (see below)
pub fn spinner(order_z: usize, max_xyz: i16) -> Vec<[i16; 2]> {
    let max_absumv: i16 = max_xyz + 1;
    let mut visited: HashMap<[i16; 2], bool> = HashMap::with_capacity(order_z);
    const DISP_VECTORS: [[[i16; 2]; 2]; 4] = [
        [[-2, 0], [0, -2]],
        [[-2, 0], [0, 2]],
        [[2, 0], [0, 2]],
        [[2, 0], [0, -2]],
    ];
    let mut disp_cycler = DISP_VECTORS.iter().cycle();
    let yx: [usize; 2] = [1, 0];
    let mut yx_switch = yx.iter().cycle();
    let mut spindle: Vec<[i16; 2]> = vec![[0, 0]; order_z];
    let mut yorx: usize;
    let mut new_vect: [i16; 2];
    let start = [max_xyz, 1];
    let [mut x, mut y]: [i16; 2];
    let mut is_visited: bool;
    spindle[0] = start;
    visited.insert(start, true);
    let mut inside = false;
    let mut curr_disp = disp_cycler.next().unwrap();
    for i in 0..order_z - 1 {
        [x, y] = spindle[i];
        yorx = *yx_switch.next().unwrap();
        new_vect = get_new_vect([x, y], curr_disp[yorx]);
        is_visited = visited.get(&new_vect).is_some();
        if !inside && is_visited {
            inside = true;
        }
        if is_visited || !inside && absumv2dc(new_vect) > max_absumv {
            curr_disp = disp_cycler.next().unwrap();
            new_vect = get_new_vect([x, y], curr_disp[yorx]);
        }
        spindle[i + 1] = new_vect;
        visited.insert(new_vect, true);
    }
    spindle
}

fn get_new_vect([x, y]: [i16; 2], [a, b]: [i16; 2]) -> [i16; 2] {
    [x + a, y + b]
}
