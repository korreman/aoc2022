use indextree::{Arena, NodeEdge, NodeId};
use itertools::Itertools;

#[derive(Debug)]
struct Entry<'a> {
    size: u32,
    name: Option<&'a str>,
}

impl<'a> Entry<'a> {
    fn is_dir(&self) -> bool {
        self.name.is_some()
    }
}

struct DirTree<'a> {
    arena: Arena<Entry<'a>>,
    root: NodeId,
}

impl<'a> DirTree<'a> {
    fn new() -> Self {
        let mut data = Arena::new();
        let root = data.new_node(Entry {
            size: 0,
            name: Some("/"),
        });
        Self { arena: data, root }
    }

    fn compute_dir_sizes(&mut self) -> u32 {
        let mut total = 0;
        for edge in self
            .root
            .traverse(&self.arena)
            .collect::<Vec<NodeEdge>>()
            .iter()
        {
            match edge {
                indextree::NodeEdge::End(id) => {
                    let node = self.arena.get(*id).unwrap().get();
                    let size = node.size;
                    if size <= 100000 && node.is_dir() {
                        total += size;
                    }
                    if let Some(parent) = id.ancestors(&self.arena).skip(1).next() {
                        let parent = self.arena.get_mut(parent).unwrap().get_mut();
                        parent.size += size;
                    }
                }
                _ => (),
            }
        }
        total
    }
}

pub fn run(input: &str) -> (u32, u32) {
    let mut tree = DirTree::new();
    let mut dir = tree.root;
    let mut lines = input.lines().peekable();
    while let Some(line) = lines.next() {
        if line == "$ ls" {
            for listing in lines.peeking_take_while(|response| !response.starts_with('$')) {
                match listing.split_once(' ').unwrap() {
                    ("dir", name) => {
                        let node = tree.arena.new_node(Entry {
                            size: 0,
                            name: Some(name),
                        });
                        dir.append(node, &mut tree.arena);
                    }
                    (size, _) => {
                        let size = size.parse::<u32>().unwrap();
                        let node = tree.arena.new_node(Entry { size, name: None });
                        dir.append(node, &mut tree.arena);
                    }
                }
            }
        } else if line == "$ cd /" {
            dir = tree.root;
        } else if line.starts_with("$ cd ..") {
            dir = tree.arena[dir].parent().unwrap();
        } else if let Some(dest) = line.strip_prefix("$ cd ") {
            for child in dir.children(&tree.arena) {
                if tree.arena[child].get().name == Some(dest) {
                    dir = child;
                }
            }
        } else {
            panic!("unknown command: \"{line}\"");
        }
    }

    let res1 = tree.compute_dir_sizes();

    let free_space = 70_000_000 - tree.arena[tree.root].get().size;
    let space_to_free = 30_000_000 - free_space;
    let res2 = tree
        .arena
        .iter()
        .filter_map(|node| {
            let size = node.get().size;
            if size > space_to_free {
                Some(size)
            } else {
                None
            }
        })
        .min()
        .unwrap();
    (res1, res2)
}
