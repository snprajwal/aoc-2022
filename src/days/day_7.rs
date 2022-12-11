#[derive(Debug, Default)]
struct Node<T>
where
    T: PartialEq,
{
    idx: usize,
    val: T,
    size: usize,
    parent: Option<usize>,
    children: Vec<usize>,
}

impl<T> Node<T>
where
    T: PartialEq,
{
    fn new(idx: usize, val: T, size: usize, parent: Option<usize>) -> Self {
        Self {
            idx,
            val,
            size,
            parent,
            children: vec![],
        }
    }
}

#[derive(Debug, Default)]
struct ArenaTree<T>
where
    T: PartialEq,
{
    arena: Vec<Node<T>>,
}

impl<T> ArenaTree<T>
where
    T: PartialEq,
{
    fn find(&mut self, parent: usize, val: T) -> Option<usize> {
        for node in &self.arena {
            if node.val == val && Some(parent) == node.parent {
                return Some(node.idx);
            }
        }
        None
    }

    fn insert(&mut self, val: T, size: usize, parent: Option<usize>) -> usize {
        let idx = self.arena.len();
        self.arena.push(Node::new(idx, val, size, parent));
        idx
    }
}

pub fn solve(input: String) -> (String, String) {
    let mut tree: ArenaTree<&str> = ArenaTree::default();
    // Track the current and prev dir
    let (mut prev, mut cur) = (0, 0);
    // Create the dir tree
    input.lines().for_each(|x| {
        let l = x.split_whitespace().collect::<Vec<&str>>();
        match l[0] {
            "$" => {
                if l[1] == "cd" {
                    match l[2] {
                        "/" => _ = tree.insert(l[2], 0, None),
                        ".." => {
                            cur = prev;
                            prev = tree.arena[cur].parent.unwrap_or_default();
                        }
                        _ => {
                            prev = cur;
                            cur = tree.find(cur, l[2]).unwrap();
                        }
                    }
                }
            }
            "dir" => {
                let n = tree.insert(l[1], 0, Some(cur));
                tree.arena[cur].children.push(n);
            }
            _ => {
                let size = l[0].parse::<usize>().unwrap();
                let mut n = tree.insert(l[1], size, Some(cur));
                tree.arena[cur].children.push(n);
                while let Some(p) = tree.arena[n].parent {
                    tree.arena[p].size += size;
                    n = p;
                }
            }
        }
    });

    // Part 1
    let s1 = tree
        .arena
        .iter()
        .filter_map(|x| {
            if !x.children.is_empty() && x.size <= 100_000 {
                Some(x.size)
            } else {
                None
            }
        })
        .sum::<usize>();

    // Part 2
    let del_size = tree.arena[0].size - 40_000_000;
    let s2 = tree
        .arena
        .iter()
        .filter_map(|x| {
            if !x.children.is_empty() && x.size > del_size {
                Some(x.size)
            } else {
                None
            }
        })
        .min()
        .unwrap();

    (s1.to_string(), s2.to_string())
}
