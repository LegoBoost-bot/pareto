use std::slice::Iter;

pub use pareto_derive::Dominate;

pub trait Dominate {
    /// Returns `true` if `self` [Pareto dominates][w] `other`.
    ///
    /// [w]: https://en.wikipedia.org/wiki/Pareto_efficiency
    fn dominates(&self, other: &Self) -> bool;
}

impl<T: Dominate> Dominate for &T {
    fn dominates(&self, other: &Self) -> bool {
        (*self).dominates(*other)
    }
}

#[derive(Debug, Clone)]
pub struct ParetoFront<T> {
    front: Vec<T>,
}

impl<T> ParetoFront<T> {
    pub fn new() -> Self {
        ParetoFront { front: Vec::new() }
    }

    pub fn len(&self) -> usize {
        self.front.len()
    }

    pub fn is_empty(&self) -> bool {
        self.front.is_empty()
    }

    pub fn as_slice(&self) -> &[T] {
        self.front.as_slice()
    }

    pub fn iter(&self) -> Iter<T> {
        self.front.iter()
    }
}

impl<T: Dominate> ParetoFront<T> {
    /// Removes all items from the front which are dominated by `new`, starting from item #`start_idx`.
    fn remove_dominated_from(&mut self, new: &T, start_idx: usize) {
        let to_remove = self
            .front
            .iter()
            .skip(start_idx)
            .enumerate()
            .filter(|&(_, e)| new.dominates(e))
            .map(|(i, _)| i)
            .rev()
            .collect::<Vec<_>>();

        for i in to_remove {
            self.front.swap_remove(i);
        }
    }

    /// Removes all items in the front which are dominated by `new`.
    ///
    /// Returns `true` if the element can be added to the front, and `false` if it was dominated by
    /// an existing element.
    fn remove_dominated(&mut self, new: &T) -> bool {
        for (i, e) in self.front.iter().enumerate() {
            if e.dominates(new) {
                // The new element is dominated by an existing one, and thus is not part of the front.
                return false;
            }
            if new.dominates(e) {
                self.remove_dominated_from(new, i);
                return true;
            }
        }

        true
    }

    /// Try adding an element (`new`) to the front.
    ///
    /// If `new` was indeed added to the front (meaning it was _not_ dominated by an existing
    /// element), returns `true`. If `new` was dominated by an existing element, returns `false`.
    pub fn push(&mut self, new: T) -> bool {
        if self.remove_dominated(&new) {
            self.front.push(new);
            return true;
        }
        false
    }
}

impl<T> Default for ParetoFront<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> From<ParetoFront<T>> for Vec<T> {
    fn from(value: ParetoFront<T>) -> Self {
        value.front
    }
}

impl<T> IntoIterator for ParetoFront<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.front.into_iter()
    }
}

impl<T: Dominate> FromIterator<T> for ParetoFront<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut front = Self::new();
        for e in iter {
            front.push(e);
        }
        front
    }
}

impl<T: Dominate> Extend<T> for ParetoFront<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for e in iter {
            self.push(e);
        }
    }
}

#[repr(transparent)]
#[derive(PartialEq, Eq, Debug)]
pub struct Inverse<T>(T);

impl<T: PartialOrd> PartialOrd for Inverse<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.0.partial_cmp(&self.0)
    }
}

impl<T: Ord> Ord for Inverse<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.0.cmp(&self.0)
    }
}
