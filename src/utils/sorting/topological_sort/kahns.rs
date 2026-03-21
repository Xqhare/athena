use std::collections::{BTreeMap, BinaryHeap, HashSet};

#[derive(Debug)]
struct Library<'a> {
    children: Vec<&'a str>,
    num_parents: usize,
}

impl<'a> Library<'a> {
    const fn new() -> Self {
        Self {
            children: Vec::new(),
            num_parents: 0,
        }
    }
}

pub fn kahns<'a>(input: &[(&'a str, Vec<&'a str>)]) -> Result<Vec<&'a str>, String> {
    let libraries = build_libraries(input);
    topological_sort(libraries)
}

fn build_libraries<'a>(input: &[(&'a str, Vec<&'a str>)]) -> BTreeMap<&'a str, Library<'a>> {
    let mut libraries: BTreeMap<&'a str, Library<'a>> = BTreeMap::new();

    // Ensure all mentioned nodes exist
    for (name, children) in input {
        libraries.entry(name).or_insert_with(Library::new);
        for &parent in children {
            libraries.entry(parent).or_insert_with(Library::new);
        }
    }

    // Build the graph
    for (name, children) in input {
        let mut num_parents: usize = 0;
        for parent in children.iter() {
            if parent == name {
                continue;
            }
            libraries.get_mut(parent).unwrap().children.push(name);
            num_parents += 1;
        }
        libraries.get_mut(name).unwrap().num_parents = num_parents;
    }
    libraries
}

fn topological_sort<'a>(
    mut libraries: BTreeMap<&'a str, Library<'a>>,
) -> Result<Vec<&'a str>, String> {
    let total_nodes = libraries.len();

    // BinaryHeap is a max-heap. To get lexicographical smallest,
    // we use Reverse or just Reverse the comparison.
    // For simplicity, we want 'std' etc to come first if they have 0 parents.
    let mut options: BinaryHeap<std::cmp::Reverse<&str>> = libraries
        .iter()
        .filter(|(_, v)| v.num_parents == 0)
        .map(|(k, _)| std::cmp::Reverse(*k))
        .collect();

    let mut sorted: Vec<&str> = Vec::new();
    while let Some(std::cmp::Reverse(cur)) = options.pop() {
        let children = libraries.get(cur).unwrap().children.clone();
        for child_name in children {
            let child = libraries.get_mut(child_name).unwrap();
            child.num_parents -= 1;
            if child.num_parents == 0 {
                options.push(std::cmp::Reverse(child_name));
            }
        }
        sorted.push(cur);
    }

    if sorted.len() == total_nodes {
        Ok(sorted)
    } else {
        let mut remaining = HashSet::new();
        for (name, lib) in libraries {
            if lib.num_parents > 0 {
                remaining.insert(name);
            }
        }
        Err(format!("Cycle detected among {remaining:?}"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rosetta_code() {
        let input: &str = "\
            des_system_lib   std synopsys std_cell_lib des_system_lib dw02 dw01 ramlib ieee\n\
            dw01             ieee dw01 dware gtech\n\
            dw02             ieee dw02 dware\n\
            dw03             std synopsys dware dw03 dw02 dw01 ieee gtech\n\
            dw04             dw04 ieee dw01 dware gtech\n\
            dw05             dw05 ieee dware\n\
            dw06             dw06 ieee dware\n\
            dw07             ieee dware\n\
            dware            ieee dware\n\
            gtech            ieee gtech\n\
            ramlib           std ieee\n\
            std_cell_lib     ieee std_cell_lib\n\
            synopsys\
        ";

        let input: Vec<(&str, Vec<&str>)> = input
            .lines()
            .map(|line| {
                let mut parts = line.split_whitespace();
                let name = parts.next().unwrap();
                let children = parts.collect();
                (name, children)
            })
            .collect();

        let result = kahns(&input);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(
            result,
            vec![
                "ieee",
                "dware",
                "dw02",
                "dw05",
                "dw06",
                "dw07",
                "gtech",
                "dw01",
                "dw04",
                "std",
                "ramlib",
                "std_cell_lib",
                "synopsys",
                "des_system_lib",
                "dw03"
            ]
        );
    }
}
