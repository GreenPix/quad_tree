extern crate nalgebra;

use std::mem;

use nalgebra::Point2;

pub type Position = Point2<f32>;

#[derive(Clone,Debug,Copy)]
pub struct Rectangle {
    pub left: f32,
    pub bot: f32,
    pub right: f32,
    pub top: f32,
}

impl Rectangle {
    pub fn new(left: f32, bot: f32, right: f32, top: f32) -> Rectangle {
        Rectangle {
            left: left,
            bot: bot,
            right: right,
            top: top,
        }
    }

    pub fn intersects(&self, other: &Rectangle) -> bool {
        if self.bot > other.top { return false; }
        if self.top < other.bot { return false; }

        if self.right < other.left  { return false; }
        if self.left  > other.right { return false; }

        true
    }

    pub fn intersects_loosened(&self, other: &Rectangle, amount: f32) -> bool {
        let other_loosened = Rectangle::new(other.left - amount,
                                            other.bot - amount,
                                            other.right + amount,
                                            other.top + amount);
        self.intersects(&other_loosened)
    }

    pub fn is_inside(&self, pos: Position) -> bool {
        pos.x >= self.left && pos.x <= self.right && pos.y >= self.bot && pos.y <= self.top
    }

    pub fn contains(&self, other: Rectangle) -> bool {
        other.left >= self.left && other.right <= self.right
            && other.bot >= self.bot && other.top <= self.top
    }
}

#[derive(Clone,Debug)]
pub struct QuadTree<T> {
    root: QuadTreeNode<T>,
}

impl<T> QuadTree<T> {
    pub fn new(area: Rectangle) -> QuadTree<T> {
        QuadTree {
            root: QuadTreeNode::new(area),
        }
    }

    pub fn add(&mut self, pos: Position, data: T) {
        self.root.add(pos, data);
    }

    pub fn visit<F>(&self, f: &mut F)
    where F: FnMut(&Rectangle, Option<(Position, &T)>) -> bool {
        self.root.visit(f);
    }

    pub fn area(&self) -> Rectangle {
        self.root.area
    }
}

#[derive(Clone,Debug)]
struct QuadTreeNode<T> {
    area: Rectangle,
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
    fn new(area: Rectangle) -> Subtrees<T> {
        debug_assert!(area.left < area.right);
        debug_assert!(area.bot < area.top);
        let mid_x = (area.right + area.left) / 2.0;
        let mid_y = (area.top   + area.bot ) / 2.0;
        Subtrees {
            top_left: QuadTreeNode::new(Rectangle::new(area.left, mid_y, mid_x, area.top)),
            top_right: QuadTreeNode::new(Rectangle::new(mid_x, mid_y, area.right, area.top)),
            bot_left: QuadTreeNode::new(Rectangle::new(area.left, area.bot, mid_x, mid_y)),
            bot_right: QuadTreeNode::new(Rectangle::new(mid_x, area.bot, area.right, mid_y)),
        }
    }

    fn add(&mut self, pos: Position, data: T) {
        let left = pos.x < (self.top_right.area.right + self.bot_left.area.left) / 2.0;
        let bot  = pos.y < (self.top_right.area.top   + self.bot_left.area.bot)  / 2.0;
        match (bot, left) {
            (true , true ) => self.bot_left.add(pos, data),
            (true , false) => self.bot_right.add(pos, data),
            (false, true ) => self.top_left.add(pos, data),
            (false, false) => self.top_right.add(pos, data),
        }
    }

    fn visit<F>(&self, f: &mut F)
    where F: FnMut(&Rectangle, Option<(Position, &T)>) -> bool {
        self.top_left.visit(f);
        self.top_right.visit(f);
        self.bot_left.visit(f);
        self.bot_right.visit(f);
    }
}

impl<T> QuadTreeNode<T> {
    fn new(area: Rectangle) -> QuadTreeNode<T> {
        QuadTreeNode {
            area: area,
            kind: QuadTreeNodeKind::Empty,
        }
    }

    fn add(&mut self, pos: Position, data: T) {
        if pos.x < self.area.left || pos.x > self.area.right ||
           pos.y < self.area.bot  || pos.y > self.area.top {
            panic!("Trying to add point to wrong subtree");
        }
        match self.kind {
            QuadTreeNodeKind::Empty => {
                self.kind = QuadTreeNodeKind::Leaf((pos, data));
            }
            QuadTreeNodeKind::Leaf(_) => {
                let mut subtree = Box::new(Subtrees::new(self.area));
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

    fn visit<F>(&self, f: &mut F)
    where F: FnMut(&Rectangle, Option<(Position, &T)>) -> bool {
        match self.kind {
            QuadTreeNodeKind::Empty => {}
            QuadTreeNodeKind::Leaf((pos, ref data)) => {
                f(&self.area, Some((pos, data)));
            }
            QuadTreeNodeKind::Interior(ref subtrees) => {
                if f(&self.area, None) {
                    subtrees.visit(f);
                }
            }
        }
    }
}
