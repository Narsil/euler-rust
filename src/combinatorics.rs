// https://www.quora.com/How-would-you-explain-an-algorithm-that-generates-permutations-using-lexicographic-ordering#
// Macro because array seems better than vec here (small array).
macro_rules! permutations {
    ($n: expr, $name: ident, $itname: ident) => {
        struct $itname {
            elements: [u8; $n],
            started: bool,
        }

        impl Iterator for $itname {
            type Item = [u8; $n];

            fn next(&mut self) -> Option<[u8; $n]> {
                // println!("---");
                if !self.started {
                    self.started = true;
                    return Some(self.elements);
                }
                // println!("elements {:?}", self.elements);
                #[inline]
                fn swap(i: usize, j: usize, array: &mut [u8; $n]) {
                    let tmp = array[i];
                    array[i] = array[j];
                    array[j] = tmp;
                };

                // 1. Find Find the largest x such that P[x]<P[x+1].
                // (If there is no such x, P is the last permutation.)
                for i in 0..$n - 1 {
                    let x = $n - 2 - i;
                    // println!("{} cmp {}", self.elements[x], self.elements[x + 1]);
                    if self.elements[x] < self.elements[x + 1] {
                        // println!("X = {} (val = {})", x, self.elements[x]);
                        // 2. Find the largest y such that P[x]<P[y].
                        for k in 0..$n - x - 1 {
                            let y = $n - 1 - k;
                            if self.elements[y] > self.elements[x] {
                                // println!("Y = {} (val = {})", y, self.elements[y]);
                                // 3. Swap P[x] and P[y].
                                swap(x, y, &mut self.elements);
                                // 4. Reverse P[x+1 .. n].
                                // println!("elts {:?}", self.elements);
                                for i in x + 1..$n {
                                    let j = $n - 1 - (i - (x + 1));
                                    if j <= i {
                                        break;
                                    }
                                    // println!("SWAP ({}, {})", i, j);
                                    swap(i, j, &mut self.elements);
                                }
                                return Some(self.elements);
                            }
                        }
                        panic!("We should never arrive here !");
                    }
                }
                return None;
            }
        }
        fn $name(elements: [u8; $n]) -> $itname {
            $itname {
                elements,
                started: false,
            }
        }
    };
}
