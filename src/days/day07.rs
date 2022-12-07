use itertools::Itertools;
use std::borrow::{Borrow, Cow};

#[derive(Debug)]
struct Filesystem {
    total_fs_size: Cow<'static, usize>,
    nodes: Vec<Node>,
}

#[derive(Debug)]
struct Node {
    inode: usize,
    fs_node: FsNode,
    parent: Option<usize>,
    children: Vec<usize>,
}

#[derive(Debug)]
enum FsNode {
    File { size: usize },
    Directory { name: String },
}

impl Filesystem {
    pub fn new(cli_log: &str) -> Self {
        let mut filesystem = Self {
            total_fs_size: Cow::Owned(70000000),
            nodes: vec![],
        };
        let root_node_idx = filesystem.insert_node(None, FsNode::Directory { name: "".into() });
        let mut node_pwd = root_node_idx;
        cli_log.lines().for_each(|l| {
            let parts: Vec<&str> = l.split_whitespace().collect();
            if parts[0].eq("$") {
                match parts[1] {
                    "cd" => {
                        if parts[2] == "/" {
                            node_pwd = root_node_idx;
                        } else if parts[2] == ".." {
                            node_pwd = filesystem
                                .nodes
                                .iter()
                                .find(|n| n.inode.eq(&node_pwd))
                                .unwrap()
                                .parent
                                .unwrap();
                        } else {
                            for c in &filesystem.nodes.get(node_pwd).unwrap().children {
                                let idx = *c;
                                if let FsNode::Directory { name } =
                                    &filesystem.nodes.get(idx).unwrap().fs_node
                                {
                                    if name == parts[2] {
                                        node_pwd = *c;
                                        break;
                                    }
                                }
                            }
                        }
                    }
                    "ls" => {}
                    _ => {
                        panic!("Invalid command {}", parts[1]);
                    }
                }
            } else {
                let node = if parts[0].eq("dir") {
                    FsNode::Directory {
                        name: parts[1].into(),
                    }
                } else {
                    FsNode::File {
                        size: parts[0].parse().unwrap(),
                    }
                };
                filesystem.insert_node(Some(node_pwd), node);
            }
        });
        filesystem
    }

    pub fn get_size_over(&self, limit: usize) -> usize {
        let mut sizes: Vec<usize> = vec![];
        self.directory_sizes(0, &mut sizes);
        sizes.iter().filter(|x| **x <= limit).sum()
    }

    pub fn get_smallest_candidate(&self, required_space: usize) -> usize {
        let mut sizes: Vec<usize> = vec![];
        let total_size = self.directory_sizes(0, &mut sizes);
        *sizes
            .iter()
            .sorted()
            .find(|x| self.total_fs_size.borrow() - total_size + **x >= required_space)
            .unwrap()
    }

    fn directory_sizes(&self, root: usize, sizes: &mut Vec<usize>) -> usize {
        let mut sum = 0;
        for ic in &self.nodes.get(root).unwrap().children {
            let idx = *ic;
            if let Some(c) = self.nodes.get(idx) {
                match &c.fs_node {
                    FsNode::File { size } => {
                        sum += size;
                    }
                    FsNode::Directory { name: _ } => sum += self.directory_sizes(*ic, sizes),
                }
            }
        }
        sizes.push(sum);
        sum
    }

    fn insert_node(&mut self, parent: Option<usize>, fs_node: FsNode) -> usize {
        let inode = self.nodes.len();
        self.nodes.push(Node {
            inode,
            fs_node,
            parent,
            children: vec![],
        });
        if let Some(p_inode) = parent {
            self.nodes[p_inode].children.push(inode);
        }
        inode
    }
}

fn part1(payload: &str) {
    println!("{}", Filesystem::new(payload).get_size_over(100000));
}

fn part2(payload: &str) {
    println!(
        "{}",
        Filesystem::new(payload).get_smallest_candidate(30000000)
    );
}

pub fn run(payload: &str) {
    part1(payload);
    part2(payload);
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = concat!(
        "$ cd /\n",
        "$ ls\n",
        "dir a\n",
        "14848514 b.txt\n",
        "8504156 c.dat\n",
        "dir d\n",
        "$ cd a\n",
        "$ ls\n",
        "dir e\n",
        "29116 f\n",
        "2557 g\n",
        "62596 h.lst\n",
        "$ cd e\n",
        "$ ls\n",
        "584 i\n",
        "$ cd ..\n",
        "$ cd ..\n",
        "$ cd d\n",
        "$ ls\n",
        "4060174 j\n",
        "8033020 d.log\n",
        "5626152 d.ext\n",
        "7214296 k\n"
    );

    #[test]
    fn get_size_over() {
        assert_eq!(Filesystem::new(EXAMPLE).get_size_over(100000), 95437)
    }

    #[test]
    fn get_smallest_candidate() {
        assert_eq!(
            Filesystem::new(EXAMPLE).get_smallest_candidate(30000000),
            24933642
        )
    }
}
