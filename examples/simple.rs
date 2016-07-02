extern crate quad_tree;

use quad_tree::QuadTree;
use quad_tree::Position;

fn main() {
    let positions = vec![
        (1.0, -1.0),
        (1.0, 1.0),
        (-1.0, 1.0),
        (-1.0, -1.0),
    ];
    let mut tree = QuadTree::new(-10.0, 10.0, 10.0, -10.0);
    for (id, (x,y)) in positions.into_iter().enumerate() {
        tree.add(Position { x: x, y: y }, id);
    }
    let left = -2.0;
    let right = 0.0;
    let bot = -2.0;
    let top = 0.0;
    tree.visit(&mut |l, b, r, t, node| {
        match node {
            Some((pos, id)) => {
                if is_inside(pos, left, bot, right, top) {
                    println!("Collision! {} {:?}", id, pos);
                }
                true    // ignored
            }
            None => {
                intersects(left,
                           bot,
                           right,
                           top,
                           l,
                           b,
                           r,
                           t)
            }
        }
    });
    println!("{:#?}", tree);
}

fn intersects(l1: f32,
              b1: f32,
              r1: f32,
              t1: f32,
              l2: f32,
              b2: f32,
              r2: f32,
              t2: f32) -> bool {
    if b1 > t2 { return false; }
    if t1 < b2 { return false; }

    if r1 < l2  { return false; }
    if l1  > r2 { return false; }

    true
}

fn is_inside(pos: Position, left: f32, bot: f32, right: f32, top: f32) -> bool {
    pos.x >= left && pos.x <= right && pos.y >= bot && pos.y <= top
}

