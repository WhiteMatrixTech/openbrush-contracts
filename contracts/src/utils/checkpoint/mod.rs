// Copyright (c) 2012-2023 727.ventures
//
// Permission is hereby granted, free of charge, to any person obtaining
// a copy of this software and associated documentation files (the"Software"),
// to deal in the Software without restriction, including
// without limitation the rights to use, copy, modify, merge, publish,
// distribute, sublicense, and/or sell copies of the Software, and to
// permit persons to whom the Software is furnished to do so, subject to
// the following conditions:
//
// The above copyright notice and this permission notice shall be
// included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE
// LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
// WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

use ink::prelude::vec::Vec;
pub use crate::traits::errors::CheckpointsError;

#[derive(scale::Decode, scale::Encode, Default, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout))]
pub struct Checkpoints {
    pub checkpoints: Vec<Checkpoint>,
}

#[derive(scale::Decode, scale::Encode, Default, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout))]
pub struct Checkpoint {
    pub key: u32,
    pub value: u128,
}

impl Checkpoints {
    ///Pushes a (`key`, `value`) pair into a Trace224 so that it is stored as the checkpoint.
    /// Returns previous value and new value.
    pub fn push(&mut self, key: u32, value: u128) -> Result<(u128, u128), CheckpointsError> {
        todo!()
    }

    ///Returns the value in the first (oldest) checkpoint with key greater or equal than the search key, or zero if there is none.
    pub fn lower_lookup(&self, key: u32) -> Option<u128> {
        todo!()
    }

    ///Returns the value in the last (most recent) checkpoint with key lower or equal than the search key, or zero if there is none.
    pub fn upper_lookup(&self, key: u32) -> Option<u128> {
        todo!()
    }

    /// Returns the value in the last (most recent) checkpoint with key lower or equal than the search key, or zero if there is none.
    ///
    /// NOTE: This is a variant of {upperLookup} that is optimised to find "recent" checkpoint (checkpoints with high keys).
    pub fn upper_lookup_recent(&self, key: u32) -> Option<u128> {
        todo!()
    }

    ///Returns the value in the most recent checkpoint, or 0 if there are no checkpoints.
    pub fn latest(&self) -> u128 {
        todo!()
    }

    ///Returns whether there is a checkpoint in the structure (i.e. it is not empty), and if so the key and value
    ///in the most recent checkpoint.
    pub fn latest_checkpoint(&self) -> (bool, u32, u128) {
        todo!()
    }

    ///Returns the number of checkpoint.
    pub fn len(&self) -> usize {
        todo!()
    }

    pub fn at(&self, index: usize) -> Option<&Checkpoint> {
        todo!()
    }

    fn _insert(&mut self, key: u32, value: u128) -> Result<(u128, u128), CheckpointsError> {
        todo!()
    }

    fn _upper_binary_lookup(&self, key: u32, mut low: usize, mut high: usize) -> usize {
        todo!()
    }

    fn _lower_binary_lookup(&self, key: u32, mut low: usize, mut high: usize) -> usize {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[ink::test]
    fn push_works() {
        let mut checkpoints = Checkpoints::default();

        let (last, value) = checkpoints.push(1, 1).unwrap();
        assert_eq!(last, 0);
        assert_eq!(value, 1);
        assert_eq!(checkpoints.len(), 1);
    }

    #[ink::test]
    fn lower_lookup_works() {
        let mut checkpoints = Checkpoints::default();
        checkpoints.push(1, 1).unwrap();
        checkpoints.push(2, 2).unwrap();
        checkpoints.push(5, 5).unwrap();
        assert_eq!(checkpoints.lower_lookup(0), Some(1));
        assert_eq!(checkpoints.lower_lookup(1), Some(1));
        assert_eq!(checkpoints.lower_lookup(2), Some(2));
        assert_eq!(checkpoints.lower_lookup(3), Some(5));
        assert_eq!(checkpoints.lower_lookup(5), Some(5));
        assert_eq!(checkpoints.lower_lookup(6), None);
    }

    #[ink::test]
    fn upper_lookup_works() {
        let mut checkpoints = Checkpoints::default();

        checkpoints.push(1, 1).unwrap();
        checkpoints.push(2, 2).unwrap();
        checkpoints.push(5, 5).unwrap();
        assert_eq!(checkpoints.upper_lookup(0), None);
        assert_eq!(checkpoints.upper_lookup(1), Some(1));
        assert_eq!(checkpoints.upper_lookup(2), Some(2));
        assert_eq!(checkpoints.upper_lookup(3), Some(2));
        assert_eq!(checkpoints.upper_lookup(5), Some(5));
        assert_eq!(checkpoints.upper_lookup(6), Some(5));
    }

    #[ink::test]
    fn upper_lookup_recent_works() {
        let mut checkpoints = Checkpoints::default();

        checkpoints.push(1, 1).unwrap();
        checkpoints.push(2, 2).unwrap();
        checkpoints.push(5, 5).unwrap();
        assert_eq!(checkpoints.upper_lookup_recent(0), None);
        assert_eq!(checkpoints.upper_lookup_recent(1), Some(1));
        assert_eq!(checkpoints.upper_lookup_recent(2), Some(2));
        assert_eq!(checkpoints.upper_lookup_recent(3), Some(2));
        assert_eq!(checkpoints.upper_lookup_recent(5), Some(5));
        assert_eq!(checkpoints.upper_lookup_recent(6), Some(5));
    }

    #[ink::test]
    fn latest_works() {
        let mut checkpoints = Checkpoints::default();
        assert_eq!(checkpoints.latest(), 0);
        checkpoints.push(1, 1).unwrap();
        checkpoints.push(2, 2).unwrap();
        checkpoints.push(5, 5).unwrap();
        assert_eq!(checkpoints.latest(), 5);
    }

    #[ink::test]
    fn latest_checkpoint_works() {
        let mut checkpoints = Checkpoints::default();
        assert_eq!(checkpoints.latest_checkpoint(), (false, 0, 0));
        checkpoints.push(1, 1).unwrap();
        checkpoints.push(2, 2).unwrap();
        checkpoints.push(5, 5).unwrap();
        assert_eq!(checkpoints.latest_checkpoint(), (true, 5, 5));
    }

    #[ink::test]
    fn len_works() {
        let mut checkpoints = Checkpoints::default();
        assert_eq!(checkpoints.len(), 0);
        checkpoints.push(1, 1).unwrap();
        checkpoints.push(2, 2).unwrap();
        checkpoints.push(5, 5).unwrap();
        assert_eq!(checkpoints.len(), 3);
    }
}
