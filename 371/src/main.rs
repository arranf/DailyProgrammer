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

    // Only need to do this for the first half of the values as doing it for the second half would be comparing each set of numbers twice
    for (i, actual_value) in solution.iter().enumerate().take(solution.len() / 2) {
        // Check that for the solution no index matches it's counterpart in either failure row
        for (j, comparison_value) in solution.iter().enumerate() {
            if i == j {
                continue;
            }

            let difference = (i as i32 - j as i32).abs() as u32;
            let diagonal_inc = *comparison_value + difference;
            let diagonal_dec = comparison_value.checked_sub(difference);
            
            if *actual_value == diagonal_inc || (diagonal_dec.is_some() && *actual_value == diagonal_dec.unwrap()) {
                return false;
            }
        }
    }
    true
}