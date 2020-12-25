use std::num::NonZeroUsize;
use std::ops::Index;
use std::ops::IndexMut;

// Sadness: Rust doesn't have a good linked list implementation
// with O(1) removals.

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NodeId(pub NonZeroUsize);

impl NodeId {
    fn index0(self) -> usize {
        self.0.get() - 1
    }

    pub fn append_after<T>(self, other: NodeId, arena: &mut Arena<T>) -> NodeId {
        let id = CircularNodeIterator::new(arena, other).last().unwrap().id;
        arena[id].next = std::mem::replace(&mut arena[self].next, other);
        id
    }

    pub fn pop_after<T>(self, cnt: usize, arena: &mut Arena<T>) -> Option<NodeId> {
        let (last_id, last_next) = if let Some(last) = CircularNodeIterator::new(arena, self)
            .skip(1)
            .take(cnt)
            .last()
        {
            (last.id, last.next)
        } else {
            return None;
        };
        let ret = std::mem::replace(&mut arena[self].next, last_next);
        arena[last_id].next = ret;
        Some(ret)
    }

    pub fn contains<T: PartialEq>(self, val: &T, arena: &Arena<T>) -> bool {
        CircularNodeIterator::new(arena, self).any(|n| n.data == *val)
    }
}

#[derive(Debug)]
pub struct Node<T> {
    pub data: T,
    pub id: NodeId,
    pub next: NodeId,
}

pub struct Arena<T> {
    nodes: Vec<Node<T>>,
}

impl<T> Arena<T> {
    pub fn new() -> Arena<T> {
        Self { nodes: Vec::new() }
    }

    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    pub fn new_node(&mut self, data: T) -> NodeId {
        let id =
            NodeId(NonZeroUsize::new(self.nodes.len().wrapping_add(1)).expect("too many nodes"));
        self.nodes.push(Node { data, id, next: id });
        id
    }
}

impl<T> Index<NodeId> for Arena<T> {
    type Output = Node<T>;

    fn index(&self, node: NodeId) -> &Node<T> {
        &self.nodes[node.index0()]
    }
}

impl<T> IndexMut<NodeId> for Arena<T> {
    fn index_mut(&mut self, node: NodeId) -> &mut Node<T> {
        &mut self.nodes[node.index0()]
    }
}

pub struct CircularNodeIterator<'a, T> {
    arena: &'a Arena<T>,
    start_at: NodeId,
    current: Option<NodeId>,
}

impl<'a, T> CircularNodeIterator<'a, T> {
    pub fn new(arena: &'a Arena<T>, start_at: NodeId) -> Self {
        CircularNodeIterator {
            arena,
            current: Some(start_at),
            start_at,
        }
    }
}

impl<'a, T> Iterator for CircularNodeIterator<'a, T> {
    type Item = &'a Node<T>;

    fn next(&mut self) -> Option<&'a Node<T>> {
        let current = self.current;
        match current {
            Some(id) => {
                let result = &self.arena[id];
                self.current = if result.next != self.start_at {
                    Some(result.next)
                } else {
                    None
                };
                Some(result)
            }
            None => None,
        }
    }
}
