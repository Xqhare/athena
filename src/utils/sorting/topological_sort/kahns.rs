use std::collections::{BTreeMap, BinaryHeap, HashSet};

/// A node in the graph
#[derive(Debug)]
struct Library<'a> {
    children: Vec<&'a str>,
    num_parents: usize,
}

impl Library<'_> {
    const fn new() -> Self {
        Self {
            children: Vec::new(),
            num_parents: 0,
        }
    }
}

/// Kahn's algorithm
///
/// Use this function if you don't want to manage the memory yourself.
/// Consider using `kahns` if you do.
///
/// This algorithm is used to sort a list of nodes in a topological order
///
/// Please note, this implementation requires that all nodes are unique.
/// It has inbuild cycle detection and will error if a cycle is detected.
///
/// # Arguments
/// * `input` - a list of tuples of the form `(name, children)`
///
/// # Returns
/// A list of topologically sorted nodes
///
/// # Errors
/// Returns an error if a cycle is detected, if a parent is not found, or if a duplicate node is found
pub fn kahns_managed(input: &[(String, Vec<String>)]) -> Result<Vec<String>, String> {
    let input: Vec<(&str, Vec<&str>)> = input
        .iter()
        .map(|(k, v)| {
            (
                k.as_str(),
                v.iter().map(std::string::String::as_str).collect(),
            )
        })
        .collect();
    Ok(kahns(&input)?
        .into_iter()
        .map(std::string::ToString::to_string)
        .collect())
}

/// Kahn's algorithm
///
/// Use this function if you want to manage the memory yourself.
/// Consider using `kahns_managed` if you don't.
///
/// This algorithm is used to sort a list of nodes in a topological order
///
/// Please note, this implementation requires that all nodes are unique.
/// It has inbuild cycle detection and will error if a cycle is detected.
///
/// # Arguments
/// * `input` - a list of tuples of the form `(name, children)`
///
/// # Returns
/// A list of topologically sorted nodes
///
/// # Errors
/// Returns an error if a cycle is detected, if a parent is not found, or if a duplicate node is found
pub fn kahns<'a>(input: &[(&'a str, Vec<&'a str>)]) -> Result<Vec<&'a str>, String> {
    let libraries = build_libraries(input)?;
    topological_sort(libraries)
}

fn build_libraries<'a>(
    input: &[(&'a str, Vec<&'a str>)],
) -> Result<BTreeMap<&'a str, Library<'a>>, String> {
    let mut libraries: BTreeMap<&'a str, Library<'a>> = BTreeMap::new();
    let mut defined_nodes: HashSet<&'a str> = HashSet::new();

    // Ensure all mentioned nodes exist and that there are no duplicate definitions
    for (name, children) in input {
        if !defined_nodes.insert(name) {
            return Err(format!("Duplicate node definition: {name}"));
        }

        libraries.entry(name).or_insert_with(Library::new);
        for &parent in children {
            libraries.entry(parent).or_insert_with(Library::new);
        }
    }

    // Build the graph
    for (name, children) in input {
        let mut num_parents: usize = 0;
        for &parent in children {
            if parent == *name {
                continue;
            }
            libraries
                .get_mut(parent)
                .ok_or_else(|| format!("Parent node {parent} not found"))?
                .children
                .push(name);
            num_parents += 1;
        }
        if let Some(lib) = libraries.get_mut(name) {
            lib.num_parents = num_parents;
        }
    }
    Ok(libraries)
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
        let children = libraries
            .get(cur)
            .expect("Node missing after heap pop")
            .children
            .clone();
        for child_name in children {
            let child = libraries
                .get_mut(child_name)
                .ok_or_else(|| format!("Child node {child_name} not found in map"))?;
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
        println!("{result:?}");
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
