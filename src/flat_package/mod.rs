use std::hash::Hash;
use std::rc::Rc;
pub type Tree<C, T> = Vec<(Key<C>, Value<T>)>;
pub type Key<C> = Vec<C>;
pub type Value<T> = (Rc<T>, usize);
#[derive(Clone)]
pub struct Package<CATEGORY: Hash + Ord + Clone, TYPE: ?Sized, const N: usize> {
    tree: Tree<CATEGORY, TYPE>,
}
#[derive(Debug)]
pub enum PackageError {
    CategoryTooBig,
    CategoryTooSmall,
    NoFound,
    AllReadyExist,
}

pub type Result<T> = std::result::Result<T, PackageError>;
impl<C: Hash + Ord + Clone, T: ?Sized, const N: usize> Default for Package<C, T, N> {
    ///Construct Package struct with Package::new() function
    fn default() -> Self {
        Self::new()
    }
}

impl<C: Hash + Ord + Clone, T: ?Sized, const N: usize> Package<C, T, N> {
    ///Construct new empty struct with empty Vec inside
    pub fn new() -> Self {
        Self { tree: Vec::new() }
    }
    ///Return sum of elements size.
    pub fn size(&self) -> usize {
        self.tree.iter().map(|x| x.1 .1).sum()
    }
    pub fn len(&self) -> usize {
        self.tree.len()
    }
    pub fn is_empty(&self) -> bool {
        self.tree.is_empty()
    }
    pub fn as_container(&self) -> &Tree<C, T> {
        &self.tree
    }
    /// # Safety
    ///
    /// Changing keys can break the container,you shall use fix() after changing.
    ///
    pub unsafe fn as_mut_container(&mut self) -> &mut Tree<C, T> {
        &mut self.tree
    }
    pub fn get(&self, category: &[C]) -> Result<Vec<&(Key<C>, Value<T>)>> {
        match category.len() {
            n if n < N => {
                let tmp = self
                    .tree
                    .iter()
                    .filter(|x| x.0.iter().zip(category).all(|x| x.0 == x.1))
                    .collect::<Vec<_>>();
                Ok(tmp)
            }
            n if n == N => Ok(vec![
                &self.tree[self
                    .tree
                    .binary_search_by_key(&category, |x| &x.0)
                    .map_err(|_| PackageError::NoFound)
                    .unwrap()],
            ]),
            _ => Err(PackageError::CategoryTooBig),
        }
    }
    pub fn get_values(&self, category: &[C]) -> Result<Vec<&Value<T>>> {
        match category.len() {
            n if n < N => {
                let tmp = self
                    .tree
                    .iter()
                    .filter(|x| x.0.iter().zip(category).all(|x| x.0 == x.1))
                    .map(|x| &x.1)
                    .collect::<Vec<_>>();
                Ok(tmp)
            }
            n if n == N => Ok(vec![
                &self.tree[self
                    .tree
                    .binary_search_by_key(&category, |x| &x.0)
                    .map_err(|_| PackageError::NoFound)
                    .unwrap()]
                .1,
            ]),
            _ => Err(PackageError::CategoryTooBig),
        }
    }

    pub fn get_values_mut(&mut self, category: &[C]) -> Result<Vec<&mut Value<T>>> {
        match category.len() {
            n if n < N => {
                let tmp = self
                    .tree
                    .iter_mut()
                    .filter(|x| x.0.iter().zip(category).all(|x| x.0 == x.1))
                    .map(|x| &mut x.1)
                    .collect::<Vec<_>>();
                Ok(tmp)
            }
            n if n == N => {
                let tmp = self
                    .tree
                    .binary_search_by_key(&category, |x| &x.0)
                    .map_err(|_| PackageError::NoFound)?;
                Ok(vec![&mut self.tree[tmp].1])
            }
            _ => Err(PackageError::CategoryTooBig),
        }
    }
    /// # Safety
    ///
    /// Changing keys can break the container,you shall use fix() after changing.
    ///
    pub unsafe fn get_mut(&mut self, category: &[C]) -> Result<Vec<&mut (Key<C>, Value<T>)>> {
        match category.len() {
            n if n < N => {
                let tmp = self
                    .tree
                    .iter_mut()
                    .filter(|x| x.0.iter().zip(category).all(|x| x.0 == x.1))
                    .collect::<Vec<_>>();
                Ok(tmp)
            }
            n if n == N => {
                let tmp = self
                    .tree
                    .binary_search_by_key(&category, |x| &x.0)
                    .map_err(|_| PackageError::NoFound)?;
                Ok(vec![&mut self.tree[tmp]])
            }
            _ => Err(PackageError::CategoryTooBig),
        }
    }
    pub fn count(&self, category: &[C]) -> Result<usize> {
        match category.len() {
            n if n < N => Ok(self
                .tree
                .iter()
                .filter(|x| x.0.iter().zip(category).all(|x| x.0 == x.1))
                .map(|_| ())
                .count()),
            n if n == N => match self.tree.binary_search_by_key(&category, |x| &x.0) {
                Ok(_) => Ok(1),
                Err(_) => Ok(0),
            },
            _ => Err(PackageError::CategoryTooBig),
        }
    }
    pub fn remove(&mut self, category: &[C]) -> Result<Vec<Value<T>>> {
        match category.len() {
            n if n < N => {
                let tmp = self
                    .tree
                    .iter()
                    .zip(0..)
                    .filter(|x| x.0 .0.iter().zip(category).all(|x| x.0 == x.1))
                    .map(|x| x.1)
                    .collect::<Vec<_>>();
                let mut res = Vec::new();
                for i in tmp.into_iter() {
                    res.push(self.tree.remove(i).1);
                }
                Ok(res)
            }
            n if n == N => Ok(vec![
                self.tree
                    .remove(
                        self.tree
                            .binary_search_by_key(&category, |x| &x.0)
                            .map_err(|_| PackageError::NoFound)?,
                    )
                    .1,
            ]),
            _ => Err(PackageError::CategoryTooBig),
        }
    }
    pub fn add(&mut self, category: &[C], data: Rc<T>, size: usize) -> Result<()> {
        if category.len() > N {
            return Err(PackageError::CategoryTooBig);
        }
        if N == category.len() {
            let i = match self.tree.binary_search_by_key(&category, |x| &x.0) {
                Ok(_) => Err(PackageError::AllReadyExist),
                Err(tmp) => Ok(tmp),
            }?;
            self.tree.insert(i, (category.to_owned(), (data, size)))
        }
        Ok(())
    }
    pub fn iter(&self) -> impl Iterator<Item = &(Key<C>, Value<T>)> + '_ {
        self.tree.iter()
    }
    pub fn values(&self) -> impl Iterator<Item = &Value<T>> + '_ {
        self.tree.iter().map(|x| &x.1)
    }
    /// # Safety
    ///
    /// Changing keys can break the container,you shall use fix() after changing.
    ///
    pub unsafe fn iter_mut(&mut self) -> impl Iterator<Item = &mut (Key<C>, Value<T>)> + '_ {
        self.tree.iter_mut()
    }
    pub fn values_mut(&mut self) -> impl Iterator<Item = &mut Value<T>> + '_ {
        self.tree.iter_mut().map(|x| &mut x.1)
    }
    pub fn seperete_by_count(&mut self, count: usize) -> Option<Self> {
        let mut res = Tree::<C, T>::new();
        for _ in 0..count {
            let tmp = self.tree.pop();
            if tmp.is_none() {
                break;
            }
            let tmp = tmp.unwrap();
            res.push((tmp.0, (tmp.1 .0, tmp.1 .1)));
        }
        if res.is_empty() {
            return None;
        }
        Some(Self { tree: res })
    }
    pub fn seperete_to_count(&mut self, count: usize) -> Option<Self> {
        let mut res = Tree::<C, T>::new();

        for _ in 0..count {
            let tmp = self.tree.pop();
            if tmp.is_none() {
                break;
            }
            let tmp = tmp.unwrap();
            res.push((tmp.0, (tmp.1 .0, tmp.1 .1)));
        }
        std::mem::swap(&mut self.tree, &mut res);
        if res.is_empty() {
            return None;
        }
        Some(Self { tree: res })
    }
    pub fn seperate_by<F: FnMut(&(Key<C>, Value<T>)) -> bool>(&mut self, mut f: F) -> Option<Self> {
        let tmp = self
            .tree
            .iter()
            .zip(0..)
            .filter(|&(x, _)| f(x))
            .map(|x| x.1)
            .collect::<Vec<_>>();
        let mut res = Vec::new();
        for i in tmp.into_iter() {
            let tmp = self.tree.swap_remove(i);
            res.push(tmp);
        }
        if res.is_empty() {
            return None;
        }
        res.sort_by_cached_key(|x| x.0.clone());
        self.tree.sort_by_cached_key(|x| x.0.clone());
        Some(Self { tree: res })
    }
    pub fn seperete_by_size(&mut self, mut size: usize) -> Option<Self> {
        let mut res = Tree::<C, T>::new();
        for _ in 0..self.tree.len() {
            let tmp = match self.tree.iter().zip(0..).find(|x| {
                if x.0 .1 .1 <= size && size > 0 {
                    size -= x.0 .1 .1;
                    true
                } else {
                    false
                }
            }) {
                Some(x) => Some(self.tree.remove(x.1)),
                None => None,
            };
            if tmp.is_none() {
                break;
            }
            let tmp = tmp.unwrap();
            res.push((tmp.0, (tmp.1 .0, tmp.1 .1)));
        }
        if res.is_empty() {
            return None;
        }
        Some(Self { tree: res })
    }
    pub fn seperete_to_size(&mut self, mut size: usize) -> Option<Self> {
        let mut res = Tree::<C, T>::new();
        let mut _size = 0;
        for _ in 0..self.tree.len() {
            let tmp = match self.tree.iter().zip(0..).find(|x| {
                if x.0 .1 .1 <= size && size > 0 {
                    size -= x.0 .1 .1;
                    true
                } else {
                    false
                }
            }) {
                Some(x) => Some(self.tree.remove(x.1)),
                None => None,
            };
            if tmp.is_none() {
                break;
            }
            let tmp = tmp.unwrap();
            res.push((tmp.0, (tmp.1 .0, tmp.1 .1)));
        }
        std::mem::swap(&mut self.tree, &mut res);
        if res.is_empty() {
            return None;
        }
        Some(Self { tree: res })
    }
    pub fn clear(&mut self) {
        self.tree.clear()
    }
    pub fn fix(&mut self) {
        self.tree.sort_by_cached_key(|x| x.0.clone());
    }
}
impl<C: Hash + Ord + Clone, T: ?Sized, const N: usize> IntoIterator for Package<C, T, N> {
    type Item = (Key<C>, Value<T>);

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.tree.into_iter()
    }
}
