use std::fmt::Debug;

mod b_rand;
// O(n^2)
pub fn bubble_sort<T: PartialOrd + Debug>(v: &mut [T]) {
    for p in 0..v.len() {
        let mut sorted = true;
        for i in 0..(v.len() - 1 - p) {
            if v[i] > v[i + 1] {
                v.swap(i, i + 1);
                sorted = false;
            }
        }
        println!("{:?}", v);
        if sorted {
            return;
        }
    }
}

pub fn merge_sort<T: PartialOrd + Debug>(mut v: Vec<T>) -> Vec<T> {
    // Sort the left half,  O(n * ln(n))
    // Sort the right half, O(n * ln(n))
    // bring the sorted halfs together O(n)

    println!("MS:{:?}", v);
    if v.len() <= 1 {
        return v;
    }

    let mut res: Vec<T> = Vec::with_capacity(v.len());
    let b = v.split_off(v.len() / 2);
    let a = merge_sort(v);
    let b = merge_sort(b);

    // bring them together
    let mut a_it = a.into_iter();
    let mut b_it = b.into_iter();
    let mut a_peek = a_it.next();
    let mut b_peek = b_it.next();

    loop {
        match a_peek {
            Some(ref a_val) => match b_peek {
                Some(ref b_val) => {
                    if b_val < a_val {
                        res.push(b_peek.take().unwrap());
                        b_peek = b_it.next();
                    } else {
                        res.push(a_peek.take().unwrap());
                        a_peek = a_it.next();
                    }
                }
                None => {
                    res.push(a_peek.take().unwrap());
                    res.extend(a_it);
                    return res;
                }
            },
            None => {
                if let Some(b_val) = b_peek {
                    res.push(b_val);
                }
                res.extend(b_it);
                return res;
            }
        }
    }
}

// Move first element to the correct place
// Everything lower should be before it
// everything higher should be after it
// return it's location
pub fn pivot<T: PartialOrd + Debug>(v: &mut [T]) -> usize {
    // let mut p = 0;

    let mut p = b_rand::rand(v.len());
    v.swap(p, 0);

    p = 0;
    for i in 1..v.len() {
        // println!("PV: i = {} p = {} v = {:?}", i, p, v);
        if v[i] < v[p] {
            // move our pivot forward 1, and put this element before it
            v.swap(p + 1, i);
            v.swap(p, p + 1);
            p += 1
        }
    }
    // println!("PV: p = {}, v = {:?}", p, v);
    p
}

pub fn quick_sort<T: PartialOrd + Debug>(v: &mut [T]) {
    if v.len() <= 1 {
        return;
    }
    let p = pivot(v);
    println!("{:?}", v);

    let (a, b) = v.split_at_mut(p);
    quick_sort(a);
    quick_sort(&mut b[1..]);
}

pub fn quick_sort_rayon<T: PartialOrd + Debug + Send>(v: &mut [T]) {
    if v.len() <= 1 {
        return;
    }
    let p = pivot(v);
    println!("{:?}", v);

    let (a, b) = v.split_at_mut(p);

    // put f2 on queue then start f1;
    // if another thread is ready it will steal f2
    // this works recursively down the stack
    rayon::join(|| quick_sort_rayon(a), || quick_sort_rayon(&mut b[1..]));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buble_sort() {
        let mut v = vec![4, 6, 1, 8, 11, 13, 3];
        bubble_sort(&mut v);
        // assert_eq!(v, vec![1, 3, 4, 6, 8, 11, 13])
        assert_eq!(v, vec![1, 3, 4, 6, 8, 11, 13])
    }

    #[test]
    fn test_merge_sort() {
        let v = vec![4, 6, 1, 8, 11, 13, 3];
        let v = merge_sort(v);
        assert_eq!(v, vec![1, 3, 4, 6, 8, 11, 13])
        // assert_eq!(v, vec![3, 4, 6, 8, 11, 13]);
    }

    #[test]
    fn test_pivot() {
        let mut v = vec![4, 6, 1, 19, 8, 11, 13, 3];
        let p = pivot(&mut v);

        for x in 0..v.len() {
            assert!((v[x] < v[p]) == (x < p));
        }

        // assert_eq!(v, vec![1, 3, 4, 6, 19, 8, 11, 13]);
        // panic!();
    }

    #[test]
    fn test_quick_sort() {
        let mut v = vec![4, 6, 1, 8, 11, 13, 3];
        quick_sort(&mut v);
        assert_eq!(v, vec![1, 3, 4, 6, 8, 11, 13]); // Expected

        let mut v = vec![1, 2, 6, 7, 9, 12, 13, 14];
        quick_sort(&mut v);
        assert_eq!(v, vec![1, 2, 6, 7, 9, 12, 13, 14]);
        // panic!();
    }

    #[test]
    fn test_quick_sort_rayon() {
        let mut v = vec![4, 6, 1, 8, 11, 13, 3];
        quick_sort_rayon(&mut v);
        assert_eq!(v, vec![1, 3, 4, 6, 8, 11, 13]); // Expected

        let mut v = vec![1, 2, 6, 7, 9, 12, 13, 14];
        quick_sort_rayon(&mut v);
        assert_eq!(v, vec![1, 2, 6, 7, 9, 12, 13, 14]);
        panic!();
    }
}
