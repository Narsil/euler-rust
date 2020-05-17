// https://www.quora.com/How-would-you-explain-an-algorithm-that-generates-permutations-using-lexicographic-ordering#
// Macro because array seems better than vec here (small array).
macro_rules! permutations {
    ($n: expr, $name: ident, $itname: ident) => {
        permutations!($n, $name, $itname, u8);
    };
    ($n: expr, $name: ident, $itname: ident, $type: ident) => {
        struct $itname {
            elements: [$type; $n],
            started: bool,
        }

        impl Iterator for $itname {
            type Item = [$type; $n];

            fn next(&mut self) -> Option<[$type; $n]> {
                // println!("---");
                if !self.started {
                    self.started = true;
                    return Some(self.elements);
                }
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
                                self.elements.swap(x, y);
                                // 4. Reverse P[x+1 .. n].
                                // println!("elts {:?}", self.elements);
                                for i in x + 1..$n {
                                    let j = $n - 1 - (i - (x + 1));
                                    if j <= i {
                                        break;
                                    }
                                    // println!("SWAP ({}, {})", i, j);
                                    self.elements.swap(i, j);
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
        fn $name(elements: [$type; $n]) -> $itname {
            $itname {
                elements,
                started: false,
            }
        }
    };
}

// struct PermutationIterator<'a, T>
// where
//     T: PartialOrd,
// {
//     elements: &'a mut [T],
//     started: bool,
// }
//
// impl<'a, T> Iterator for PermutationIterator<'a, T>
// where
//     T: PartialOrd,
// {
//     type Item = Vec<T>;
//
//     fn next(&mut self) -> Option<Self::Item> {
//         // println!("---");
//         let n = self.elements.len();
//         if !self.started {
//             self.started = true;
//             return Some(self.elements.clone().to_vec());
//         }
//         // 1. Find Find the largest x such that P[x]<P[x+1].
//         // (If there is no such x, P is the last permutation.)
//         for i in 0..n - 1 {
//             let x = n - 2 - i;
//             // println!("{} cmp {}", self.elements[x], self.elements[x + 1]);
//             if self.elements[x] < self.elements[x + 1] {
//                 // println!("X = {} (val = {})", x, self.elements[x]);
//                 // 2. Find the largest y such that P[x]<P[y].
//                 for k in 0..n - x - 1 {
//                     let y = n - 1 - k;
//                     if self.elements[y] > self.elements[x] {
//                         // println!("Y = {} (val = {})", y, self.elements[y]);
//                         // 3. Swap P[x] and P[y].
//                         self.elements.swap(x, y);
//                         // 4. Reverse P[x+1 .. n].
//                         // println!("elts {:?}", self.elements);
//                         for i in x + 1..n {
//                             let j = n - 1 - (i - (x + 1));
//                             if j <= i {
//                                 break;
//                             }
//                             // println!("SWAP ({}, {})", i, j);
//                             self.elements.swap(i, j);
//                         }
//                         return Some(self.elements);
//                     }
//                 }
//                 panic!("We should never arrive here !");
//             }
//         }
//         None
//     }
// }
//
// fn permutations<'a, T>(elements: &'a mut [T]) -> PermutationIterator<'a, T>
// where
//     T: PartialOrd,
// {
//     PermutationIterator {
//         elements,
//         started: false,
//     }
// }
