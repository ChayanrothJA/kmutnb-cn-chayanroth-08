fn main() {
    let n = 5;
    let mut i = 0;
    let mut j = 0;
    while i < n {  //ตัวรันเเถว
        while j < i+1{  //ตัวรันหลัก
            print!("* ");
            j += 1;   
        }
        j = 0;  
        i += 1;
        println!(""); //เว้นบรรทัด
    }
   
}





