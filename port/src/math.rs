//! Minimal math helpers used by the solver.

/// Returns the index of the first element in `slice` that is not less than
/// `value`, mimicking C++'s `lower_bound`.
pub fn lower_bound(slice: &[i32], value: i32) -> usize {
    if slice.is_empty() {
        return 0;
    }

    let mut start = 0usize;
    let mut end = slice.len();
    let mut mid = end / 2;

    while mid != end {
        if slice[mid] < value {
            start = mid;
            mid = (start + end + 1) / 2;
        } else {
            end = mid;
            mid = (start + end) / 2;
        }
    }

    mid
}
