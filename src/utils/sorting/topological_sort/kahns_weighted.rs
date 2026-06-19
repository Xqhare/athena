use std::collections::{BTreeMap, BinaryHeap, HashSet};

/// A node in the graph
#[derive(Debug)]
struct Library<'a> {
    children: Vec<&'a str>,
    num_parents: usize,
    priority: i32, // Added priority tracking
}

impl Library<'_> {
    const fn new(priority: i32) -> Self {
        Self {
            children: Vec::new(),
            num_parents: 0,
            priority,
        }
    }
}
#[derive(Eq, PartialEq)]
struct HeapElement<'a> {
    /// Priority, smaller is higher
    priority: i32,
    name: &'a str,
}

// Custom ordering logic for the Max-Heap
impl<'a> Ord for HeapElement<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // First compare by priority (Highest priority comes first, i.e. smaller priority value is higher)
        other
            .priority
            .cmp(&self.priority)
            // If priorities match, fall back to alphabetical order (A before Z)
            .then_with(|| other.name.cmp(&self.name))
    }
}

impl<'a> PartialOrd for HeapElement<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

/// Kahn's algorithm with support for weighted dependencies
///
/// Use this function if you want to manage the memory yourself.
///
/// This algorithm is used to sort a list of nodes in a topological order.
///
/// The nodes are then sorted by the supplied `i32` priority. Smaller values are higher priority.
pub fn kahns_weighted<'a>(input: &[(&'a str, i32, Vec<&'a str>)]) -> Result<Vec<&'a str>, String> {
    let libraries = build_libraries(input)?;
    topological_sort(libraries)
}

fn build_libraries<'a>(
    input: &[(&'a str, i32, Vec<&'a str>)],
) -> Result<BTreeMap<&'a str, Library<'a>>, String> {
    let mut libraries: BTreeMap<&'a str, Library<'a>> = BTreeMap::new();
    let mut defined_nodes: HashSet<&'a str> = HashSet::new();

    // 1. Ensure all mentioned nodes exist with default priorities
    for (name, priority, _) in input {
        if !defined_nodes.insert(name) {
            return Err(format!("Duplicate node definition: {name}"));
        }
        libraries.insert(name, Library::new(*priority));
    }

    // Fill in missing child references with a default priority of 0 if not defined explicitly
    for (_, _, children) in input {
        for &parent in children {
            libraries.entry(parent).or_insert_with(|| Library::new(0));
        }
    }

    // 2. Build the graph edges
    for (name, _, children) in input {
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

    // The heap now organizes items dynamically based on our custom HeapElement rules
    let mut options: BinaryHeap<HeapElement> = libraries
        .iter()
        .filter(|(_, v)| v.num_parents == 0)
        .map(|(k, v)| HeapElement {
            priority: v.priority,
            name: *k,
        })
        .collect();

    let mut sorted: Vec<&str> = Vec::new();
    while let Some(HeapElement { name: cur, .. }) = options.pop() {
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
                options.push(HeapElement {
                    priority: child.priority,
                    name: child_name,
                });
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
    fn test_basic_sort() {
        // Simple chain: A depends on B, B depends on C
        let input = vec![("A", 0, vec!["B"]), ("B", 0, vec!["C"]), ("C", 0, vec![])];
        let result = kahns_weighted(&input);
        assert_eq!(result, Ok(vec!["C", "B", "A"]));
    }

    #[test]
    fn test_priority_ordering() {
        // No dependencies. A (priority 2), B (priority 1), C (priority 3).
        // Since smaller priority value is higher priority:
        // Priority order: B (1), A (2), C (3)
        let input = vec![("A", 2, vec![]), ("B", 1, vec![]), ("C", 3, vec![])];
        let result = kahns_weighted(&input);
        assert_eq!(result, Ok(vec!["B", "A", "C"]));
    }

    #[test]
    fn test_alphabetical_tie_breaking() {
        // No dependencies. Same priority (0).
        // A, B, C should sort alphabetically (A before Z)
        let input = vec![("C", 0, vec![]), ("A", 0, vec![]), ("B", 0, vec![])];
        let result = kahns_weighted(&input);
        assert_eq!(result, Ok(vec!["A", "B", "C"]));
    }

    #[test]
    fn test_priority_and_alphabetical_mix() {
        // A (priority 2), B (priority 1), C (priority 1).
        // B and C have same priority, so alphabetical tie-breaker: B before C.
        // A has lower priority (2), so it comes after.
        let input = vec![("A", 2, vec![]), ("C", 1, vec![]), ("B", 1, vec![])];
        let result = kahns_weighted(&input);
        assert_eq!(result, Ok(vec!["B", "C", "A"]));
    }

    #[test]
    fn test_cycle_detection() {
        // A depends on B, B depends on A
        let input = vec![
            ("A", 0, vec!["B"]),
            ("B", 0, vec!["A"]),
        ];
        let result = kahns_weighted(&input);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Cycle detected"));
    }

    #[test]
    fn test_duplicate_node_definition() {
        let input = vec![
            ("A", 0, vec![]),
            ("A", 1, vec![]),
        ];
        let result = kahns_weighted(&input);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Duplicate node definition"));
    }

    #[test]
    fn test_missing_child_filled_with_default_priority() {
        // A depends on B. B is not explicitly defined in the input.
        // B should be filled in with a default priority of 0.
        // Since B has no dependencies, it runs first.
        let input = vec![
            ("A", 1, vec!["B"]),
        ];
        let result = kahns_weighted(&input);
        assert_eq!(result, Ok(vec!["B", "A"]));
    }
}
