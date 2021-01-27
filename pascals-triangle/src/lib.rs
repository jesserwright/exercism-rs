pub struct PascalsTriangle {
    rows: u32,
}

// If `new()` stores the row count and `rows()` lazily evaluates, that is probably better (?)
// Actually, ideally, it would be cached, so lots of calls to rows() would be a saved computation :)

impl PascalsTriangle {
    pub fn new(rows: u32) -> Self {
        Self { rows }
    }

    pub fn rows1(&self) -> Vec<Vec<u32>> {
        if self.rows == 0 {
            return vec![];
        }

        let mut rows: Vec<Vec<u32>> = Vec::with_capacity(self.rows as usize);

        for i in 1..=self.rows {
            match i {
                1 => {
                    rows.push(vec![1]);
                }
                2 => rows.push(vec![1, 1]),
                _ => {
                    let mut new_vec: Vec<u32> = vec![1];
                    let prev_vec = rows.last().unwrap();

                    for j in 0..i - 2 {
                        let j = j as usize;
                        new_vec.push(prev_vec[j] + prev_vec[j + 1]);
                    }

                    new_vec.push(1);
                    rows.push(new_vec);
                }
            }
        }
        rows
    }

    // https://exercism.io/tracks/rust/exercises/pascals-triangle/solutions/c72010ed92784ff1a9c238044430fd20
    pub fn rows2(&self) -> Vec<Vec<u32>> {
        if self.rows == 0 {
            return vec![];
        }

        // With capacity will hold n elements without reallocating
        let mut rows: Vec<Vec<u32>> = Vec::with_capacity(self.rows as usize);
        // there is always at least 1, if row count != 0
        rows.push(vec![1]);

        // if row_count is 2, this will run once with n as 2, because the end is inclusive
        for n in 2..=self.rows {
            let mut row = Vec::with_capacity(n as usize);
            row.push(1);
            let previous = rows.last().unwrap(); // this gets the previous row.
            let previous = previous.as_slice();

            // If the slice is smaller than the given number, it will not return.
            // this means that when n = 2, this for loop is never run! Which is perfect,
            // becaus we only get 2 calls to `row.push(1)`, resulting in [1,1]

            // Windows will create overlap. [1,2,1] will generate two windows: [1,2] and [2,1]
            for pair in previous.windows(2) {
                row.push(pair[0] + pair[1]); // add the pair
            }
            //
            row.push(1);
            rows.push(row);
        }

        rows
    }

    // How does mutability happen here?
    // Fold can build up a result from a collection. JS reduce does this too.
    // https://exercism.io/tracks/rust/exercises/pascals-triangle/solutions/5717008c0a83444fad113bfdf14108b6
    pub fn rows(&self) -> Vec<Vec<u32>> {
        // fold is good for turning a collection into some value or values.
        // the result value could be, for example, a vector!
        // we will start with a vector as our starting element and add vectors as we go.
        // the aggregate can be accessed, so the next rows can build their values from previous rows
        (0..self.rows).fold(
            Vec::with_capacity(self.rows as usize), // The accumulate element. The capacity will be the size of rows it will hold
            |mut agg, idx| // This is a function that will mutate. `FnMut`
            // agg is owned and mutable
             {
                // These two calls are 'imperative'
                // this is never constructed if the range is 0..0
                let mut v = Vec::with_capacity(idx as usize + 1);
                v.push(1);

                if let Some(prev_row) = agg.last() {
                    for pair in prev_row.windows(2) {
                        v.push(pair[0] + pair[1])
                    }
                    v.push(1);
                }
                agg.push(v);
                println!("{:?}", &agg);
                agg
            },
        )
    }
}
