fn my_insert(arr: &mut [i32], len: &mut usize, pos: usize, val: i32) {
    for i in (pos..*len).rev() {
        arr[i + 1] = arr[i];
    }
    arr[pos] = val;
    *len += 1;
}

fn my_delete(arr: &mut [i32], len: &mut usize, pos: usize) {
    *len -= 1;
    for i in pos..*len {
        arr[i] = arr[i + 1];
    }
}

fn my_append(arr: &mut [i32], len: &mut usize, val: i32) {
    arr[*len] = val;
    *len += 1;
}

fn my_prepend(arr: &mut [i32], len: &mut usize, val: i32) {
    my_insert(arr, len, 0, val);
}

fn my_pop_front(arr: &mut [i32], len: &mut usize) -> i32 {
    let ret = arr[0];
    my_delete(arr, len, 0);
    ret
}

fn my_pop_back(arr: &mut [i32], len: &mut usize) -> i32 {
    let ret = arr[*len - 1];
    *len -= 1;
    ret
}

fn print_array(arr: &[i32], len: usize) {
    for i in 0..len {
        print!("{} ", arr[i]);
    }
    println!();
}

fn main() {
    let mut arr = [0; 100];
    let mut len = 3;
    arr[0] = 10;
    arr[1] = 20;
    arr[2] = 30;

    my_insert(&mut arr, &mut len, 1, 40); // 10 40 20 30
    print_array(&arr, len);

    my_delete(&mut arr, &mut len, 2); // 10 40 30
    print_array(&arr, len);

    my_append(&mut arr, &mut len, 50); // 10 40 30 50
    print_array(&arr, len);

    my_prepend(&mut arr, &mut len, 60); // 60 10 40 30 50
    print_array(&arr, len);

    println!("{}", my_pop_front(&mut arr, &mut len)); // 60
    print_array(&arr, len); // 10 40 30 50

    println!("{}", my_pop_back(&mut arr, &mut len)); // 50
    print_array(&arr, len); // 10 40 30
}
