const NF: usize = 120;//fila max
const NC: usize = 120;//colum max

struct Matriz {
    nro_filas: usize,
    nro_columnas: usize,
    celdas: [[u64; NC]; NF],//la matiz en cuestion
}

impl Matriz {
    fn new(nro_filas: usize, nro_columnas: usize) -> Matriz {   //constructor
        Matriz {
            nro_filas,
            nro_columnas,
            celdas: [[0; NC]; NF],
        }
    }

    fn obt_nro_filas(&self) -> usize {
        self.nro_filas
    }

    fn obt_nro_columnas(&self) -> usize {
        self.nro_columnas
    }

    fn get_celda(&self, f: usize, c: usize) -> u64 {        //que valor es en?
        self.celdas[f][c]
    }

    fn set_celda(&mut self, f: usize, c: usize, valor: u64) {       //pon valor en..
        self.celdas[f][c] = valor;
    }

    fn suma(&self) -> u64 {
        let mut suma = 0;

        for f in 0..self.nro_filas {            //de aqui 
            for c in 0..self.nro_columnas {     //hasta aqui, recorre toda la matriz
                suma += self.celdas[f][c];              // += es lo mismo que suma = suma + self.celdas[f][c]
            }
        }

        suma                                            // devuelve la suma total de la matriz -> u64
    }

    fn pares(&self) -> u64 {                            
        let mut cont = 0;                       //contador

        for f in 0..self.nro_filas {
            for c in 0..self.nro_columnas {       
                if self.celdas[f][c] % 2 == 0 {         //si el valor de la celda es par, == es comparar % significa dividir pero me da el resto
                    cont += 1;
                }
            }
        }

        cont
    }

    fn promedio(&self) -> u64 {
        self.suma() / (self.nro_filas * self.nro_columnas) as u64  // llamo la funcion con self.lafuncion, / significa dividir pero me da el cociente, as significa convertir el tipo de dato a u64 por que la matriz es u 64
    }

    fn mayor(&self) -> u64 {
        let mut mayor = self.celdas[0][0]; //inicializo el mayor con el primer valor de la matriz

        for f in 0..self.nro_filas {
            for c in 0..self.nro_columnas {
                if self.celdas[f][c] > mayor {
                    mayor = self.celdas[f][c];   //si el valor de la celda es mayor que el mayor actual, actualizo el mayor
                }
            }
        }

        mayor
    }

    fn suma_fila(&self, f: usize) -> u64 {        //f:usize es un valor de fila que da el usuario
        let mut suma = 0;                   //mochila

        for c in 0..self.nro_columnas {
            suma += self.celdas[f][c];
        }

        suma
    }

    fn suma_col(&self, c: usize) -> u64 {       //c:usize es un valor de columna que da el usuario
        let mut suma = 0;

        for f in 0..self.nro_filas {
            suma += self.celdas[f][c];
        }

        suma
    }

    fn busq_lineal(&self, n: u64) -> (usize, usize) { //n:u64 es un elemento que da el usuario para buscar en la matriz, ->(usize, usize) es la ubicacion del elemento ¨hallado¨
        for f in 0..self.nro_filas {
            for c in 0..self.nro_columnas {
                if self.celdas[f][c] == n {
                    return (f, c);                     //return termina la ejecucion al encontra el elemento 
                }
            }
        }

        (50, 50)                                       //si no se encuentra el elemento, devuelve (50, 50) como un valor que indica que no se encontro
    }

    fn contar_apariciones(&self, n: u64) -> u64 {
        let mut cont = 0;                     // contador

        for f in 0..self.nro_filas {
            for c in 0..self.nro_columnas {
                if self.celdas[f][c] == n {
                    cont += 1;
                }
            }
        }

        cont
    }
    fn contar_menores(&self, n: u64) -> u64 {
        let mut cont = 0;                     // contador

        for f in 0..self.nro_filas {
            for c in 0..self.nro_columnas {
                if self.celdas[f][c] < n {
                    cont += 1;
                }
            }
        }

        cont
    }
    fn suma_dig_principal(&self) -> u64 {
        let mut suma = 0;

        for i in 0..self.nro_filas.min(self.nro_columnas) { //min es para evitar salirnos de los limites de la matriz
            suma += self.celdas[i][i]; //la diagonal principal tiene la misma fila y columna
        }

        suma
    }
    fn suma_dig_secundaria(&self) -> u64 {
        let mut suma = 0;

        for i in 0..self.nro_filas.min(self.nro_columnas) {
            suma += self.celdas[i][self.nro_columnas - 1 - i]; //la diagonal secundaria tiene la columna inversa a la fila, la magia es col-1-i si fuese inversa de fila seria fil-1-i
        }

        suma
    }

    fn ordenamiento_lineal(&mut self) {
        for f in 0..self.nro_filas {
            for c in 0..self.nro_columnas { //mano 1 apunta a ubicacion actual
                for ff in 0..self.nro_filas {
                    for cc in 0..self.nro_columnas { //mano 2 apunta a ubicacion de comparacion
                        if self.celdas[ff][cc] < self.celdas[f][c] {      //si el valor de la celda de comparacion es menor que el valor de la celda actual
                            let cambio = self.celdas[ff][cc];         //mano 2 mueve el elemento al bolsillo
                            self.celdas[ff][cc] = self.celdas[f][c];        //mano 1 mueve el elemento a donde estaba mano 2
                            self.celdas[f][c] = cambio;                      //mano 2 saca el elemnto del bolsillo y lo pone donde estaba mano 1 antes de moverse
                        }
                    }
                }
            }
        }
    }
    fn ordenamiento_linealv1(&mut self, asc: bool) {
    for f in 0..self.nro_filas {
        for c in 0..self.nro_columnas {
            for ff in 0..self.nro_filas {
                for cc in 0..self.nro_columnas {

                    let condicion = if asc {
                        self.celdas[ff][cc] > self.celdas[f][c]
                    } else {
                        self.celdas[ff][cc] < self.celdas[f][c]
                    };

                    if condicion {
                        let temp = self.celdas[ff][cc];
                        self.celdas[ff][cc] = self.celdas[f][c];
                        self.celdas[f][c] = temp;
                    }
                }
            }
        }
    }
}

    fn busqueda_bin(&self, n: u64) -> (usize, usize) {
        for f in 0..self.nro_filas {
            let mut izq = 0;
            let mut der = self.nro_columnas - 1;

            while izq <= der {
                let medio: usize = (izq + der) / 2;

                if self.celdas[f][medio] == n {
                    return (f, medio);
                }

                if self.celdas[f][medio] < n {
                    izq = medio + 1;             //mueve el limite izq a medio + 1
                } else {
                    if medio == 0 {             //evita caer en numero negativo 
                        break;
                    }

                    der = medio - 1;            //mueve el limite der a medio - 1
                }
            }
        }

        (50, 50)
    }
    fn may_dig_principal(&self) -> u64 {
        let mut mayor = self.celdas[0][0];

        for i in 0..self.nro_filas.min(self.nro_columnas) {
            if self.celdas[i][i] > mayor {
                mayor = self.celdas[i][i];
            }
        }

        mayor
    }
    fn intercambiar_filas(&mut self, f1: usize, f2: usize) {
        for c in 0..self.nro_columnas {
            let temp = self.celdas[f1][c];      //vector temporal de almacenamiento 
            self.celdas[f1][c] = self.celdas[f2][c];   //fila 1 toma el valor de fila 2
            self.celdas[f2][c] = temp;      //fila 2 toma el valor de fila 1 que estaba en el vector temporal
        }
    }
    fn es_triangular_superior(&self) -> bool {
        for f in 1..self.nro_filas { //empieza desde la fila 1 porque la fila 0 no tiene elementos debajo de la diagonal si fuese inferio nrofilas-1
            for c in 0..f { //recorre las columnas hasta la diagonal (excluyendo la diagonal)
                if self.celdas[f][c] != 0 { //si encuentra un elemento debajo de la diagonal que no es cero, no es triangular superior
                    return false;
                }
            }
        }

        true
    }
    fn may_dig_secundaria(&self) -> u64 {
        let mut mayor = self.celdas[0][self.nro_columnas - 1];

        for i in 0..self.nro_filas.min(self.nro_columnas) {
            if self.celdas[i][self.nro_columnas - 1 - i] > mayor {
                mayor = self.celdas[i][self.nro_columnas - 1 - i];
            }
        }

        mayor
    }
    fn es_triangular_inferior(&self) -> bool {
        for f in 0..self.nro_filas { //recorre todas las filas
            for c in f + 1..self.nro_columnas { //recorre las columnas desde la diagonal (excluyendo la diagonal)
                if self.celdas[f][c] != 0 { //si encuentra un elemento encima de la diagonal que no es cero, no es triangular inferior
                    return false;
                }
            }
        }

        true
    }
    fn es_diagonal(&self) -> bool {
        for f in 0..self.nro_filas {
            for c in 0..self.nro_columnas {
                if f != c && self.celdas[f][c] != 0 { //si encuentra un elemento que no esta en la diagonal y no es cero, no es diagonal
                    return false;
                }
            }
        }

        true
    }
    fn es_identidad(&self) -> bool {
        for f in 0..self.nro_filas {
            for c in 0..self.nro_columnas {
                if (f == c && self.celdas[f][c] != 1) || (f != c && self.celdas[f][c] != 0) { //si encuentra un elemento en la diagonal que no es 1 o un elemento fuera de la diagonal que no es 0, no es identidad
                    return false;
                }
            }
        }

        true
    }
    fn es_nula(&self) -> bool {
        for f in 0..self.nro_filas {
            for c in 0..self.nro_columnas {
                if self.celdas[f][c] != 0 { //si encuentra un elemento que no es cero, no es nula
                    return false;
                }
            }
        }

        true
    }
    fn es_simetrica(&self) -> bool {
        if self.nro_filas != self.nro_columnas { //si no es cuadrada, no puede ser simetrica
            return false;
        }

        for f in 0..self.nro_filas {
            for c in 0..self.nro_columnas {
                if self.celdas[f][c] != self.celdas[c][f] { //si encuentra un elemento que no es igual a su transpuesto, no es simetrica
                    return false;
                }
            }
        }

        true
    }
    fn es_antisimetrica(&self) -> bool {
        if self.nro_filas != self.nro_columnas { //si no es cuadrada, no puede ser antisimetrica
            return false;
        }

        for f in 0..self.nro_filas {
            for c in 0..self.nro_columnas {
                if self.celdas[f][c] != 0 && self.celdas[f][c] == self.celdas[c][f] { //si encuentra un elemento que no es cero y es igual a su transpuesto, no es antisimetrica
                    return false;
                }
            }
        }

        true
    }
    fn es_ortogonal(&self) -> bool {
        if self.nro_filas != self.nro_columnas { //si no es cuadrada, no puede ser ortogonal
            return false;
        }

        for f in 0..self.nro_filas {
            for c in 0..self.nro_columnas {
                if self.celdas[f][c] != 0 && self.celdas[f][c] != 1 { //si encuentra un elemento que no es cero ni uno, no es ortogonal
                    return false;
                }
            }
        }

        true
    }
    fn es_cuadrada(&self) -> bool {
        self.nro_filas == self.nro_columnas //si el numero de filas es igual al numero de columnas, es cuadrada
    }
    fn mover_ceros_final_de_fila(&mut self) {
        for f in 0..self.nro_filas {
            let mut pos = 0; //posicion para colocar el siguiente numero no cero

            for c in 0..self.nro_columnas {
                if self.celdas[f][c] != 0 { //si el elemento no es cero, lo movemos a la posicion indicada por pos
                    self.celdas[f][pos] = self.celdas[f][c];
                    if pos != c { //si pos es diferente a c, significa que hubo un movimiento, entonces ponemos un cero en la posicion original
                        self.celdas[f][c] = 0;
                    }
                    pos += 1; //actualizamos la posicion para el siguiente numero no cero
                }
            }
        }
    }
    fn eliminar_fila(&mut self, f: usize) {
        for c in 0..self.nro_columnas {
            self.celdas[f][c] = 0; //pone cero en toda la fila indicada por f
        }
    }
    fn eliminar_columna(&mut self, c: usize) {
        for f in 0..self.nro_filas {
            self.celdas[f][c] = 0; //pone cero en toda la columna indicada por c
        }
    }
    fn eliminar_fila_columna(&mut self, f: usize, c: usize) {
        self.eliminar_fila(f); //elimina la fila f
        self.eliminar_columna(c); //elimina la columna c
    }
    fn eliminar_menores(&mut self, n: u64) {
        for f in 0..self.nro_filas {
            for c in 0..self.nro_columnas {
                if self.celdas[f][c] < n { //si el elemento es menor que n, lo eliminamos
                    self.celdas[f][c] = 0;
                }
            }
        }
    }
    fn eliminar_mayores(&mut self, n: u64) {
        for f in 0..self.nro_filas {
            for c in 0..self.nro_columnas {
                if self.celdas[f][c] > n { //si el elemento es mayor que n, lo eliminamos
                    self.celdas[f][c] = 0;
                }
            }
        }
    }
    fn eliminar_apariciones(&mut self, n: u64) {
        for f in 0..self.nro_filas {
            for c in 0..self.nro_columnas {
                if self.celdas[f][c] == n { //si el elemento es igual a n, lo eliminamos
                    self.celdas[f][c] = 0;
                }
            }
        }
    }
    fn may_al_promedio_de_la_matriz(&self) -> u64 {
        let promedio = self.promedio(); //calcula el promedio de la matriz
        let mut mayor = 0;

        for f in 0..self.nro_filas {
            for c in 0..self.nro_columnas {
                if self.celdas[f][c] > promedio && self.celdas[f][c] > mayor { //si el elemento es mayor que el promedio y mayor que el mayor actual, actualizamos el mayor
                    mayor = self.celdas[f][c];
                }
            }
        }

        mayor
    }
    //fn mostrar_central(&self) {
    //    for f in 0..self.nro_filas{
    //        for c in 0..self.nro_columnas {
    //            if f > 0  && f < self.nro_filas -1 && c>0 && c<self.nro_columnas -1 { 
    //                print!("{:^9}", self.celdas[f][c]);
    //            }
    //        }
    //    }
    //}
    fn suma_recorre(&self) -> u64 {
        let mut suma = 0;

        self.recorre(|_, _, v| {
            suma += v; //suma el valor de cada celda a la variable suma
        });

        suma
    }
    fn numeros_mayores_a(&self, n: u64) -> Vec<u64> {
        let mut mayores = Vec::new(); //vector para almacenar los numeros mayores a n

        self.recorre(|_, _, v| {
            if v > n { //si el valor de la celda es mayor que n, lo agregamos al vector
                mayores.push(v);
            }
        });

        mayores
    }
    fn recorre<F>(&self, mut func: F)
    where
        F: FnMut(usize, usize, u64),                        //evita bucles ahora solo paso ubicacion y valor
    {
        for fila in 0..self.nro_filas {
            for columna in 0..self.nro_columnas {
                func(fila, columna, self.celdas[fila][columna]);
            }
        }
    }
    fn mostrar_sinborde(&self) -> Matriz{
        if self.nro_filas < 3 || self.nro_columnas < 3 { //si la matriz es demasiado pequeña para eliminar el borde, devolvemos una matriz vacia
            return Matriz::new(0, 0);
        }
        let mut nueva= Matriz::new(self.nro_filas - 2, self.nro_columnas - 2);
        for f in 1..self.nro_filas - 1 {
            for c in 1..self.nro_columnas - 1 {
                nueva.set_celda(f-1, c-1, self.celdas[f][c]);
            }
        }
        nueva
    }
    fn mostrar_sinbordeconrecorrer(&self) -> Matriz {
        let mut nueva = Matriz::new(self.nro_filas - 2, self.nro_columnas - 2);//-1 + -1por cadafila y/o columna quitada

        self.recorre(|f, c, v| {
            if f > 0 && f < self.nro_filas - 1 && c > 0 && c < self.nro_columnas - 1 { //0 fila inicial, -1 fila final, 0 columna inicial, -1 columna final
                nueva.set_celda(f - 1, c - 1, v);           //f y c deben concidir con la ubicacion inicial en este caso es 0 pero quiero eliminar borde asi que es -1
            }
        });
        nueva
    }

    fn mostrar(&self) {
        println!();

        for f in 0..self.nro_filas {
            print!("  ");
            for _ in 0..self.nro_columnas {     //_ variable descartable
                print!("┌─────────┐");
            }
            println!();

            print!("  ");
            for c in 0..self.nro_columnas {
                print!("│{:^9}│", self.celdas[f][c]); //{:^9} es para centrar el numero en un espacio de 9 caracteres, el ^ es para centrar, el 9 es el ancho del campo
            }
            println!();

            print!("  ");
            for _ in 0..self.nro_columnas {
                print!("└─────────┘");
            }
            println!();
        }
    }
}

fn main() {
    let mut m = Matriz::new(3, 4);

    m.set_celda(0, 0, 1);
    m.set_celda(0, 1, 5);
    m.set_celda(0, 2, 3);
    m.set_celda(0, 3, 19);

    m.set_celda(1, 0, 4);
    m.set_celda(1, 1, 53);
    m.set_celda(1, 2, 6);
    m.set_celda(1, 3, 20);

    m.set_celda(2, 0, 7);
    m.set_celda(2, 1, 8);
    m.set_celda(2, 2, 9);
    m.set_celda(2, 3, 21);

    m.mostrar();

    println!("Suma = {}", m.suma());
    println!("Pares = {}", m.pares());
    println!("Promedio = {}", m.promedio());
    println!("Mayor = {}", m.mayor());
    println!("Apariciones de 5 = {}", m.contar_apariciones(5));
    let (f, c) = m.busqueda_bin(8);
    println!("8 encontrado en ({}, {})", f, c);
    println!("Mayor al promedio = {}", m.may_al_promedio_de_la_matriz());
    println!( "Central = ");
    println!("numeros mayores a 10 = {:?}", m.numeros_mayores_a(10)); //{:?} es para imprimir un vector
  //  m.ordenamiento_linealv1(true);//true  o false para orden ascendente o descendente
//m.mostrar();
    m.mostrar_sinborde().mostrar();
    m.mostrar_sinbordeconrecorrer().mostrar();
}