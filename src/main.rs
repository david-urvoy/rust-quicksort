fn main() {
    let mut arr = [9,5,-3,10,20,42,-23,0,45,8,-7,3];
    let size = arr.len();
    println!("Functional quicksort: {:?}", functional_quicksort(&arr));
    println!("Lomuto quicksort: {:?}", quicksort(&mut arr, 0, size-1));
}

/** Lomuto quicksort */

fn quicksort(arr: &mut [i32], lo: usize, hi: usize) -> &[i32] {
    if lo < hi {
        let p = partition(arr, lo, hi);
        quicksort(arr, lo, p-1);
        quicksort(arr, p+1, hi);
    }
    arr
}

fn partition(arr: &mut [i32], lo: usize, hi: usize) -> usize {
    let last = arr[hi];
    let mut i = lo;

    for j in lo..hi {
        if arr[j] < last {
            arr.swap(i, j);
            i += 1;
        }
    }
    arr.swap(i, hi);
    i
}


/** Purely functional implementation */

fn functional_quicksort(arr: &[i32]) -> Vec<i32> {
    match arr {
        &[] => vec!(),
        arr => {
            let (head, tail) = arr.split_at(1);
            let mut sorted = smaller_sorted(&head[0], tail.to_vec());
            sorted.append(&mut head.to_vec());
            sorted.append(&mut bigger_sorted(&head[0], tail.to_vec()));
            sorted
        }
    }
}

fn smaller_sorted(head: &i32, tail: Vec<i32>) -> Vec<i32> {
    functional_quicksort(&tail.into_iter().filter(|x| x <= head).collect::<Vec<i32>>())
}

fn bigger_sorted(head: &i32, tail: Vec<i32>) -> Vec<i32> {
    functional_quicksort(&tail.into_iter().filter(|x| x > head).collect::<Vec<i32>>())
}

