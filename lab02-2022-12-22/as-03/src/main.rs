fn main() {
    let n = 5;
    let mut i = 1;
    let mut j = n;
    let mut k = i;
    let mut l = n;
    let mut m = n+5;
    let mut count = 1;
    while i < n+1 {
        while j < l {
            print!(" ");
            j += 1;

        }
        while count < m {
            print!("* ");
            count += 1;
        }
        count = 1;
        i+= 1;
        j = n;
        k+= 2;
        l+= 2;
        m-= 2;
        
            println!("");
    }

}
