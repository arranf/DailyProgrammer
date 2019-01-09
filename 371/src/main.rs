fn main() {
    assert!(qcheck(vec![4, 2, 7, 3, 6, 8, 5, 1]), "Correct solution #1");
    assert!(qcheck(vec![2, 5, 7, 4, 1, 8, 6, 3]), "Correct solution #2");
    assert!(qcheck(vec![5, 3, 1, 4, 2, 8, 6, 3]) == false, "Fail: multiple queens share the same row");
    assert!(qcheck(vec![5, 8, 2, 4, 7, 1, 3, 6]) == false, "Fail: multiple queens share the same diagonal");
    assert!(qcheck(vec![4, 3, 1, 8, 1, 3, 5, 2]) == false, "Fail: multiple reasons");
}

fn qcheck(solution: Vec<u32>) -> bool {
    // Check no two queens are in the same row
    if solution.iter().any(|&x| solution.iter().fold(0, |acc, y| if *y == x {return acc + 1} else { return acc }) > 1) {
        return false;
    }

    for (i, value) in solution.iter().enumerate() {
        // Produce the diagonal failure rows for the current index
        
        let mut up_diagonal_values = (0..i).enumerate().map(|(j, _x)| *value + (j as u32) + 1).rev().collect::<Vec<u32>>();
        up_diagonal_values.push(*value); // the current value stays the same, but we add one plus the index from this point in both directions
        up_diagonal_values.extend((i+1..solution.len()).enumerate().map(|(j, _x)| *value + (j as u32) + 1));

        let mut down_diagonal_values = (0..i).enumerate().map(|(j, _x)| value.checked_sub(j as u32).and_then(|v| v.checked_sub(1)) ).rev().collect::<Vec<Option<u32>>>();
        down_diagonal_values.push(Option::from(*value)); // the current value stays the same, but we subtract one minus the index from this point in both directions
        down_diagonal_values.extend((i+1..solution.len()).enumerate().map(|(j, _x)| value.checked_sub(j as u32).and_then(|v| v.checked_sub(1))));

        // Check that for the solution no index matches it's counterpart in either failure row
        for (j, val) in solution.iter().enumerate() {
            if j != i && (*val == up_diagonal_values[j] || (down_diagonal_values[j].is_some() && *val == down_diagonal_values[j].unwrap())) {
                return false;
            }
        }
    }
    true
}