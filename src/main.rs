//para el control de flujo existen principalmente 2 estructuras: if(else) y loop
fn main() {

    //if evalua una condicion booleana y ejecuta el codigo solo si es verdadera, para el caso contrario se puede usar else

    let x = 6;

    if x>5 {
        println!("{x} es mayor a 5");
    }else {
        println!("{x} es menor o igual a 5");
    }

    //aunque if espera un expresion, esta debe de tener como resolucion un valorr booleano
    //en Rust no se relaiza un cast de valores no booleanos a booleanos de forma automatica
    //por lo que indicar una expresion no booleana en una expresion if lanza un panic de mismatch type

    //if else
    //la expresion if else nos permite manejar una logica de primer brazo
    //evaluando hasta que se cumple la expresion, los if else posteriores no se evaluan

    let y = 0;

    if y == 0{
        println!("{x} es 0");
    } else if y<5 {
        println!("{x} es menor a 5");
    } else if y==5 {
        println!("{x} es igual a 5");
    } else if y>5 {
        println!("{x} es mayor a 5");
    }

    //como la construccion if es una expresion y ofrece un scope puede ser asignado a una variable
    //es importante recordar que la unica condicion es que estos bloques tengan una expresion de retorno y no un statement
    //es la razon de que la expresion dentro del scope no tenga punto y coma.
    let condition = true;
    let z = if condition {1} else {0};
    println!("el valor de z es {z}");

    //en caso de que los valores devueltos en las expresiones de if y else sean distintos
    //se tendra un panic debido a que no se tiene certeza del tipo que se asignara, ya que se realiza
    //en tiempo de ejecucion y el compilador no tiene certeza.

    //realizar esa inferencia, haria al compilador mas complejo y ofreceria menos garantias de seguridad.

}
