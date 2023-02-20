use std::hash::Hash;
use std::rc::Rc;
pub type Tree<C, T> = Vec<(Key<C>, Value<T>)>;
pub type Key<C> = Vec<C>;
pub type Value<T> = (Rc<T>, usize);
#[derive(Clone)]
pub struct Package<CATEGORY: Clone, TYPE: ?Sized, const N: usize> {
    data: Tree<CATEGORY, TYPE>,
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
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    /// It returns the size of the data.
    ///
    /// Returns:
    ///
    /// The sum of the sizes of the files in the container.
    pub fn size(&self) -> usize {
        self.data.iter().map(|x| x.1 .1).sum()
    }

    /// `len` returns the length of the `data` vector
    ///
    /// Returns:
    ///
    /// The length of the data vector.
    pub fn len(&self) -> usize {
        self.data.len()
    }
    /// `is_empty` returns `true` if the `data` field is empty
    ///
    /// Returns:
    ///
    /// A boolean value.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    /// Return a reference to the data field of the current object.
    ///
    /// Returns:
    ///
    /// A reference to the data field of the TreeNode struct.
    pub fn as_container(&self) -> &Tree<C, T> {
        &self.data
    }
    /// # Safety
    ///
    /// The function is unsafe because it returns a mutable reference to a private field.
    /// Changing keys can break the container,you should use fix() after changing.
    ///
    /// Returns:
    ///
    /// A mutable reference to the data field of the TreeNode.
    pub unsafe fn as_mut_container(&mut self) -> &mut Tree<C, T> {
        &mut self.data
    }
    /// > The function takes a vector of categories and returns a vector of references to the key-value
    /// pairs that match the categories
    ///
    /// Arguments:
    ///
    /// * `category`: The category of the data you want to get.
    ///
    /// Returns:
    ///
    /// A vector of references to the tuples of the key and value.
    pub fn get(&self, category: &[C]) -> Result<Vec<&(Key<C>, Value<T>)>> {
        match category.len() {
            n if n < N => {
                let tmp = self
                    .data
                    .iter()
                    .filter(|x| x.0.iter().zip(category).all(|x| x.0 == x.1))
                    .collect::<Vec<_>>();
                Ok(tmp)
            }
            n if n == N => Ok(vec![
                &self.data[self
                    .data
                    .binary_search_by_key(&category, |x| &x.0)
                    .map_err(|_| PackageError::NoFound)
                    .unwrap()],
            ]),
            _ => Err(PackageError::CategoryTooBig),
        }
    }
    /// > This function returns a vector of references to the values in the package that match
    /// the category
    ///
    /// Arguments:
    ///
    /// * `category`: The category to search for.
    ///
    /// Returns:
    ///
    /// A vector of references to the values in the package.
    pub fn get_values(&self, category: &[C]) -> Result<Vec<&Value<T>>> {
        match category.len() {
            n if n < N => {
                let tmp = self
                    .data
                    .iter()
                    .filter(|x| x.0.iter().zip(category).all(|x| x.0 == x.1))
                    .map(|x| &x.1)
                    .collect::<Vec<_>>();
                Ok(tmp)
            }
            n if n == N => Ok(vec![
                &self.data[self
                    .data
                    .binary_search_by_key(&category, |x| &x.0)
                    .map_err(|_| PackageError::NoFound)
                    .unwrap()]
                .1,
            ]),
            _ => Err(PackageError::CategoryTooBig),
        }
    }

    /// > This function returns a vector of mutable references to the values in the package that match
    /// the category
    ///
    /// Arguments:
    ///
    /// * `category`: The category to search for.
    ///
    /// Returns:
    ///
    /// A vector of references to the values in the package.
    pub fn get_values_mut(&mut self, category: &[C]) -> Result<Vec<&mut Value<T>>> {
        match category.len() {
            n if n < N => {
                let tmp = self
                    .data
                    .iter_mut()
                    .filter(|x| x.0.iter().zip(category).all(|x| x.0 == x.1))
                    .map(|x| &mut x.1)
                    .collect::<Vec<_>>();
                Ok(tmp)
            }
            n if n == N => {
                let tmp = self
                    .data
                    .binary_search_by_key(&category, |x| &x.0)
                    .map_err(|_| PackageError::NoFound)?;
                Ok(vec![&mut self.data[tmp].1])
            }
            _ => Err(PackageError::CategoryTooBig),
        }
    }
    ///
    /// > This function returns a mutable reference to a vector of tuples of a key and a value
    ///
    /// Arguments:
    ///
    /// * `category`: The category of the data you want to get.
    ///
    /// Returns:
    ///
    /// A vector of mutable references to the key and value pairs.
    /// # Safety
    ///
    /// Changing keys can break the container,you shall use fix() after changing.
    pub unsafe fn get_mut(&mut self, category: &[C]) -> Result<Vec<&mut (Key<C>, Value<T>)>> {
        match category.len() {
            n if n < N => {
                let tmp = self
                    .data
                    .iter_mut()
                    .filter(|x| x.0.iter().zip(category).all(|x| x.0 == x.1))
                    .collect::<Vec<_>>();
                Ok(tmp)
            }
            n if n == N => {
                let tmp = self
                    .data
                    .binary_search_by_key(&category, |x| &x.0)
                    .map_err(|_| PackageError::NoFound)?;
                Ok(vec![&mut self.data[tmp]])
            }
            _ => Err(PackageError::CategoryTooBig),
        }
    }
    /// > If the category is smaller than the number of dimensions, then count the number of times the
    /// category appears in the data. If the category is the same size as the number of dimensions, then
    /// check if the category is in the data. If the category is larger than the number of dimensions,
    /// then return an error
    ///
    /// Arguments:
    ///
    /// * `category`: The category to count.
    ///
    /// Returns:
    ///
    /// A Result<usize>
    pub fn count(&self, category: &[C]) -> Result<usize> {
        match category.len() {
            n if n < N => Ok(self
                .data
                .iter()
                .filter(|x| x.0.iter().zip(category).all(|x| x.0 == x.1))
                .map(|_| ())
                .count()),
            n if n == N => match self.data.binary_search_by_key(&category, |x| &x.0) {
                Ok(_) => Ok(1),
                Err(_) => Ok(0),
            },
            _ => Err(PackageError::CategoryTooBig),
        }
    }
    /// > This function removes a value from the package, given a category
    ///
    /// Arguments:
    ///
    /// * `category`: The category of the data you want to remove.
    ///
    /// Returns:
    ///
    /// A vector of values.
    pub fn remove(&mut self, category: &[C]) -> Result<Vec<Value<T>>> {
        match category.len() {
            n if n < N => {
                let tmp = self
                    .data
                    .iter()
                    .zip(0..)
                    .filter(|x| x.0 .0.iter().zip(category).all(|x| x.0 == x.1))
                    .map(|x| x.1)
                    .collect::<Vec<_>>();
                let mut res = Vec::new();
                for i in tmp.into_iter() {
                    res.push(self.data.remove(i).1);
                }
                Ok(res)
            }
            n if n == N => Ok(vec![
                self.data
                    .remove(
                        self.data
                            .binary_search_by_key(&category, |x| &x.0)
                            .map_err(|_| PackageError::NoFound)?,
                    )
                    .1,
            ]),
            _ => Err(PackageError::CategoryTooBig),
        }
    }
    /// > If the category is too big, return an error. If the category is the right size, insert the
    /// data into the vector
    ///
    /// Arguments:
    ///
    /// * `category`: The category of the data.
    /// * `data`: The data to be stored.
    /// * `size`: The size of the data in bytes.
    ///
    /// Returns:
    ///
    /// A Result<()>
    pub fn add(&mut self, category: &[C], data: Rc<T>, size: usize) -> Result<()> {
        if category.len() > N {
            return Err(PackageError::CategoryTooBig);
        }
        if N == category.len() {
            let i = match self.data.binary_search_by_key(&category, |x| &x.0) {
                Ok(_) => Err(PackageError::AllReadyExist),
                Err(tmp) => Ok(tmp),
            }?;
            self.data.insert(i, (category.to_owned(), (data, size)))
        }
        Ok(())
    }
    /// This function returns an iterator over the data .
    pub fn iter(&self) -> impl Iterator<Item = &(Key<C>, Value<T>)> + '_ {
        self.data.iter()
    }
    /// Return an iterator over the values in the data
    pub fn values(&self) -> impl Iterator<Item = &Value<T>> + '_ {
        self.data.iter().map(|x| &x.1)
    }

    ///
    /// This function returns an iterator over mutable references to the key-value pairs in the map.
    ///  # Safety
    ///
    /// Changing keys can break the container,you shall use fix() after changing.
    pub unsafe fn iter_mut(&mut self) -> impl Iterator<Item = &mut (Key<C>, Value<T>)> + '_ {
        self.data.iter_mut()
    }
    /// This function returns an iterator over mutable references to the values in the data.
    pub fn values_mut(&mut self) -> impl Iterator<Item = &mut Value<T>> + '_ {
        self.data.iter_mut().map(|x| &mut x.1)
    }
    /// It takes a tree and a count and returns a new tree with the last count elements of the original
    /// tree.
    ///
    /// Arguments:
    ///
    /// * `count`: The number of elements to be separated.
    ///
    /// Returns:
    ///
    /// A new tree with the first `count` elements of the original tree.
    pub fn seperete_by_count(&mut self, count: usize) -> Option<Self> {
        let mut res = Tree::<C, T>::new();
        for _ in 0..count {
            let tmp = self.data.pop();
            if tmp.is_none() {
                break;
            }
            let tmp = tmp.unwrap();
            res.push((tmp.0, (tmp.1 .0, tmp.1 .1)));
        }
        if res.is_empty() {
            return None;
        }
        Some(Self { data: res })
    }
    /// It takes a tree and separates it into two trees.
    ///
    /// Arguments:
    ///
    /// * `count`: The number of elements to be separated.
    ///
    /// Returns:
    ///
    /// A new tree with the first `len() - count` elements of the original tree.
    pub fn seperete_to_count(&mut self, count: usize) -> Option<Self> {
        let mut res = Tree::<C, T>::new();

        for _ in 0..count {
            let tmp = self.data.pop();
            if tmp.is_none() {
                break;
            }
            let tmp = tmp.unwrap();
            res.push((tmp.0, (tmp.1 .0, tmp.1 .1)));
        }
        std::mem::swap(&mut self.data, &mut res);
        if res.is_empty() {
            return None;
        }
        Some(Self { data: res })
    }
    /// > This function takes a mutable reference to a `HashMap` and a function that takes a reference
    /// to a `Key` and `Value` pair and returns a `bool`. It then returns an `Option<HashMap>` that
    /// contains all the `Key` and `Value` pairs that the function returns `true` for
    ///
    /// Arguments:
    ///
    /// * `f`: A function that takes a reference to a tuple of a key and a value, and returns a bool.
    ///
    /// Returns:
    ///
    /// A new `Map` with the elements that satisfy the predicate.
    pub fn seperate_by<F: FnMut(&(Key<C>, Value<T>)) -> bool>(&mut self, mut f: F) -> Option<Self> {
        let tmp = self
            .data
            .iter()
            .zip(0..)
            .filter(|&(x, _)| f(x))
            .map(|x| x.1)
            .collect::<Vec<_>>();
        let mut res = Vec::new();
        for i in tmp.into_iter() {
            let tmp = self.data.swap_remove(i);
            res.push(tmp);
        }
        if res.is_empty() {
            return None;
        }
        res.sort_by_cached_key(|x| x.0.clone());
        self.data.sort_by_cached_key(|x| x.0.clone());
        Some(Self { data: res })
    }
    /// It takes a tree and a size and returns a tree with the same data but with the size of the data
    /// being less than or equal to the size.
    ///
    /// Arguments:
    ///
    /// * `size`: The size of the tree to be separated.
    ///
    /// Returns:
    ///
    /// A new tree with the same data type and the same comparator, but with the data that is less than
    /// or equal to the size.
    pub fn seperete_by_size(&mut self, mut size: usize) -> Option<Self> {
        let mut res = Tree::<C, T>::new();
        for _ in 0..self.data.len() {
            let tmp = match self.data.iter().zip(0..).find(|x| {
                if x.0 .1 .1 <= size && size > 0 {
                    size -= x.0 .1 .1;
                    true
                } else {
                    false
                }
            }) {
                Some(x) => Some(self.data.remove(x.1)),
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
        Some(Self { data: res })
    }
    /// It takes a tree and a size and returns a tree with the size of the data in the tree.
    ///
    /// Arguments:
    ///
    /// * `size`: The size of the data to be separated.
    ///
    /// Returns:
    ///
    /// A new tree with the first size elements of the original tree.
    pub fn seperete_to_size(&mut self, mut size: usize) -> Option<Self> {
        let mut res = Tree::<C, T>::new();
        let mut _size = 0;
        for _ in 0..self.data.len() {
            let tmp = match self.data.iter().zip(0..).find(|x| {
                if x.0 .1 .1 <= size && size > 0 {
                    size -= x.0 .1 .1;
                    true
                } else {
                    false
                }
            }) {
                Some(x) => Some(self.data.remove(x.1)),
                None => None,
            };
            if tmp.is_none() {
                break;
            }
            let tmp = tmp.unwrap();
            res.push((tmp.0, (tmp.1 .0, tmp.1 .1)));
        }
        std::mem::swap(&mut self.data, &mut res);
        if res.is_empty() {
            return None;
        }
        Some(Self { data: res })
    }
    /// It clears the data.
    pub fn clear(&mut self) {
        self.data.clear()
    }
    /// It sorts the data by the first element of the tuple
    pub fn fix(&mut self) {
        self.data.sort_by_cached_key(|x| x.0.clone());
    }
}
impl<C: Hash + Ord + Clone, T: ?Sized, const N: usize> IntoIterator for Package<C, T, N> {
    type Item = (Key<C>, Value<T>);

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}
