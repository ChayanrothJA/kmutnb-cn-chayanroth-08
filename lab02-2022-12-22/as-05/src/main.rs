fn main() {
    let mut n = 10; //keyboard
    let mut i = 0;
    let mut j = 0;
   
   
    
    while i < n {
      
        while j+1 <= n{
            if j == 0 ||  j == n-1 || j == i {
                print!("X ");
                j+=1;
            }  
  
        else{
             print!("O ");
              j+=1;
            }
          
            

        }
       
      
        j = 0;  
        i += 1;
        println!("");//เว้นบรรทัด
    } 
}

