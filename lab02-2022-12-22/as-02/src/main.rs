fn main() {
    let n = 5;
    let mut i = 1;
    let mut j = 1;
    let mut k = i;
    let mut count = 0;

    while i < n+1 {
        while j <= n-i {
            print!("  ");
            j += 1;
        }
        while count < k {
            print!("* ");
            count += 1;
        }
        count = 0;
        j = 1;
        k+= 2;
        i+= 1;
            println!("");
    }

}

