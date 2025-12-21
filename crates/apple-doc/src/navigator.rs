//! Parse the data in `index/navigator.index`.
//!
//! This is in a custom binary format.
use core::fmt;
use std::path::Path;

use crate::{AvailabilityId, AvailabilityMask, Kind, Lang};

/// The ID of a [`NavigatorItem`] in a [`NavigatorTree`].
pub type NavigatorId = u32;

/// An ordered tree of [`NavigatorItem`]s.
///
/// This comes from `index/navigator.index`. Each item has an ID based on its
/// order in the data set.
///
/// You can use either the ID or the "index path" (the indexing steps one has
/// to do to get from the root to the item) to look up an item.
///
/// # Examples
///
/// ```
/// use apple_doc::{external_dir, NavigatorTree};
///
/// let external_dir = external_dir();
/// let tree = NavigatorTree::from_external_dir(&external_dir).unwrap();
///
/// // Get the root node. This is the item with ID 0.
/// let root = tree.item_at([]).unwrap();
/// assert_eq!(root.name, "[Root]");
/// assert_eq!(root, tree.item(0).unwrap());
///
/// // Get an iterator over the root node's children.
/// let mut children = tree.children_at([]).unwrap();
/// assert_eq!(children.next().unwrap().name, "Swift");
/// assert_eq!(children.next().unwrap().name, "Objective-C");
/// assert_eq!(children.next().unwrap().name, "Data");
/// assert_eq!(children.next().unwrap().name, "Other");
/// assert_eq!(children.next(), None);
///
/// // Get the children of the Objective-C node (index 1 of root).
/// let mut children = tree.children_at([1]).unwrap();
/// assert_eq!(children.next().unwrap().name, "App Frameworks");
///
/// // Get the children of that node (index 0 of Objective-C which is index 1 of root).
/// let mut children = tree.children_at([1, 0]).unwrap();
/// assert_eq!(children.next(), None); // No children
///
/// // And so on.
/// ```
#[derive(Clone)]
pub struct NavigatorTree {
    data: Box<[u8]>,
    nodes: Box<[Node]>,
}

impl fmt::Debug for NavigatorTree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NavigatorTree")
            .field("data", &"[...]")
            .field("data_len", &self.data.len())
            .field("nodes", &"[...]")
            .field("nodes_len", &self.nodes.len())
            .finish()
    }
}

/// A helper struct to represent nodes in the navigator tree.
///
/// This contains offsets instead of e.g. `Vec<Node>`, to avoid needless heap
/// allocations (the dataset here is fairly large, and such allocations would
/// be noticeable).
///
/// The offsets are `u32` for size efficiency, which is valid because the
/// frame header itself uses `u32` as the frame length.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Node {
    /// The offset in `data` for the information about this node.
    frame_data_offset: u32,
    /// The index / offset in `nodes` where the children of this node begins.
    first_child_id: NavigatorId,
}

impl NavigatorTree {
    pub fn from_external_dir(external_dir: &Path) -> std::io::Result<Self> {
        let data = std::fs::read(external_dir.join("index/navigator.index"))?;
        Ok(Self::load(data))
    }

    /// Load the data and build the tree.
    pub fn load(data: Vec<u8>) -> Self {
        let mut iter = FrameIter(&data);
        let mut nodes: Vec<Node> = Vec::with_capacity(iter.size_hint().0);

        // Handle root node specially, it has the same parent ID as the
        // subsequent nodes.
        let _ = iter.next().expect("must have root node");
        nodes.push(Node {
            frame_data_offset: 0,
            first_child_id: 1,
        });

        // Iterate over each item, create a node for each, and add child node
        // sections to their parent nodes.
        //
        // We do this parsing up front, because it's useful to access the data
        // as a tree, and we need it to know e.g. the node IDs.
        let mut current_parent_id = 0;
        for (parent_id, frame) in iter {
            let frame_data_offset =
                (frame.as_ptr().addr() - data.as_ptr().addr() - HEADER_SIZE) as u32;

            if current_parent_id != parent_id {
                // The parent changed; we're in a new section of children.
                current_parent_id = parent_id;

                let first_child_node_id = nodes.len() as NavigatorId;
                let parent_node = &mut nodes[parent_id as usize];
                parent_node.first_child_id = first_child_node_id;
            }

            nodes.push(Node {
                frame_data_offset,
                // Unknown at this point, will be filled in above when
                // we reach a section that applies to this node.
                first_child_id: NavigatorId::MAX,
            });
        }

        Self {
            data: data.into(),
            nodes: nodes.into(),
        }
    }

    /// Get the item index at the given index path.
    ///
    /// If the index path is invalid, `None` is returned.
    pub fn id_at(&self, index_path: impl IntoIterator<Item = u32>) -> Option<NavigatorId> {
        let mut current_id = 0; // Root node
        let mut current_node = &self.nodes[current_id as usize];

        for idx in index_path {
            let prev_id = current_id;

            // We don't store the amount of children, so this calculation may
            // go "out of bounds" of the childre of the current node...
            current_id = current_node.first_child_id.checked_add(idx)?;
            current_node = self.nodes.get(current_id as usize)?;

            // ... so we verify that the parent node ID was the one we expected.
            let parent_id =
                FrameIter::one(&self.data[current_node.frame_data_offset as usize..])?.0;
            if prev_id != parent_id {
                return None;
            }
        }

        Some(current_id)
    }

    /// Get the item with the given ID.
    ///
    /// If no item with that ID exists, `None` is returned.
    pub fn item(&self, id: NavigatorId) -> Option<NavigatorItem<'_>> {
        let node = self.nodes.get(id as usize)?;
        let data = &self.data[node.frame_data_offset as usize..];
        Some(NavigatorItem::from_frame(FrameIter::one(data).unwrap().1))
    }

    /// Helper to get the item at the given index path.
    pub fn item_at(&self, index_path: impl IntoIterator<Item = u32>) -> Option<NavigatorItem<'_>> {
        self.id_at(index_path).and_then(|id| self.item(id))
    }

    /// Get the parent ID of the item with the given ID.
    ///
    /// If no item with that ID exists, `None` is returned.
    ///
    /// The root node (id = 0) has no parent, so that also returns `None`.
    pub fn parent_id(&self, id: NavigatorId) -> Option<NavigatorId> {
        if id == 0 {
            // It's more consistent with `fn children` below to say that the
            // root node doesn't have any parent.
            return None;
        }
        let node = self.nodes.get(id as usize)?;
        let data = &self.data[node.frame_data_offset as usize..];
        Some(FrameIter::one(data).unwrap().0)
    }

    /// Get the children of the item with the given ID.
    ///
    /// If the index is invalid, `None` is returned.
    pub fn children(&self, id: NavigatorId) -> Option<impl Iterator<Item = NavigatorItem<'_>>> {
        let node = self.nodes.get(id as usize)?;
        Some(
            (node.first_child_id != NavigatorId::MAX) // -> no children
                .then(|| {
                    let first_child_node = self.nodes[node.first_child_id as usize];
                    let slice = &self.data[first_child_node.frame_data_offset as usize..];

                    FrameIter(slice).map_while(move |(parent_id, frame)| {
                        if id == parent_id {
                            Some(NavigatorItem::from_frame(frame))
                        } else {
                            None
                        }
                    })
                })
                .into_iter()
                .flatten(),
        )
    }

    /// Helper to get the item at the given index path.
    pub fn children_at(
        &self,
        index_path: impl IntoIterator<Item = u32>,
    ) -> Option<impl Iterator<Item = NavigatorItem<'_>>> {
        self.id_at(index_path).and_then(|id| self.children(id))
    }

    pub fn prev_sibling_id(&self, id: NavigatorId) -> Option<NavigatorId> {
        let prev_id = id.checked_sub(1)?;

        let parent_id = self.parent_id(id)?;
        let prev_parent_id = self.parent_id(prev_id)?;
        if parent_id != prev_parent_id {
            return None;
        }

        Some(prev_id)
    }

    pub fn next_sibling_id(&self, id: NavigatorId) -> Option<NavigatorId> {
        let next_id = id.checked_add(1)?;

        let parent_id = self.parent_id(id)?;
        let next_parent_id = self.parent_id(next_id)?;
        if parent_id != next_parent_id {
            return None;
        }

        Some(next_id)
    }

    /// Traverse the tree, visiting each item in turn.
    ///
    /// The closure is given the current index path and the item at that path.
    pub fn visit(&self, mut f: impl FnMut(&[u32], NavigatorItem<'_>)) {
        fn inner(
            tree: &NavigatorTree,
            id: NavigatorId,
            index_path: &mut Vec<u32>,
            f: &mut impl FnMut(&[u32], NavigatorItem<'_>),
        ) {
            for (i, child) in tree.children(id).unwrap().enumerate() {
                let i = i as u32;

                // Extend the index path while processing this child.
                index_path.push(i);

                f(index_path, child);

                let child_id = tree.nodes[id as usize].first_child_id + i;
                inner(tree, child_id, index_path, f);

                index_path.pop();
            }
        }

        let root_id = 0;

        f(&[], self.item(root_id).unwrap());

        let mut index_path = vec![];
        inner(self, root_id, &mut index_path, &mut f);
    }
}

/// An entry in `index/navigator.index`.
#[derive(Copy, Clone, PartialEq, Eq, Ord, PartialOrd, Hash, Debug)]
pub struct NavigatorItem<'data> {
    pub kind: Kind,
    pub language: Lang,
    pub availability_mask: AvailabilityMask,
    pub availability_id: AvailabilityId,
    /// The display name of the item.
    ///
    /// This is not unique.
    ///
    /// To translate to `txt` variant, strip all non `[a-zA-Z0-9:(){}_]` chars.
    pub name: &'data str,
}

/// An iterator over individual data frames in `index/navigator.index`.
struct FrameIter<'data>(&'data [u8]);

impl<'data> FrameIter<'data> {
    fn one(data: &'data [u8]) -> Option<(NavigatorId, &'data [u8])> {
        Self(data).next()
    }
}

const HEADER_SIZE: usize = 8;
const MIN_FRAME_SIZE: usize = 2 + 8 + 8 + 8 + 8;

impl<'data> Iterator for FrameIter<'data> {
    type Item = (NavigatorId, &'data [u8]);

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.0.len() / (HEADER_SIZE + MIN_FRAME_SIZE), None)
    }

    fn next(&mut self) -> Option<Self::Item> {
        if self.0.is_empty() {
            return None;
        }

        let (header, rest) = self.0.split_at(HEADER_SIZE);
        let parent_id = NavigatorId::from_le_bytes([header[0], header[1], header[2], header[3]]);
        let frame_len = u32::from_le_bytes([header[4], header[5], header[6], header[7]]) as usize;

        let (frame, rest) = rest.split_at(frame_len);
        self.0 = rest;

        Some((parent_id, frame))
    }
}

impl<'data> NavigatorItem<'data> {
    /// Iterate over all data in `index/navigator.index`.
    ///
    /// Outputs the navigator item, as well as the ID / index of the parent
    /// navigator item.
    ///
    /// Use [`NavigatorTree`] if you want to view the data as the structured
    /// tree that it actually is.
    pub fn iter_all(data: &'data [u8]) -> impl Iterator<Item = (NavigatorId, Self)> {
        FrameIter(data).map(|(parent_id, frame)| (parent_id, Self::from_frame(frame)))
    }

    fn from_frame(mut frame: &'data [u8]) -> NavigatorItem<'data> {
        assert!(MIN_FRAME_SIZE <= frame.len());

        let kind = match frame[0] {
            0x00 => Kind::Root,
            0x01 => Kind::Article,
            0x02 => Kind::Tutorial,
            0x05 => Kind::Hub,
            0x07 => Kind::Overview,
            0x0A => Kind::Module,
            0x14 => Kind::Class,
            0x15 => Kind::Struct,
            0x16 => Kind::Protocol,
            0x17 => Kind::Enum,
            0x18 => Kind::Function,
            0x1B => Kind::GlobalVariable,
            0x1C => Kind::Typedef,
            0x1D => Kind::AssociatedType,
            0x1E => Kind::SwiftOp,
            0x1F => Kind::Macro,
            0x20 => Kind::Union,
            0x21 => Kind::EnumCase,
            0x22 => Kind::InitMethod,
            0x23 => Kind::Method,
            0x24 => Kind::Property,
            0x25 => Kind::Const,
            0x26 => Kind::SwiftSubscriptOp,
            0x27 => Kind::TypeMethod,
            0x2B => Kind::PropertyListKey,
            0x2C => Kind::SampleCode,
            0x2D => Kind::Endpoint,
            0x2E => Kind::Object,
            0x30 => Kind::Namespace,
            0x7F => Kind::LangHeader,
            0xFF => Kind::GroupMarker,
            kind => panic!("unknown kind {kind:0X} in {frame:?}"),
        };
        let language = Lang(frame[1]);
        frame = &frame[2..];

        let availability_mask = AvailabilityMask(u64::from_le_bytes([
            frame[0], frame[1], frame[2], frame[3], frame[4], frame[5], frame[6], frame[7],
        ]));
        frame = &frame[8..];

        let availability_id = AvailabilityId(u64::from_le_bytes([
            frame[0], frame[1], frame[2], frame[3], frame[4], frame[5], frame[6], frame[7],
        ]));
        frame = &frame[8..];

        let name_len = u64::from_le_bytes([
            frame[0], frame[1], frame[2], frame[3], frame[4], frame[5], frame[6], frame[7],
        ]) as usize;
        frame = &frame[8..];

        let reserved = u64::from_le_bytes([
            frame[0], frame[1], frame[2], frame[3], frame[4], frame[5], frame[6], frame[7],
        ]);
        frame = &frame[8..];
        debug_assert_eq!(reserved, 0);

        let name = &frame[..name_len];
        let name = str::from_utf8(name).unwrap();

        frame = &frame[name_len..];

        assert_eq!(frame.len(), 0);

        NavigatorItem {
            kind,
            language,
            availability_mask,
            availability_id,
            name,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::external_dir;

    #[test]
    fn simple() {
        let external_dir = external_dir();
        let tree = NavigatorTree::from_external_dir(&external_dir).unwrap();

        assert_eq!(tree.item_at([]).unwrap().kind, Kind::Root);

        assert_eq!(tree.children_at([]).unwrap().count(), 4);

        assert_eq!(tree.item_at([0]).unwrap().name, "Swift");
        assert_eq!(tree.item_at([1]).unwrap().name, "Objective-C");
        assert_eq!(tree.item_at([2]).unwrap().name, "Data");
        assert_eq!(tree.item_at([3]).unwrap().name, "Other");
        assert_eq!(tree.item_at([4]), None); // Invalid index

        assert_eq!(tree.item_at([1, 4]).unwrap().name, "AppKit");
        assert_eq!(tree.item_at([1, 4, 0]).unwrap().name, "Essentials");
        assert_eq!(tree.item_at([1, 4, 1000]), None);

        assert_eq!(tree.parent_id(0), None);
        assert_eq!(tree.parent_id(1), Some(0));
        assert_eq!(tree.parent_id(5), Some(1));

        assert_eq!(tree.prev_sibling_id(0), None);
        assert_eq!(tree.prev_sibling_id(1), None);
        assert_eq!(tree.prev_sibling_id(2), Some(1));
        assert_eq!(tree.prev_sibling_id(3), Some(2));
        assert_eq!(tree.prev_sibling_id(4), Some(3));
        assert_eq!(tree.prev_sibling_id(5), None);

        assert_eq!(tree.next_sibling_id(0), None);
        assert_eq!(tree.next_sibling_id(1), Some(2));
        assert_eq!(tree.next_sibling_id(2), Some(3));
        assert_eq!(tree.next_sibling_id(3), Some(4));
        assert_eq!(tree.next_sibling_id(4), None);
        assert_eq!(tree.next_sibling_id(5), Some(6));
    }

    #[test]
    fn visit() {
        let external_dir = external_dir();
        let tree = NavigatorTree::from_external_dir(&external_dir).unwrap();

        // Ensure that the number of items visited is the same as the total
        // number of items.
        let mut count = 0;
        tree.visit(|index_path, item| {
            let id = tree.id_at(index_path.iter().copied()).unwrap();
            assert_eq!(tree.item(id).unwrap(), item);

            count += 1;
        });

        assert_eq!(count, NavigatorItem::iter_all(&tree.data).count());
    }
}
