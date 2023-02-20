use crate::flat_package::{Key, Package, Value};

use std::hash::Hash;
#[derive(Clone)]
pub struct PackagePack<CATEGORY: Hash + Ord + Clone, TYPE: ?Sized, const N: usize> {
    packages: Vec<Package<CATEGORY, TYPE, N>>,
    max_size: usize,
    max_len: usize,
}
impl<C: Hash + Ord + Clone, T: ?Sized, const N: usize> PackagePack<C, T, N> {
    pub fn new(package: Package<C, T, N>, max_size: usize, max_len: usize) -> Self {
        let mut tmp = Self {
            packages: Vec::new(),
            max_size,
            max_len,
        };
        tmp.add(package);
        tmp
    }
    fn add(&mut self, mut package: Package<C, T, N>) {
        let max_size = self.max_size;
        let max_len = self.max_len;
        let mut tmp = Vec::new();
        if max_len != 0 {
            while package.len() > max_len {
                match package.seperete_by_count(max_len) {
                    None => {}
                    Some(i) => tmp.push(i),
                };
            }
        }
        let mut tmp1 = Vec::new();
        if max_size != 0 {
            for mut package in tmp {
                while package.len() > max_len {
                    match package.seperete_by_size(max_size) {
                        None => {
                            if package.is_empty() {
                                tmp1.push(package);
                            }
                            break;
                        }
                        Some(i) => tmp1.push(i),
                    };
                }
            }
        }
        self.packages.append(&mut tmp1);
    }
    pub fn iter(&self) -> impl Iterator<Item = &Package<C, T, N>> + '_ {
        self.packages.iter()
    }
    pub unsafe fn iter_mut(&mut self) -> impl Iterator<Item = &mut Package<C, T, N>> + '_ {
        self.packages.iter_mut()
    }
    pub fn iter_item(&mut self) -> impl Iterator<Item = &(Key<C>, Value<T>)> + '_ {
        self.packages.iter().flat_map(|x| x.iter())
    }
    pub unsafe fn iter_mut_item(&mut self) -> impl Iterator<Item = &mut (Key<C>, Value<T>)> + '_ {
        self.packages.iter_mut().flat_map(|x| x.iter_mut())
    }
    pub fn clear(&mut self) {
        self.packages.clear()
    }
}

impl<CATEGORY: Hash + Ord + Clone, TYPE: ?Sized, const N: usize> IntoIterator
    for PackagePack<CATEGORY, TYPE, N>
{
    type Item = Package<CATEGORY, TYPE, N>;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.packages.into_iter()
    }
}
