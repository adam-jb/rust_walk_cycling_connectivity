use std::cmp::Ordering;

/// Use with `BinaryHeap`. Since it's a max-heap, reverse the comparison to get the smallest cost
/// first.
#[derive(PartialEq, Eq, Clone)]
pub struct PriorityQueueItem<K, V, A, L> {
    pub cost: K,
    pub value: V,
    pub angle_arrived_from: A,
    pub link_arrived_from: L,
}

impl<K: Ord, V: Ord, A: Ord, L: Ord> PartialOrd for PriorityQueueItem<K, V, A, L> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<K: Ord, V: Ord, A: Ord, L: Ord> Ord for PriorityQueueItem<K, V, A, L> {
    fn cmp(&self, other: &Self) -> Ordering {
        let ord = other.cost.cmp(&self.cost);
        if ord != Ordering::Equal {
            return ord;
        }
        // The tie-breaker is arbitrary, based on the value
        self.value.cmp(&other.value)
    }
}
