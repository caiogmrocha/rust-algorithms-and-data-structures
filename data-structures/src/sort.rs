pub fn insertion_sort(array: &mut [i32]) {
    let mut out = vec![-1; array.len()];

    for i in 0..array.len() {
        let mut j = 0;

        while array[i] >= out[j] && out[j] != -1 {
            j += 1;
        }

        for k in (j+1..array.len()).rev() {
            out[k] = out[k-1];
        }

        out[j] = array[i];
    }

    array.clone_from_slice(&out)
}

pub fn bubble_sort(array: &mut [i32]) {
    let mut is_sorted = true;

    'first_for: for i in 0..array.len() {
        for j in 0..array.len() {
            if array[i] != array[j] && array[i] < array[j] {
                array.swap(i, j);
                is_sorted = false;
            }
        }

        if is_sorted == true {
            println!("here");

            break 'first_for;
        }
    }
}