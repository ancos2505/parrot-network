#[allow(dead_code)]
pub(crate) fn safe_swap_remove<T>(vec: &mut Vec<T>, index: usize) -> Option<T> {
    if vec.get_mut(index).is_some() {
        let last_idx = vec.len() - 1;
        vec.swap(index, last_idx);
        vec.pop()
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safe_swap_remove_out_of_bounds() {
        let mut vec = vec![1, 2, 3];

        // Test index equal to length
        assert_eq!(safe_swap_remove(&mut vec, 3), None);
        assert_eq!(vec, vec![1, 2, 3]);

        // Test index greater than length
        assert_eq!(safe_swap_remove(&mut vec, 4), None);
        assert_eq!(vec, vec![1, 2, 3]);

        // Test with very large index
        assert_eq!(safe_swap_remove(&mut vec, usize::MAX), None);
        assert_eq!(vec, vec![1, 2, 3]);
    }

    #[test]
    fn test_safe_swap_remove_empty_vec() {
        let mut vec: Vec<i32> = Vec::new();
        assert_eq!(safe_swap_remove(&mut vec, 0), None);
        assert!(vec.is_empty());
    }

    #[test]
    fn test_safe_swap_remove_valid_indices() {
        let mut vec = vec![1, 2, 3, 4];

        // Remove first element
        assert_eq!(safe_swap_remove(&mut vec, 0), Some(1));
        assert_eq!(vec, vec![4, 2, 3]);

        // Remove last element
        assert_eq!(safe_swap_remove(&mut vec, 2), Some(3));
        assert_eq!(vec, vec![4, 2]);

        // Remove middle element
        assert_eq!(safe_swap_remove(&mut vec, 1), Some(2));
        assert_eq!(vec, vec![4]);

        // Remove last remaining element
        assert_eq!(safe_swap_remove(&mut vec, 0), Some(4));
        assert!(vec.is_empty());
    }
}
