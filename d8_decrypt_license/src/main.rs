use std::{error, fs, result};

type Result<T> = result::Result<T, Box<dyn error::Error>>;

fn main() -> Result<()> {
    let license = fs::read_to_string("input.txt")?;
    let lic_root = build_license_tree(&license);

    println!("Part 1: {}", lic_root.op_decrypt());
    println!("Part 2: {}", lic_root.value);

    Ok(())
}

fn build_license_tree(license: &str) -> TreeNode {
    // Covert raw license into vector of parts
    let mut parts = license
        .split(' ')
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    // First value in header is number of child nodes
    let num_children = parts[0];
    // Second value is number of metatdata values for node
    let num_data = parts[1];
    // initialize pointer to data after root header
    // (start of child node header or metatdata)
    let ptr = 2;

    let mut root = TreeNode::new();

    // Add children recursively
    for _ in 0..num_children {
        root.add_child(&mut parts, ptr);
    }

    // Add metadata to root node now that `parts` has been drained
    let start = ptr;
    let end = start + num_data;

    root.metadata.extend_from_slice(&parts[start..end]);

    // Get value of node
    let mut sum = 0;
    for child_num in &root.metadata {
        if child_num > &num_children {
            continue;
        } else {
            sum += root.children[child_num - 1].value;
        }
    }

    root.value = sum;

    root
}

struct TreeNode {
    value: usize,
    metadata: Vec<usize>,
    children: Vec<TreeNode>,
}

impl TreeNode {
    fn new() -> Self {
        TreeNode {
            value: 0,
            metadata: Vec::new(),
            children: Vec::new(),
        }
    }

    fn add_child(&mut self, parts: &mut Vec<usize>, ptr: usize) {
        let num_children = parts[ptr];
        let num_data = parts[ptr + 1];

        let mut child = TreeNode::new();

        if num_children > 0 {
            // Add next `n` children
            for _ in 0..num_children {
                child.add_child(parts, ptr + 2);
            }
        }

        // Add metadata to child node
        let start = ptr + 2;
        let end = start + num_data;

        child.metadata.extend_from_slice(&parts[start..end]);

        // Get value of node
        if num_children == 0 {
            child.value = child.metadata.iter().sum::<usize>();
        } else {
            let mut sum = 0;

            for child_num in &child.metadata {
                if child_num > &num_children {
                    continue;
                } else {
                    sum += child.children[child_num - 1].value;
                }
            }

            child.value = sum;
        }

        // Append child node
        self.children.push(child);

        // Remove node info from parts
        parts.drain(ptr..end);
    }

    fn op_decrypt(&self) -> usize {
        let mut queue = vec![self];
        let mut sum = 0;

        while !queue.is_empty() {
            // Get next node in queue
            let node = queue.pop().unwrap();
            // Add this node's children to queue
            for child in &node.children {
                queue.push(child);
            }
            // Add this node's metadata to sum
            sum += node.metadata.iter().sum::<usize>();
        }

        sum
    }

    #[allow(dead_code)]
    fn print_tree(&self) {
        let mut queue = vec![(self, 0)];

        while !queue.is_empty() {
            let (node, lvl) = queue.pop().unwrap();

            for child in node.children.iter().rev() {
                queue.push((&child, lvl + 1))
            }

            println!(
                "{}({}, {:?})",
                " -".repeat(lvl),
                &node.value,
                &node.metadata
            );
        }
    }
}

#[cfg(test)]
mod test {
    use crate::build_license_tree;

    #[test]
    fn test_part_1() {
        const EX_INPUT: &str = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2";
        let root = build_license_tree(EX_INPUT);

        assert_eq!(138, root.op_decrypt());
    }

    #[test]
    fn test_part_2() {
        const EX_INPUT: &str = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2";
        let root = build_license_tree(EX_INPUT);

        assert_eq!(66, root.value);
    }
}
