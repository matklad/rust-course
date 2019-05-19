// nop!(); // не работает
macro_rules! nop {
    () => ()
}
nop!();

mod m {
    nop!(); // Работает!
}

macro_rules! nop { // Переопределили макрос!
    () => { 92 }
}
