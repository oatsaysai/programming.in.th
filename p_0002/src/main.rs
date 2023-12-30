use std::io;

fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).unwrap();
    let n = n.trim().parse::<i32>().unwrap();

    let mut arr = vec![Default::default(); n as usize];

    let mut i = 0;
    while i < n {
        let mut m = String::new();
        io::stdin().read_line(&mut m).unwrap();
        let m = m.trim().parse::<i32>().unwrap();

        arr[i as usize] = m;

        i = i + 1;
    }

    i = 1;
    let mut min_val = arr[0];
    let mut max_val = arr[1];

    while i < n {
        if arr[i as usize] < min_val {
            min_val = arr[i as usize]
        }
        if arr[i as usize] > max_val {
            max_val = arr[i as usize]
        }

        i = i + 1;
    }

    println!("{}", min_val);
    println!("{}", max_val);
}
