//! Activity-Selection Problem (Section 16.1)
//!
//! Given a set of activities with start and finish times, select the maximum
//! number of mutually compatible activities.

/// Represents an activity with start and finish times
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Activity {
    pub start: i32,
    pub finish: i32,
}

impl Activity {
    /// Creates a new activity
    ///
    /// # Arguments
    /// * `start` - Start time
    /// * `finish` - Finish time
    ///
    /// # Panics
    /// Panics if `start >= finish`
    pub fn new(start: i32, finish: i32) -> Self {
        assert!(start < finish, "start must be < finish");
        Activity { start, finish }
    }
}

/// Solves the activity-selection problem using a greedy algorithm
///
/// This corresponds to GREEDY-ACTIVITY-SELECTOR from CLRS Section 16.1.
/// Assumes activities are sorted by finish time.
///
/// # Arguments
/// * `activities` - Vector of activities sorted by finish time
///
/// # Returns
/// A vector of indices of selected activities
///
/// # Complexity
/// - Time: O(n) where n is the number of activities
/// - Space: O(n)
///
/// # Example
/// ```
/// use clrs::chapter_16::{Activity, greedy_activity_selector};
/// let activities = vec![
///     Activity::new(1, 4),
///     Activity::new(3, 5),
///     Activity::new(0, 6),
///     Activity::new(5, 7),
///     Activity::new(8, 9),
/// ];
/// let selected = greedy_activity_selector(&activities);
/// assert_eq!(selected, vec![0, 3, 4]);
/// ```
pub fn greedy_activity_selector(activities: &[Activity]) -> Vec<usize> {
    if activities.is_empty() {
        return Vec::new();
    }

    let mut selected = vec![0]; // Always select the first activity
    let mut k = 0;

    for m in 1..activities.len() {
        if activities[m].start >= activities[k].finish {
            selected.push(m);
            k = m;
        }
    }

    selected
}

/// Solves the activity-selection problem using dynamic programming
///
/// This corresponds to DYNAMIC-ACTIVITY-SELECTOR from CLRS Exercise 16.1-1.
///
/// # Arguments
/// * `activities` - Vector of activities sorted by finish time
///
/// # Returns
/// A vector of indices of selected activities
///
/// # Complexity
/// - Time: O(n³) where n is the number of activities
/// - Space: O(n²)
pub fn dynamic_activity_selector(activities: &[Activity]) -> Vec<usize> {
    let n = activities.len();
    if n == 0 {
        return Vec::new();
    }

    // Add dummy activities at the beginning and end
    // Use special values that won't conflict with real activities
    let mut extended = vec![Activity {
        start: i32::MIN,
        finish: i32::MIN,
    }];
    extended.extend_from_slice(activities);
    extended.push(Activity {
        start: i32::MAX,
        finish: i32::MAX,
    });

    let n_extended = extended.len();
    let mut c = vec![vec![0; n_extended]; n_extended];
    let mut act = vec![vec![None; n_extended]; n_extended];

    // Fill the table
    for l in 2..=n_extended - 1 {
        for i in 0..=n_extended - l - 1 {
            let j = i + l;
            c[i][j] = 0;

            let mut k = j - 1;
            while k > i && extended[k].finish > extended[i].finish {
                if extended[i].finish <= extended[k].start
                    && extended[k].finish <= extended[j].start
                    && c[i][k] + c[k][j] + 1 > c[i][j]
                {
                    c[i][j] = c[i][k] + c[k][j] + 1;
                    act[i][j] = Some(k);
                }
                k -= 1;
            }
        }
    }

    // Reconstruct the solution
    let mut result = Vec::new();
    print_activities(&act, 0, n_extended - 1, &mut result);
    result.sort();
    result
}

fn print_activities(act: &[Vec<Option<usize>>], i: usize, j: usize, result: &mut Vec<usize>) {
    if let Some(k) = act[i][j] {
        result.push(k - 1); // Adjust for dummy activity at index 0
        print_activities(act, i, k, result);
        print_activities(act, k, j, result);
    }
}

/// Solves the activity-selection problem with values (weighted version)
///
/// This corresponds to the weighted activity-selection problem from CLRS Exercise 16.1-5.
///
/// # Arguments
/// * `activities` - Vector of activities sorted by finish time
/// * `values` - Value for each activity
///
/// # Returns
/// The maximum total value achievable
///
/// # Complexity
/// - Time: O(n log n) where n is the number of activities
/// - Space: O(n)
pub fn weighted_activity_selector(activities: &[Activity], values: &[i32]) -> i32 {
    let n = activities.len();
    if n == 0 {
        return 0;
    }

    let mut dp = vec![0; n];
    dp[0] = values[0];

    for i in 1..n {
        // Option 1: Don't include activity i
        let without_i = dp[i - 1];

        // Option 2: Include activity i, find last compatible activity
        let mut with_i = values[i];
        let j = find_last_compatible(activities, i);
        if j != usize::MAX {
            with_i += dp[j];
        }

        dp[i] = without_i.max(with_i);
    }

    dp[n - 1]
}

fn find_last_compatible(activities: &[Activity], i: usize) -> usize {
    let target = activities[i].start;
    activities[..i]
        .iter()
        .rposition(|a| a.finish <= target)
        .unwrap_or(usize::MAX)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greedy_activity_selector() {
        let activities = vec![
            Activity::new(1, 4),
            Activity::new(3, 5),
            Activity::new(0, 6),
            Activity::new(5, 7),
            Activity::new(8, 9),
            Activity::new(5, 9),
        ];
        let selected = greedy_activity_selector(&activities);
        // Should select activities 0, 3, 4 (indices)
        assert_eq!(selected, vec![0, 3, 4]);
    }

    #[test]
    fn test_greedy_activity_selector_empty() {
        let activities = vec![];
        let selected = greedy_activity_selector(&activities);
        assert!(selected.is_empty());
    }

    #[test]
    fn test_dynamic_activity_selector() {
        let activities = vec![
            Activity::new(1, 4),
            Activity::new(3, 5),
            Activity::new(0, 6),
            Activity::new(5, 7),
            Activity::new(8, 9),
        ];
        let selected = dynamic_activity_selector(&activities);
        // Should select maximum number of compatible activities
        assert!(!selected.is_empty());
    }

    #[test]
    fn test_weighted_activity_selector() {
        let activities = vec![
            Activity::new(1, 4),
            Activity::new(3, 5),
            Activity::new(0, 6),
            Activity::new(5, 7),
            Activity::new(8, 9),
        ];
        let values = vec![1, 2, 3, 4, 5];
        let max_value = weighted_activity_selector(&activities, &values);
        assert!(max_value > 0);
    }
}
