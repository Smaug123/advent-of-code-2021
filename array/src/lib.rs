pub mod array {

    #[derive(Debug, Clone)]
    pub struct Array<T> {
        row_len: usize,
        elts: Vec<T>,
    }

    pub struct ArrayIterator<'a, T>
    where
        T: 'a,
    {
        arr: &'a Array<T>,
        curr_count: usize,
    }

    impl<'a, T> Iterator for ArrayIterator<'a, T>
    where
        T: 'a,
        T: std::fmt::Display,
    {
        type Item = &'a T;

        fn next(&mut self) -> Option<Self::Item> {
            if self.curr_count >= self.arr.elts.len() {
                return None;
            }

            let to_ret = &self.arr.elts[self.curr_count];
            self.curr_count += 1;
            Some(to_ret)
        }
    }

    impl<T> Array<T> {
        pub fn col_len(&self) -> usize {
            self.elts.len() / self.row_len
        }
        pub fn row_len(&self) -> usize {
            self.row_len
        }
        pub fn get_clone(&self, row: usize, col: usize) -> Option<T>
        where
            T: Clone,
        {
            let index = row * self.row_len + col;
            if row < self.row_len && col < self.col_len() && index < self.elts.len() {
                Some(self.elts[index].clone())
            } else {
                None
            }
        }
        pub fn get(&self, row: usize, col: usize) -> Option<&T> {
            let index = row * self.row_len + col;
            if row < self.col_len() && col < self.row_len && index < self.elts.len() {
                Some(&self.elts[index])
            } else {
                None
            }
        }
        pub fn get_unsafe(&self, row: usize, col: usize) -> &T {
            let index = row * self.row_len + col;
            &self.elts[index]
        }
        pub fn set(&mut self, row: usize, col: usize, val: T)
        where
            T: Clone,
        {
            self.elts[row * self.row_len + col] = val;
        }

        /// Make a 2D array from its elements, given as concatenated rows.
        pub fn make(elts: Vec<T>, row_len: usize) -> Array<T> {
            Array { elts, row_len }
        }

        pub fn make_default(row_len: usize, col_len: usize, elt: T) -> Array<T>
        where
            T: Copy,
        {
            let mut v = Vec::with_capacity(row_len * col_len);
            v.resize(row_len * col_len, elt);
            Array { elts: v, row_len }
        }

        pub fn apply<F>(&mut self, f: F)
        where
            F: Fn(T) -> T,
            T: Clone,
        {
            for elt in self.elts.iter_mut() {
                *elt = f(elt.clone());
            }
        }

        pub fn apply_at<F>(&mut self, row: usize, col: usize, f: F)
        where
            F: Fn(T) -> T,
            T: Clone,
        {
            self.set(row, col, f(self.get(row, col).unwrap().clone()));
        }

        pub fn from_rows<I, J>(rows: I) -> Array<T>
        where
            I: Iterator<Item = J>,
            J: Iterator<Item = T>,
        {
            let mut elts = Vec::new();
            let mut row_len = 0;

            for row in rows {
                if row_len == 0 {
                    for entry in row {
                        elts.push(entry);
                        row_len += 1;
                    }
                } else {
                    elts.extend(row);
                }
            }

            Array { elts, row_len }
        }

        pub fn iter(&self) -> ArrayIterator<'_, T> {
            ArrayIterator {
                curr_count: 0,
                arr: self,
            }
        }
    }

    impl std::fmt::Display for Array<bool> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            for row in 0..self.row_len {
                for col in 0..self.col_len() {
                    write!(
                        f,
                        "{}",
                        if *self.get(row, col).unwrap() {
                            'X'
                        } else {
                            '.'
                        }
                    )?;
                }
                writeln!(f)?;
            }
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::array::*;

    #[test]
    fn iteration() {
        let data: Vec<_> = (1..100).collect();
        let arr = Array::make(data.iter().cloned().collect(), 10);

        assert_eq!(arr.iter().cloned().collect::<Vec<_>>(), data);
    }

    #[test]
    fn from_rows() {
        let data: Vec<Vec<_>> = (0..12)
            .map(|row| (10 * row..=10 * row + 9).collect())
            .collect();
        let arr = Array::from_rows(data.iter().map(|row| row.iter().cloned()));

        assert_eq!(
            arr.iter().cloned().collect::<Vec<_>>(),
            (0..120).collect::<Vec<_>>()
        );
        assert_eq!(arr.row_len(), 10);
        assert_eq!(arr.col_len(), 12);
    }
}
