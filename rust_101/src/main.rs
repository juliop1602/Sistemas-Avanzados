use std::ffi::c_float;

fn main() {
    let x = 5;
    //x = 10; esto no es valido

    let mut y = 10;
    y = 20;

    let x = 5;
    {
        let x = x+1; //Nueva variable
        println!("El valor de x es:{}", x);
    }    

    println!("El valor de x es:{}", x);

    let entero: i32 = 42;
    let flotante: f64 = 3.14;
    let booleano: bool = true;  
    let caracter: char = 'a';  

    let tupla: (i32, f64, char) = (42, 3.14, 'a');
    println!("Tupla forma 1:{:?}", tupla);
    println!("Tupla forma 2:({},{},{})", tupla.0, tupla.1, tupla.2);
    
    let array: [i32; 3] = [1, 2, 3];    

}
