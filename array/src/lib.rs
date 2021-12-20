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
        curr_row: usize,
        curr_col: usize,
    }

    impl<'a, T> Iterator for ArrayIterator<'a, T>
    where
        T: 'a,
    {
        type Item = &'a T;

        fn next(&mut self) -> Option<Self::Item> {
            let to_ret = self.arr.get(self.curr_row, self.curr_col);

            self.curr_col += 1;
            if self.curr_col >= self.arr.row_len {
                self.curr_col = 0;
                self.curr_row += 1;
            }

            to_ret
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
            T: Copy,
        {
            let index = row * self.row_len + col;
            if row < self.row_len && col < self.col_len() && index < self.elts.len() {
                Some(self.elts[index])
            } else {
                None
            }
        }
        pub fn get(&self, row: usize, col: usize) -> Option<&T> {
            let index = row * self.row_len + col;
            if row < self.row_len && col < self.col_len() && index < self.elts.len() {
                Some(&self.elts[index])
            } else {
                None
            }
        }
        pub fn set(&mut self, row: usize, col: usize, val: T)
        where
            T: Copy,
        {
            self.elts[row * self.row_len + col] = val;
        }

        /// Make a 2D array from its elements, given as concatenated rows.
        pub fn make(elts: Vec<T>, row_len: usize) -> Array<T> {
            Array { elts, row_len }
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
                curr_row: 0,
                curr_col: 0,
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
