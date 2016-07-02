extern crate quad_tree;

use quad_tree::QuadTree;
use quad_tree::Rectangle;
use quad_tree::Position;

fn main() {
    let positions = vec![
        (1.0, -1.0),
        (1.0, 1.0),
        (-1.0, 1.0),
        (-1.0, -1.0),
    ];
    let quad_tree_area = Rectangle::new(-10.0, -10.0, 10.0, 10.0);
    let mut tree = QuadTree::new(quad_tree_area);
    for (id, (x,y)) in positions.into_iter().enumerate() {
        tree.add(Position { x: x, y: y }, id);
    }
    let rectangle = Rectangle::new(-2.0, -2.0, 0.0, 0.0);
    tree.visit(&mut |area, node| {
        match node {
            Some((pos, id)) => {
                if rectangle.is_inside(pos) {
                    println!("Collision! {} {:?}", id, pos);
                }
                true    // ignored
            }
            None => {
                rectangle.intersects(area)
            }
        }
    });
    println!("{:#?}", tree);
}
