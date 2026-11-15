use std::backtrace;

const MAX: usize = 100;

struct Vectores {
    //Atributos
    dimension: usize,
    elemento: [u64;MAX],
}

impl Vectores {
    //Constructor
    fn new() -> Vectores {
        Vectores {
            dimension: 0,
            elemento: [0;MAX],
        }
    }

    fn dim(&self) -> usize {
        self.dimension
    }

    fn dimensionar(&mut self, d:usize) {
        self.dimension = d;
    }

    fn addelement(&mut self, e:u64) {
        if self.dimension < MAX {
            self.elemento[self.dimension] = e;
            self.dimension += 1;
        }
    }

    fn show(&self) {
        for i in 0..self.dimension { //Va hasta el nro - 1
            println!("Elemento[{}] = {}", i, self.elemento[i]);
        }
    }

    fn reemplazar(&mut self, p:usize, e:u64){
        if p <= self.dimension {
            self.elemento[p-1] = e;
        }
    }

    fn insertar(&mut self, p:usize, e:u64){
        if (self.dimension < MAX) && (p <= self.dimension) {
            self.dimension += 1;
            let mut x = self.dimension;
            while x > p-1 {
                self.elemento[x-1] = self.elemento[x-2];
                x -= 1;
            }
            self.elemento[p-1] = e;
        }
    }
    fn pares_impares(&self) -> bool{
        let mut pares = 0;
        let mut impares = 0;
        for i in 0..self.dimension {
            if self.elemento[i] % 2 == 0 {
                pares += 1;
            } else {
                impares += 1;
            }
        }
        pares > impares
    }
    fn tiene_escala(&self) -> bool{
        let n = self.dimension;
        for i in 0..n-2 {
            let a = self.elemento[i];
            let b = self.elemento[i+1];
            let c = self.elemento[i+2];
            if b == a + 1 && c == b+1 {
                return true;
            }
        }
        false

    }
    //=======================================================
    // TAREAS
    //=======================================================
    //=======================================================
    // 1.- ORDENAR IMPARES Y PARES DE FORMA ASCENDENTE
    //=======================================================
    fn ordenar_impares_pares(&mut self) {
        if self.dimension == 0 { return; }
        // Paso 1: Mover todos los impares al inicio y pares al final
        let mut i = 0;
        let mut j = self.dimension - 1;
        while i < j {
            while i < self.dimension && self.elemento[i] % 2 != 0 {
                i += 1;
            }
            while j > 0 && self.elemento[j] % 2 == 0 {
                j -= 1;
            }
            if i < j {
                self.elemento.swap(i, j);
            }
        }

        // Determinar cuántos impares hay para saber el límite divisorio
        let mut cant_impares = 0;
        for k in 0..self.dimension {
            if self.elemento[k] % 2 != 0 {
                cant_impares += 1;
            }
        }

        // Ordenar la sección de impares (de 0 hasta cant_impares)
        if cant_impares > 1 {
            for k in 0..cant_impares {
                for m in 0..cant_impares - 1 - k {
                    if self.elemento[m] > self.elemento[m + 1] {
                        self.elemento.swap(m, m + 1);
                    }
                }
            }
        }

        // Ordenar la sección de pares (desde cant_impares hasta el final)
        if self.dimension > cant_impares {
            for k in cant_impares..self.dimension {
                for m in cant_impares..self.dimension - 1 - (k - cant_impares) {
                    if self.elemento[m] > self.elemento[m + 1] {
                        self.elemento.swap(m, m + 1);
                    }
                }
            }
        }
    }

    //=======================================================
    // 2.- ORDENAR DE ADENTRO HACIA AFUERA
    //=======================================================
    fn ordenar_adentro_afuera(&mut self) {
        let n = self.dimension;
        if n <= 1 { return; }

        // Ordenar todo de menor a mayor
        for i in 0..n {
            for j in 0..n - 1 - i {
                if self.elemento[j] > self.elemento[j + 1] {
                    self.elemento.swap(j, j + 1);
                }
            }
        }
        //La banderilla
        let mut salida = [0u64; MAX]; // Arreglo temporal        
        let c = n / 2;             // Centro inicial
        let mut pasos_izq = 0;     // Pasos a la izquierda
        let mut pasos_der = 0;     // Pasos a la derecha
        let mut izquierda = true;  // La banderilla

        for i in 0..n {
            if i == 0 {
                // El primer elemento al centro
                salida[c] = self.elemento[i];
            } else if izquierda {
                // Siguiente elemento va a la izquierda
                pasos_izq += 1;
                salida[c - pasos_izq] = self.elemento[i];
                izquierda = false; // Volteamos la banderilla
            } else {
                // Siguiente elemento va a la derecha
                pasos_der += 1;
                salida[c + pasos_der] = self.elemento[i];
                izquierda = true;  // Volteamos la banderilla
            }
        }
        for i in 0..n {
            self.elemento[i] = salida[i];
        }
    }
}

fn main() {
    //Crear instancia
    let mut v = Vectores::new();
    v.addelement(1);
    v.addelement(6);
    v.addelement(5);
    v.addelement(2);
    v.addelement(0);
    v.addelement(4);
    v.addelement(3);
//    v.addelement(3);

    println!("----------------------");
    v.show();
    println!("----------------------");
    println!("La cantidad de elementos es: {}", v.dim());

    //v.insertar(7, 5);
    println!("----------------------");
    v.show();
    println!("----------------------");
    println!("La cantidad de elementos es: {}", v.dim());
    if v.pares_impares() {
        println!("Hay más números pares.");
    } else {
        println!("Hay más números impares.");
    }   
    if v.tiene_escala() {
        println!("El vector tiene una escala de 3 elementos consecutivos.");
    } else {
        println!("El vector NO tiene una escala de 3 elementos consecutivos.");
    } 
    println!("----------------------");
    //v.ordenar_impares_pares();
    //v.show();
    println!("----------------------");  
    println!("----------------------"); 
    v.ordenar_adentro_afuera();
    v.show();   
    println!("----------------------");
    println!("----------------------");
}