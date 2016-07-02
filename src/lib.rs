use std::mem;

#[derive(Clone,Copy,Debug)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Clone,Debug)]
pub struct QuadTree {
    root: QuadTreeNode,
}

impl QuadTree {
    pub fn new(left: f32, right: f32, top: f32, bot: f32) -> QuadTree {
        QuadTree {
            root: QuadTreeNode::new(left, right, top, bot),
        }
    }

    pub fn add(&mut self, node: Position) {
        self.root.add(node);
    }

    pub fn visit<F>(&mut self, f: &mut F)
    where F: FnMut(f32, f32, f32, f32, Option<&mut Position>) -> bool {
        self.root.visit(f);
    }
}

#[derive(Clone,Debug)]
struct QuadTreeNode {
    left: f32,
    right: f32,
    top: f32,
    bot: f32,
    kind: QuadTreeNodeKind,
}

#[derive(Clone,Debug)]
enum QuadTreeNodeKind {
    Empty,
    Leaf(Position),
    Interior(Box<Subtrees>)
}

#[derive(Clone,Debug)]
struct Subtrees {
    top_left:  QuadTreeNode,
    top_right: QuadTreeNode,
    bot_left:  QuadTreeNode,
    bot_right: QuadTreeNode,
}

impl Subtrees {
    fn new(left: f32, right: f32, top: f32, bot: f32) -> Subtrees {
        debug_assert!(left < right);
        debug_assert!(bot < top);
        let mid_x = (right + left) / 2.0;
        let mid_y = (top   + bot ) / 2.0;
        Subtrees {
            top_left: QuadTreeNode::new(left, mid_x, top, mid_y),
            top_right: QuadTreeNode::new(mid_x, right, top, mid_y),
            bot_left: QuadTreeNode::new(left, mid_x, mid_y, bot),
            bot_right: QuadTreeNode::new(mid_x, right, mid_y, bot),
        }
    }

    fn add(&mut self, node: Position) {
        let left = node.x < (self.top_right.right + self.bot_left.left) / 2.0;
        let bot  = node.y < (self.top_right.top   + self.bot_left.bot)  / 2.0;
        match (bot, left) {
            (true , true ) => self.bot_left.add(node),
            (true , false) => self.bot_right.add(node),
            (false, true ) => self.top_left.add(node),
            (false, false) => self.top_right.add(node),
        }
    }

    fn visit<F>(&mut self, f: &mut F)
    where F: FnMut(f32, f32, f32, f32, Option<&mut Position>) -> bool {
        self.top_left.visit(f);
        self.top_right.visit(f);
        self.bot_left.visit(f);
        self.bot_right.visit(f);
    }
}

impl QuadTreeNode {
    fn new(left: f32, right: f32, top: f32, bot: f32) -> QuadTreeNode {
        QuadTreeNode {
            left: left,
            right: right,
            top: top,
            bot: bot,
            kind: QuadTreeNodeKind::Empty,
        }
    }

    fn add(&mut self, node: Position) {
        if (node.x < self.left || node.x > self.right ||
            node.y < self.bot  || node.y > self.top) {
            panic!("Trying to add point to wrong subtree");
        }
        match self.kind {
            QuadTreeNodeKind::Empty => {
                self.kind = QuadTreeNodeKind::Leaf(node);
            }
            QuadTreeNodeKind::Leaf(_) => {
                let mut subtree = Box::new(Subtrees::new(self.left, self.right, self.top, self.bot));
                subtree.add(node);
                let other_node = match mem::replace(&mut self.kind, QuadTreeNodeKind::Empty) {
                    QuadTreeNodeKind::Leaf(other_node) => other_node,
                    _ => unreachable!(),
                };
                subtree.add(other_node);
                self.kind = QuadTreeNodeKind::Interior(subtree);
            }
            QuadTreeNodeKind::Interior(ref mut nodes) => {
                nodes.add(node);
            }
        }
    }

    fn visit<F>(&mut self, f: &mut F)
    where F: FnMut(f32, f32, f32, f32, Option<&mut Position>) -> bool {
        match self.kind {
            QuadTreeNodeKind::Empty => {}
            QuadTreeNodeKind::Leaf(ref mut pos) => {
                f(self.left, self.bot, self.right, self.top, Some(pos));
            }
            QuadTreeNodeKind::Interior(ref mut subtrees) => {
                if f(self.left, self.bot, self.right, self.top, None) {
                    subtrees.visit(f);
                }
            }
        }
    }
}
