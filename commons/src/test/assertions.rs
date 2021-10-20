//! Additional assertions for [`spectral`]

use spectral::{iter::ContainingIntoIterAssertions, vec::VecAssertions, AssertionFailure, Spec};
use std::fmt::Debug;

/// Additional assertions for Vec
pub trait ExtendedVecAssertions<'s, T: 's> {
    fn is_not_empty(&self);
    fn contains_exactly_in_any_order<E: 's>(&mut self, expected_values_iter: &'s E)
    where
        E: IntoIterator<Item = &'s T> + Clone,
        E::IntoIter: ExactSizeIterator;
}

impl<'s, T: 's> ExtendedVecAssertions<'s, T> for Spec<'s, Vec<T>>
where
    T: PartialEq + Debug,
{
    /// Asserts that the subject vector is not empty.
    ///
    /// ```rust
    /// # use spectral::prelude::*;
    /// # use integration_test_commons::test::assertions::ExtendedVecAssertions;
    /// assert_that(&vec![1]).is_not_empty();
    /// ```
    fn is_not_empty(&self) {
        if self.subject.is_empty() {
            AssertionFailure::from_spec(self)
                .with_expected(String::from("a non-empty vec"))
                .with_actual(String::from("an empty vec"))
                .fail();
        }
    }

    /// Asserts that the subject vector contains exactly the provided values in any order.
    ///
    /// ```rust
    /// # use spectral::prelude::*;
    /// # use integration_test_commons::test::assertions::ExtendedVecAssertions;
    /// assert_that(&vec![1, 2, 3]).contains_exactly_in_any_order(&vec![&3, &1, &2]);
    /// ```
    fn contains_exactly_in_any_order<E: 's>(&mut self, expected_values_iter: &'s E)
    where
        E: IntoIterator<Item = &'s T> + Clone,
        E::IntoIter: ExactSizeIterator,
    {
        self.has_length(expected_values_iter.clone().into_iter().len());
        self.contains_all_of(expected_values_iter);
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use spectral::prelude::*;

    #[test]
    fn should_not_panic_if_vec_was_expected_to_be_not_empty_and_is_not() {
        assert_that(&vec![1]).is_not_empty();
    }

    #[test]
    #[should_panic(expected = "\u{1b}[31mexpected: a non-empty vec\n\t \
        but was: an empty vec\u{1b}[0m")]
    fn should_panic_if_vec_was_expected_to_be_not_empty_and_is() {
        assert_that(&Vec::<u8>::new()).is_not_empty();
    }

    #[test]
    fn should_not_panic_if_vec_contains_exactly_the_expected_values_in_any_order() {
        assert_that(&vec![1, 2, 3]).contains_exactly_in_any_order(&vec![&1, &2, &3]);
        assert_that(&vec![1, 2, 3]).contains_exactly_in_any_order(&vec![&3, &1, &2]);
    }

    #[test]
    #[should_panic(expected = "\u{1b}[31mexpected: vec to have length <3>\n\t \
        but was: <2>\u{1b}[0m")]
    fn should_panic_if_vec_does_not_contain_all_expected_values() {
        assert_that(&vec![1, 2]).contains_exactly_in_any_order(&vec![&1, &2, &3]);
    }

    #[test]
    #[should_panic(expected = "\u{1b}[31mexpected: vec to have length <3>\n\t \
        but was: <4>\u{1b}[0m")]
    fn should_panic_if_vec_contains_more_than_the_expected_values() {
        assert_that(&vec![1, 2, 3, 3]).contains_exactly_in_any_order(&vec![&1, &2, &3]);
    }

    #[test]
    #[should_panic(
        expected = "\u{1b}[31mexpected: iterator to contain items <[1, 2, 3]>\n\t \
            but was: <[1, 2, 4]>\u{1b}[0m"
    )]
    fn should_panic_if_vec_contains_the_same_number_but_different_values_than_the_expected_ones() {
        assert_that(&vec![1, 2, 4]).contains_exactly_in_any_order(&vec![&1, &2, &3]);
    }
}
