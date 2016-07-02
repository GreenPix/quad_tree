use std::mem;

#[derive(Clone,Copy,Debug)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Clone,Debug)]
pub struct QuadTree<T> {
    root: QuadTreeNode<T>,
}

impl<T> QuadTree<T> {
    pub fn new(left: f32, bot: f32, right: f32, top: f32) -> QuadTree<T> {
        QuadTree {
            root: QuadTreeNode::new(left, bot, right, top),
        }
    }

    pub fn add(&mut self, pos: Position, data: T) {
        self.root.add(pos, data);
    }

    pub fn visit<F>(&mut self, f: &mut F)
    where F: FnMut(f32, f32, f32, f32, Option<(Position, &mut T)>) -> bool {
        self.root.visit(f);
    }
}

#[derive(Clone,Debug)]
struct QuadTreeNode<T> {
    left: f32,
    bot: f32,
    right: f32,
    top: f32,
    kind: QuadTreeNodeKind<T>,
}

#[derive(Clone,Debug)]
enum QuadTreeNodeKind<T> {
    Empty,
    Leaf((Position,T)),
    Interior(Box<Subtrees<T>>)
}

#[derive(Clone,Debug)]
struct Subtrees<T> {
    top_left:  QuadTreeNode<T>,
    top_right: QuadTreeNode<T>,
    bot_left:  QuadTreeNode<T>,
    bot_right: QuadTreeNode<T>,
}

impl<T> Subtrees<T> {
    fn new(left: f32, bot: f32, right: f32, top: f32) -> Subtrees<T> {
        debug_assert!(left < right);
        debug_assert!(bot < top);
        let mid_x = (right + left) / 2.0;
        let mid_y = (top   + bot ) / 2.0;
        Subtrees {
            top_left: QuadTreeNode::new(left, mid_y, mid_x, top),
            top_right: QuadTreeNode::new(mid_x, mid_y, right, top),
            bot_left: QuadTreeNode::new(left, bot, mid_x, mid_y),
            bot_right: QuadTreeNode::new(mid_x, bot, right, mid_y),
        }
    }

    fn add(&mut self, pos: Position, data: T) {
        let left = pos.x < (self.top_right.right + self.bot_left.left) / 2.0;
        let bot  = pos.y < (self.top_right.top   + self.bot_left.bot)  / 2.0;
        match (bot, left) {
            (true , true ) => self.bot_left.add(pos, data),
            (true , false) => self.bot_right.add(pos, data),
            (false, true ) => self.top_left.add(pos, data),
            (false, false) => self.top_right.add(pos, data),
        }
    }

    fn visit<F>(&mut self, f: &mut F)
    where F: FnMut(f32, f32, f32, f32, Option<(Position, &mut T)>) -> bool {
        self.top_left.visit(f);
        self.top_right.visit(f);
        self.bot_left.visit(f);
        self.bot_right.visit(f);
    }
}

impl<T> QuadTreeNode<T> {
    fn new(left: f32, bot: f32, right: f32, top: f32) -> QuadTreeNode<T> {
        QuadTreeNode {
            left: left,
            right: right,
            top: top,
            bot: bot,
            kind: QuadTreeNodeKind::Empty,
        }
    }

    fn add(&mut self, pos: Position, data: T) {
        if pos.x < self.left || pos.x > self.right ||
           pos.y < self.bot  || pos.y > self.top {
            panic!("Trying to add point to wrong subtree");
        }
        match self.kind {
            QuadTreeNodeKind::Empty => {
                self.kind = QuadTreeNodeKind::Leaf((pos, data));
            }
            QuadTreeNodeKind::Leaf(_) => {
                let mut subtree = Box::new(Subtrees::new(self.left, self.bot, self.right, self.top));
                subtree.add(pos, data);
                let (other_pos, other_data) = match mem::replace(&mut self.kind, QuadTreeNodeKind::Empty) {
                    QuadTreeNodeKind::Leaf(o) => o,
                    _ => unreachable!(),
                };
                subtree.add(other_pos, other_data);
                self.kind = QuadTreeNodeKind::Interior(subtree);
            }
            QuadTreeNodeKind::Interior(ref mut poss) => {
                poss.add(pos, data);
            }
        }
    }

    fn visit<F>(&mut self, f: &mut F)
    where F: FnMut(f32, f32, f32, f32, Option<(Position, &mut T)>) -> bool {
        match self.kind {
            QuadTreeNodeKind::Empty => {}
            QuadTreeNodeKind::Leaf((pos, ref mut data)) => {
                f(self.left, self.bot, self.right, self.top, Some((pos, data)));
            }
            QuadTreeNodeKind::Interior(ref mut subtrees) => {
                if f(self.left, self.bot, self.right, self.top, None) {
                    subtrees.visit(f);
                }
            }
        }
    }
}
