use std::{io::{self, Write}, result};
const N: usize= 100;

struct Cadena {
    longitud: usize,
    caracteres: [char; N], //Arreglo
}

impl Cadena {
    //Constructor
    fn new() -> Self {
        Cadena {
            longitud: 0,
            caracteres: ['\0'; N],
        }
    }

    fn obtener_longitud(&self) -> usize {
        self.longitud
    }

    //Metodo para adicionar caracteres
    fn add_char(&mut self, c:char) {
        if self.longitud < N {
            self.caracteres[self.longitud] = c;
            self.longitud += 1;
        }
    }

    //Metodo para devolver un caracter, dada la posicion.
    fn obtener_char(&self, pos: usize) -> char {
        if pos > 0 && pos <= self.longitud {
            self.caracteres[pos-1]
        } else {
            '\0'
        }
    }

    //Metodo para contar la cantidad de apariciones de un caracter.
     fn contar_apariciones(&self, c:char) -> usize {
        let mut contador: usize = 0;
        for i in 0..self.longitud {
            if self.caracteres[i] == c {
                contador += 1;
            }
        }
        contador
    }

    //Metodo que devuelva el caracter mas repetido.
     fn char_mas_repetido(&self) -> char {
        let mut max_char = self.caracteres[0];
        let mut max_cont = 0;

        for i in 0..self.longitud {
            let car = self.caracteres[i];
            let mut cont = 0;
            for j in 0..self.longitud {
                if self.caracteres[j] == car {
                    cont += 1;
                }
            }
            if cont > max_cont {
                max_cont = cont;
                max_char = car;
            }
        }
        max_char
    }

    //Metodo para invertir la cadena.
    fn invertir(&mut self) {
        if self.longitud <= 1 {
            return
        }
        let mut izq = 0;
        let mut der = self.longitud - 1;
        while izq < der {
            let temp = self.caracteres[izq];
            self.caracteres[izq] = self.caracteres[der];
            self.caracteres[der] = temp;
            izq += 1;
            der -= 1;
        }
    }

    //Metodo para contar vocales y consonantes
    fn contar_vocales_consonantes(&self) -> (usize, usize) {
        let mut vocal:usize = 0;
        let mut consonante: usize = 0;
        for i in 0..self.longitud {
            let car = self.caracteres[i];
            let letra = (car >= 'a' && car <= 'z') || (car >= 'A' && car <= 'Z');
            if letra {
                let esvocal = car == 'a' || car == 'e' || car == 'i' || car == 'o' || car == 'u';
                if esvocal {
                    vocal += 1;
                } else {
                    consonante += 1;
                }
            }
        }
        (vocal, consonante)
    }

    //Metodo para eliminar caracteres duplicados contiguos (consecutivos). Ej:
    //aaabbbccdfd = abcdfd
    fn eliminar_repetidos_consecutivos(&self) -> Cadena {
        let mut cad = Cadena::new();
        for i in 0..self.longitud {
            if i == 0 || self.caracteres[i] != self.caracteres[i-1] {
                cad.add_char(self.caracteres[i]);
            }
        }
        cad
    }

    //Metodo para eliminar un caracter de una posicion dada.
    fn eliminar_car(&mut self, p:usize) {
        if p > 0 && p <= self.longitud {
            let pos = p-1;
            for i in pos..self.longitud-1 {
                self.caracteres[i] = self.caracteres[i+1];
            }
            self.longitud = self.longitud-1;
        }
    }

    //Metodo para obtener una subcadena, indicando el inicio y fin:
    //Ej: Hola como va -> inicio = 3 fin = 9 => la como
    fn subcadena(&self, inicio:usize, fin:usize) -> Cadena {
        let mut subcad = Cadena::new();
        if inicio < 1 && fin > self.longitud && inicio > fin {
            return subcad;
        }
        for i in (inicio-1)..fin {
            subcad.add_char(self.caracteres[i]);
        }
        subcad
    }

    fn limpiar(&mut self) {
        self.longitud = 0;
        self.caracteres = ['\0'; N];
    }

    fn mostrar(&self) {
        for i in 0..self.longitud {
            print!("{}", self.caracteres[i]);
        }
        println!();
    }
    fn eliminar_match(&mut self, c: char){
        for i in (0..self.longitud).rev() {
            if self.caracteres[i] == c {
                self.eliminar_car(i+1);
            }
        }
    }
    fn ordenar_alafabeticamente(&mut self) {
        for i in 0..self.longitud {
            for j in i+1..self.longitud {// Comparamos los caracteres en las posiciones i y j
                if self.caracteres[i] > self.caracteres[j] {
                    let temp = self.caracteres[i];
                    self.caracteres[i] = self.caracteres[j];
                    self.caracteres[j] = temp;
                }
            }
        }
    }
        fn dejar_fecha_mayor(&mut self) {
        let fechas = self.dividir(',');
        if fechas.is_empty() { return; }

        let mut indice_mayor = 0;
        for i in 1..fechas.len() {
            if Self::comparar_fechas(&fechas[i], &fechas[indice_mayor]) == 1 {
                indice_mayor = i;
            }
        }

        // Modificamos el propio objeto reemplazando su contenido por el de la fecha mayor
        self.limpiar();
        for i in 0..fechas[indice_mayor].longitud {
            self.add_char(fechas[indice_mayor].caracteres[i]);
        }
    }
        fn dejar_fecha_menor(&mut self) {
        let fechas = self.dividir(',');
        if fechas.is_empty() { return; }

        let mut indice_menor = 0;
        for i in 1..fechas.len() {
            if Self::comparar_fechas(&fechas[i], &fechas[indice_menor]) == -1 {
                indice_menor = i;
            }
        }

        // Modificamos el propio objeto
        self.limpiar();
        for i in 0..fechas[indice_menor].longitud {
            self.add_char(fechas[indice_menor].caracteres[i]);
        }
    }
        fn remover_hexadecimal_mayor(&mut self) {
        let hex_list = self.dividir(',');
        if hex_list.is_empty() { return; }

        let mut indice_mayor = 0;
        let mut valor_mayor = hex_list[0].hex_a_decimal();

        for i in 1..hex_list.len() {
            let valor_actual = hex_list[i].hex_a_decimal();
            if valor_actual > valor_mayor {
                valor_mayor = valor_actual;
                indice_mayor = i;
            }
        }

        // Limpiamos el objeto actual y lo reconstruimos sin el elemento mayor
        self.limpiar();
        let mut primero = true;
        for i in 0..hex_list.len() {
            if i == indice_mayor { continue; }
            
            if !primero { self.add_char(','); }
            primero = false;

            for j in 0..hex_list[i].longitud {
                self.add_char(hex_list[i].caracteres[j]);
            }
        }
    }
    fn eliminar_palabras_vocal_repetida(&mut self) {
        let palabras = self.dividir(' ');       
        //Usamos una instancia limpia para armar el resultado
        self.limpiar();
        let mut primero = true;
        for palabra in palabras {
            //palbras es vector dinamico de tipo cadena y palabra hace referencia a 
            // los elementos dentro de ese vector
            let mut tiene_repetida = false;
            // Verificamos si alguna vocal se repite dentro de ESTA palabra
            for i in 0..palabra.longitud {
                //recorremos letra por letra
                let c1 = palabra.caracteres[i];
                let es_vocal = c1 == 'a' || c1 == 'e' || c1 == 'i' || c1 == 'o' || c1 == 'u' ||
                               c1 == 'A' || c1 == 'E' || c1 == 'I' || c1 == 'O' || c1 == 'U';
                
                if es_vocal {
                    // Comparamos contra el resto de caracteres de la misma palabra
                    //tecnica de dos dedos
                    for j in i + 1..palabra.longitud {
                        // Comparamos ignorando mayúsculas/minúsculas
                        if palabra.caracteres[j].to_ascii_lowercase() == c1.to_ascii_lowercase() {
                            tiene_repetida = true; //encontramos una vocal repetida
                            break;//break del for j o segundo dedo
                        }
                    }
                }
                if tiene_repetida { break; } //break del for i o primer dedo
            }

            //Si la palabra NO tiene ninguna vocal repetida, la conservamos
            // ! convertimo el si en un si no es decir ejecuta si es falso
            if !tiene_repetida {
                if !primero {//si no es la primera palabra agregamos un espacio de separacion
                    self.add_char(' '); // Añade el espacio de separación
                }
                primero = false;

                for i in 0..palabra.longitud {
                    self.add_char(palabra.caracteres[i]); //reconstructor de la cadena original
                }
            }
        }
    }
    fn eliminar_por_parentesis(&mut self){
        for i in 0.. self.longitud{
            if self.caracteres[i] == '('{
                for j in i..self.longitud{
                    self.eliminar_car(i+1);
                    if self.caracteres[i] == ')'{
                        self.eliminar_car(i+1);
                        break;//rompemos el ciclo for j
                    }
                }
            }
        }
    }
    fn mostrar_por_parametro(&mut self, c:char)-> Cadena{
        let mut palabras = self.dividir(' ');
        let mut primero = true;
        let mut resultado = Cadena::new();
        for palabra in palabras{
            let mut contiene = false;
            for i in 0..palabra.longitud{
                if palabra.caracteres[i].to_ascii_lowercase() == c.to_ascii_lowercase(){
                    contiene = true;
                    break;
                }
            }
            if contiene{
                if !primero{
                    resultado.add_char(' ');
                }
                primero = false;
                for i in 0..palabra.longitud{
                    resultado.add_char(palabra.caracteres[i]);
                }
            }
        }        
        resultado
    }
    fn eliminar_si_p_f_consonantes(&mut self){
        let mut palabras = self.dividir(' ');
        self.limpiar();
        let mut primero = true;
        for palabra in palabras{
            for i in 0..palabra.longitud{
                let primer_car = palabra.caracteres[0].to_ascii_lowercase();
                let ultimo_car = palabra.caracteres[palabra.longitud-1].to_ascii_lowercase();
                let es_consonante = |c: char| {
                    let c = c.to_ascii_lowercase();
                    (c >= 'a' && c <= 'z') && (c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u')
                };
                if es_consonante(primer_car) || es_consonante(ultimo_car){
                    if !primero{
                        self.add_char(' ');
                    }
                    primero = false;
                    for j in 0..palabra.longitud{
                        self.add_char(palabra.caracteres[j]);
                    }
                    break;//rompemos el ciclo for i para pasar a la siguiente palabra
                }
            }
        }
    }
    fn pal_rep(&self, palabra_a_buscar: &Cadena) -> usize {
      let palabras = self.dividir(' ');
     let mut contador = 0;

      for j in 0..palabras.len() {
         let palabra_lista = &palabras[j];//&hace que se pase la referencia de la palabra en lugar de copiarla
        // Comparación
         if palabra_a_buscar.longitud == palabra_lista.longitud {
                let mut son_iguales = true;
                for k in 0..palabra_a_buscar.longitud {
                   if palabra_a_buscar.caracteres[k].to_ascii_lowercase() != palabra_lista.caracteres[k].to_ascii_lowercase() {
                        son_iguales = false;
                       break;
                  }
             }
             if son_iguales {
                   contador += 1;
               }
           }
      }
    contador // Devolvemos el número entero acumulado
    }

    fn devolver_pal_rep(&self) -> Cadena{
        let mut resultado = Cadena::new();
        let mut palabras = self.dividir(' ');
        if  palabras.is_empty() { return resultado; }
        let mut index=0;
        let mut gan=0;
        for i in 0..palabras.len() {
            let conteo:usize = self.pal_rep(&palabras[i]);
            if conteo > gan {
                gan = conteo;
                index = i;
            }
        }
        if gan > 1 {
            let ganadora = &palabras[index];
            for k in 0..ganadora.longitud {
                resultado.add_char(ganadora.caracteres[k]);
            }
        }
        resultado
    }
    fn devolver_may_longitud(&self) -> Cadena {
        let mut resultado = Cadena::new();
        let palabras = self.dividir(' ');
        if palabras.is_empty() { return resultado; }

        let mut max_longitud = 0;
        for palabra in &palabras {//&palabras hace que se pase la referencia de la palabra en lugar de copiarla
            if palabra.longitud > max_longitud {
                max_longitud = palabra.longitud;
            }
        }

        for palabra in palabras {//aqui si copiamos la palabra porque necesitamos construir el resultado
            if palabra.longitud == max_longitud {
                if resultado.longitud > 0 {
                    resultado.add_char(' ');
                }
                for i in 0..palabra.longitud {
                    resultado.add_char(palabra.caracteres[i]);
                }
            }
        }
        resultado
    }
    fn eliminar_palabras_con_subcadena(&mut self, subcadena_eliminar: &Cadena) {
        // Separamos nuestra cadena original en palabras usando tu método dividir
        let palabras = self.dividir(' ');
        // Limpiamos el objeto actual para reconstruirlo desde cero sin las palabras prohibidas
        self.limpiar();
        let mut primero = true;
        // Evaluamos cada palabra del vector una por una
        for i in 0..palabras.len() {
            let palabra_actual = &palabras[i];
            // Usamos nuestra función auxiliar para verificar si contiene la subcadena
            if !palabra_actual.contiene_subcadena(subcadena_eliminar) {
                // Si NO la contiene, significa que queremos CONSERVAR esta palabra      
                // Si no es la primera palabra que añadimos, ponemos un espacio de separación
                if !primero {
                    self.add_char(' ');
                }
                primero = false;
                // Copiamos los caracteres de la palabra conservada al objeto principal
                for j in 0..palabra_actual.longitud {
                    self.add_char(palabra_actual.caracteres[j]);
                }
            }
            // Si la palabra SÍ contenía la subcadena, simplemente el bucle la ignora y pasa a la siguiente
        }
    }
    fn contar_palabras_con_subcadena(&self, subcadena_buscar: &Cadena) -> usize {
        let palabras = self.dividir(' ');
        let mut contador = 0;

        for palabra in palabras {
            if palabra.contiene_subcadena(subcadena_buscar) {
                contador += 1;
            }
        }
        contador
    }
    // Ej: "cadena" pasando "cad" -> pasa a ser "ena"
    fn eliminar_solo_subcadena(&mut self, subcadena_a_borrar: &Cadena) {
        if subcadena_a_borrar.longitud == 0 || self.longitud == 0 {
            return;
        }
        let mut resultado = Cadena::new();
        let mut i = 1; // Usamos base 1 de acuerdo a tu método subcadena
        while i <= self.longitud {
            // Verificamos si a partir de la posición 'i' comienza la subcadena
            if i + subcadena_a_borrar.longitud - 1 <= self.longitud {
                let final_fragmento = i + subcadena_a_borrar.longitud - 1;
                let fragmento = self.subcadena(i, final_fragmento);
                // Comparamos el fragmento del texto con la subcadena
                let mut son_iguales = true;
                for j in 0..subcadena_a_borrar.longitud {
                    if fragmento.caracteres[j].to_ascii_lowercase() != subcadena_a_borrar.caracteres[j].to_ascii_lowercase() {
                        son_iguales = false;
                        break;
                    }
                }
                // SI SON IGUALES: Encontramos la subcadena. 
                // "Saltamos" sus caracteres sumando su longitud a nuestro índice, sin copiarlos.
                if son_iguales {
                    i += subcadena_a_borrar.longitud;
                    continue;
                }
            }
            // SI NO SON IGUALES: El carácter actual no es parte de la subcadena oculta.
            // Lo conservamos copiándolo al resultado.
            resultado.add_char(self.caracteres[i - 1]);
            i += 1;
        }
        // Reemplazamos el estado de nuestro objeto con el resultado limpio
        self.limpiar();
        for i in 0..resultado.longitud {
            self.add_char(resultado.caracteres[i]);
        }
    }
    fn ordenar_leng_men(&mut self) {
        let mut palabras = self.dividir(' ');
        // Ordenamos las palabras por longitud usando un algoritmo de burbuja
        for i in 0..palabras.len() {
            for j in i + 1..palabras.len() {
                if palabras[i].longitud > palabras[j].longitud {
                    palabras.swap(i, j);
                }
            }
        }
        // Reconstruimos la cadena original con las palabras ordenadas
        self.limpiar();
        let mut primero = true;
        for palabra in palabras {
            if !primero {
                self.add_char(' ');
            }
            primero = false;
            for i in 0..palabra.longitud {
                self.add_char(palabra.caracteres[i]);
            }
        }
    }
    fn eliminar_si_conso_rep_2(&mut self){
        let mut palabras = self.dividir(' ');
        self.limpiar();
        let mut cont=0;
        let mut primero = true;
        'evaluamos:for palabra in palabras{
            for i in 0..palabra.longitud{
                for j in i+1..palabra.longitud{
                    let primer_car = palabra.caracteres[i].to_ascii_lowercase();
                    let segundo_car = palabra.caracteres[j].to_ascii_lowercase();
                    let es_consonante = |c: char| {
                        let c = c.to_ascii_lowercase();
                        (c >= 'a' && c <= 'z') && !(c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u')
                    };
                    if es_consonante(primer_car) && primer_car == segundo_car{
                        cont+=1;
                        if cont== 2{
                            continue 'evaluamos;
                        }
                        // Si encontramos una consonante repetida, omitimos esta palabra y pasamos a la siguiente
                        // continue 'evaluamos;
                    }
                }  
            }
            if !primero{
                self.add_char(' ');
            }
            primero=false;
            for j in 0..palabra.longitud{
                self.add_char(palabra.caracteres[j]);
            }
        }
    }
        // --- Ejercicio 26: Devolver el nombre del producto de menor precio ---
    fn devolver_menor_precio(&self) -> Cadena {
        let mut resultado = Cadena::new();
        
        // 1. Separamos los bloques por comas (Ej: ["Manzana=10", " Pera=20", ...])
        let bloques = self.dividir(',');
        if bloques.is_empty() {
            return resultado;
        }

        let mut indice_menor = 0;
        let mut precio_menor = i32::MAX; // Inicializamos con el número más alto posible

        // 2. Analizamos cada bloque uno por uno
        for i in 0..bloques.len() {
            // Dividimos el bloque actual por el signo '=' (Ej: ["Manzana", "10"])
            let partes = bloques[i].dividir('=');
            
            // Verificamos que el bloque tenga el nombre y el precio válidos
            if partes.len() == 2 {
                // El precio está en la segunda posición (índice 1). Lo pasamos a entero de forma manual.
                let precio_actual = partes[1].para_entero();

                // Buscamos cuál es el menor valor numérico
                if precio_actual < precio_menor {
                    precio_menor = precio_actual;
                    indice_menor = i; // Guardamos la posición del bloque ganador
                }
            }
        }

        // 3. Extraemos el nombre del producto ganador (está antes del '=', índice 0)
        let bloques_ganador = bloques[indice_menor].dividir('=');
        if !bloques_ganador.is_empty() {
            let nombre_producto = &bloques_ganador[0];
            
            // Copiamos los caracteres quitando posibles espacios molestos al inicio o final
            let mut inicio_car = 0;
            while inicio_car < nombre_producto.longitud && nombre_producto.caracteres[inicio_car] == ' ' {
                inicio_car += 1;
            }

            for j in inicio_car..nombre_producto.longitud {
                resultado.add_char(nombre_producto.caracteres[j]);
            }
        }

        resultado
    }



    //==================================
    //AUXULIARES
    //==================================
        // Auxiliar: Devuelve true si la 'subcadena_buscar' está dentro de esta palabra
    fn contiene_subcadena(&self, subcadena_buscar: &Cadena) -> bool {
        // Si la subcadena es más larga que la palabra, es imposible que esté dentro
        if subcadena_buscar.longitud > self.longitud {
            return false;
        }

        // Si la subcadena está vacía, técnicamente siempre está contenida
        if subcadena_buscar.longitud == 0 {
            return true;
        }

        // Usamos una ventana deslizante. Si la palabra mide 7 y la subcadena 3,
        // revisamos los rangos: 1..3, 2..4, 3..5, 4..6 y 5..7.
        let limite = self.longitud - subcadena_buscar.longitud + 1;

        for i in 1..=limite {
            // Extraemos un fragmento de la palabra del mismo tamaño que la subcadena
            let final_fragmento = i + subcadena_buscar.longitud - 1;
            let fragmento = self.subcadena(i, final_fragmento);

            // Comparamos el fragmento contra la subcadena letra por letra
            let mut son_iguales = true;
            for j in 0..subcadena_buscar.longitud {
                if fragmento.caracteres[j].to_ascii_lowercase() != subcadena_buscar.caracteres[j].to_ascii_lowercase() {
                    son_iguales = false;
                    break; // Si una letra no coincide, rompemos este bucle interno
                }
            }

            // Si encontramos una coincidencia perfecta, la palabra sí contiene la subcadena
            if son_iguales {
                return true;
            }
        }

        // Si recorrimos toda la palabra y ningún fragmento coincidió
        false
    }

    //divide la cadena es subcadenas usanod la coma 
    fn dividir(&self, separador: char) -> Vec<Cadena> {
        let mut resultado = Vec::new();
        let mut token = Cadena::new();

        for i in 0..self.longitud {
            if self.caracteres[i] == separador {
                resultado.push(token);
                token = Cadena::new();
            } else {
                token.add_char(self.caracteres[i]);
            }
        }
        resultado.push(token);
        resultado
    }
    // Auxiliar: Convierte una Cadena numérica (ej: "2019") a entero base 10
    fn para_entero(&self) -> i32 {
        let mut num = 0;
        for i in 0..self.longitud {
            let c = self.caracteres[i];
            if c >= '0' && c <= '9' {
                num = num * 10 + (c as i32 - '0' as i32);
            }
        }
        num
    }
    // Auxiliar: Convierte una Cadena hexadecimal a entero base 16 de forma manual (ej: "FA9")
    fn hex_a_decimal(&self) -> u32 {
        let mut suma = 0;
        for i in 0..self.longitud {
            let c = self.caracteres[i];
            let valor_digito = match c.to_ascii_uppercase() {
                '0'..='9' => c as u32 - '0' as u32,
                'A'..='F' => c as u32 - 'A' as u32 + 10,
                _ => 0,
            };
            suma = (suma * 16) + valor_digito;
        }
        suma
    }
    // Auxiliar: Compara dos Cadenas con formato de fecha DD/MM/AAAA usando subcadenas fijas
    fn comparar_fechas(f1: &Cadena, f2: &Cadena) -> i32 {
        let d1 = f1.subcadena(1, 2).para_entero();
        let m1 = f1.subcadena(4, 5).para_entero();
        let a1 = f1.subcadena(7, 10).para_entero();

        let d2 = f2.subcadena(1, 2).para_entero();
        let m2 = f2.subcadena(4, 5).para_entero();
        let a2 = f2.subcadena(7, 10).para_entero();

        if a1 != a2 { return if a1 > a2 { 1 } else { -1 }; }
        if m1 != m2 { return if m1 > m2 { 1 } else { -1 }; }
        if d1 != d2 { return if d1 > d2 { 1 } else { -1 }; }
        0
    }


    
}

fn leer_linea() -> String {
    let mut entrada = String::new();
    io::stdin().read_line(&mut entrada).expect("Error al leer");
    entrada.trim().to_string()
}

fn leer_numero() -> Option<usize> {
    leer_linea().parse::<usize>().ok()
}

fn mostrar_menu(c: &Cadena) {
    // construimos la cadena actual para mostrarla en el encabezado
    let mut preview = String::new();
    for i in 0..c.longitud {
        preview.push(c.caracteres[i]);
    }
    if preview.is_empty() {
        preview = String::from("(vacía)");
    }

    println!("\n╔══════════════════════════════════╗");
    println!("║   CADENA: {:>22}  ║", preview);
    println!("╠══════════════════════════════════╣");
    println!("║  1. Ingresar nueva cadena        ║");
    println!("║  2. Mostrar cadena               ║");
    println!("║  3. Longitud                     ║");
    println!("║  4. Obtener carácter (posición)  ║");
    println!("║  5. Cantidad repeticiones (char) ║");
    println!("║  6. Invertir cadena              ║");
    println!("║  7. Nros. Vocales y Consonantes  ║");
    println!("║  8. Eliminar caracter (pos)      ║");
    println!("║  9. Subcadena                    ║");
    println!("║  10. Eliminar todos los matches  ║");
    println!("║  11. Dejar fecha mayor           ║");
    println!("║  12. Dejar fecha menor           ║");
    println!("║  13. Remover hexadecimal mayor   ║");
    println!("║  14. Eliminar palabra vocal repe ║");
    println!("║  15. Eliminar por parentesis     ║");
    println!("║  16. Mostrar por parámetro       ║");
    println!("║  17. Eliminar si la p&u es conso ║");
    println!("║  18. Mostrar por pal repe        ║");
    println!("║  19. Mostrar palabra de may long ║");
    println!("║  20. Eliminar palabras con subcad║");
    println!("║  21. Contar palabras con subcad  ║");
    println!("║  22. Ordenar palabras por longitu║");
    println!("║  23. elim si   rep mas de 1 conso║");
    println!("║  24. Devolver producto menor prec║");
    println!("║  Q. Salir                        ║");
    println!("║                                  ║");
    println!("╚══════════════════════════════════╝");
    print!("   Opción: ");
    io::stdout().flush().expect("Error al mostrar menú");
}

fn main() {
    println!("════════════════════════════════════");
    println!("  Cadenas - POO — Programación I   ");
    println!("════════════════════════════════════");

    let mut c = Cadena::new(); //Creando la instancia de clase

    loop {
        mostrar_menu(&c);
        let opcion = leer_linea();

        match opcion.as_str() {
            "1" => {
                println!("  Ingresa la cadena:");
                let entrada ="Manzana=10, Pera=20, Naranja=15, uva=17"; //leer_linea1();

                c.limpiar(); // reiniciamos antes de cargar la nueva

                // ── proceso artesanal: carácter por carácter ──
                for ch in entrada.chars() {
                    c.add_char(ch);
                }

                println!("  ✓ Cadena cargada ({} caracteres)", c.obtener_longitud());
            }

            "2" => {
                print!("  Cadena: ");
                c.mostrar();
            }

            "3" => println!("  Longitud: → {}", c.obtener_longitud()),

            "4" => {
                println!("  Ingresa la posición (1 = izquierda):");
                match leer_numero() {
                    Some(pos) if pos >= 1 && pos <= c.obtener_longitud() => {
                        println!("  Carácter en posición {}: → '{}'", pos, c.obtener_char(pos));
                    }
                    Some(_) => println!("  Posición fuera de rango (1 a {}).", c.obtener_longitud()),
                    None    => println!("  Posición inválida."),
                }
            }

            "5" => {
                println!("  Ingresa el caracter:");
                let entrada = leer_linea();
                match entrada.chars().next() {
                    Some(car)  => {
                        let cantidad = c.contar_apariciones(car);
                        println!("  El caracter aparecer: {} vez/veces", cantidad);
                    }
                    None    => println!("  No ingresaste ningun caracter choquito."),
                }
            }

            "6" => {
                if c.obtener_longitud() == 0 {
                    println!("La cadena esta vacia");
                } else {
                    c.invertir();
                    println!("La cadena invertida es: ");
                    c.mostrar();
                }
            }

            "7" => {
                if c.obtener_longitud() == 0 {
                    println!("La cadena esta vacia");
                } else {
                    let (vocal, consonante) = c.contar_vocales_consonantes();
                    println!("El nro de vocales, es: {}", vocal);
                    println!("El nro de consonantes es: {}", consonante);
                }
            }

            "8" => {
                if c.obtener_longitud() == 0 {
                    println!("La cadena esta vacia");
                } else {
                    println!("Ingresa la posicion a eliminar: ");
                    match leer_numero() {
                        Some(pos) if pos > 0 && pos <= c.obtener_longitud() => {
                            c.eliminar_car(pos);
                            println!("Resultado de la nueva cadena: ");
                            c.mostrar();
                        }Some(_) => println!("Posicion fuera de rango"),
                        None => println!("Invalido"),
                    }
                }
            }

            "9" => {
                if c.obtener_longitud() == 0 {
                    println!("La cadena esta vacia");
                } else {
                    println!("Ingresa la posicion de inicio: ");
                    match leer_numero() {
                        Some(inicio) if inicio > 0 && inicio <= c.obtener_longitud() => {
                            println!("Ingres la posicion del fin: ");
                            match leer_numero() {
                                Some(fin) if fin >= inicio && fin <= c.obtener_longitud() => {
                                    let subca = c.subcadena(inicio, fin);
                                    println!("La Subcadena entre las posiciones: [{}-{}] es: ", inicio, fin);
                                    subca.mostrar();
                                    let long = subca.longitud;
                                    println!("La longitud de la nueva cadena es: {}", long);
                            }
                            Some(_) => println!("Fin invalido"),
                            None => println!("Posicion invalida"),
                        }
                    }
                    Some(_) => println!("Inicio fuera de rango"),
                        None => println!("Posicion Invalida"),
                    }
                }
            }
            "10" => {
                if c.obtener_longitud() == 0 {
                    println!("La cadena esta vacia");
                } else {
                    println!("Ingresa el caracter a eliminar: ");
                    let entrada = leer_linea();
                    match entrada.chars().next(){
                        Some(car)  => {
                            c.eliminar_match(car);
                            println!("Resultado de la nueva cadena: ");
                            c.mostrar();
                        }
                        None    => {
                            c.eliminar_match(' ');
                            println!("  No ingresaste ningun caracter choquito.");
                            c.mostrar();
                        }
                    }
                }
            }
            "11" => {
                if c.obtener_longitud() == 0 {
                    println!("La cadena esta vacia");
                } else {
                    c.dejar_fecha_mayor();
                    println!("Resultado de la nueva cadena: ");
                    c.mostrar();
                }
            }
            "12" => {
                if c.obtener_longitud() == 0 {
                    println!("La cadena esta vacia");
                } else {
                    c.dejar_fecha_menor();
                    println!("Resultado de la nueva cadena: ");
                    c.mostrar();
                }
            }
            "13" => {
                if c.obtener_longitud() == 0 {
                    println!("La cadena esta vacia");
                } else {
                    c.remover_hexadecimal_mayor();
                    println!("Resultado de la nueva cadena: ");
                    c.mostrar();
                }
            }
            "14" => {
                if c.obtener_longitud() == 0 {
                    println!("La cadena esta vacia");
                } else {
                    c.eliminar_palabras_vocal_repetida();
                    println!("Resultado de la nueva cadena: ");
                    c.mostrar();
                }
            }
            "15" => {
                if c.obtener_longitud() == 0 {
                    println!("La cadena esta vacia");
                } else {
                    c.eliminar_por_parentesis();
                    println!("Resultado de la nueva cadena: ");
                    c.mostrar();
                }
            }
            "16" => {
                if c.obtener_longitud() == 0 {
                    println!("La cadena esta vacia");
                } else {
                    println!("Ingresa el caracter a buscar: ");
                    let entrada = leer_linea();
                    match entrada.chars().next(){
                        Some(car)  => {
                            let resultado = c.mostrar_por_parametro(car);
                            println!("Resultado de la nueva cadena: ");
                            resultado.mostrar();
                        }
                        None    => {
                            println!("  No ingresaste ningun caracter choquito.");
                        }
                    }
                }
            }
            "17" => {
                if c.obtener_longitud() == 0 {
                    println!("La cadena esta vacia");
                } else {
                    c.eliminar_si_p_f_consonantes();
                    println!("Resultado de la nueva cadena: ");
                    c.mostrar();
                }
            }
            "18" => { 
                if c.obtener_longitud() == 0 {
                    println!("La cadena esta vacia");
                } else {
                    let resultado = c.devolver_pal_rep();
                    println!("Resultado de la nueva cadena: ");
                    resultado.mostrar();
                }
            }
            "19" => { 
                if c.obtener_longitud() == 0 {
                    println!("La cadena esta vacia");
                } else {
                    let resultado = c.devolver_may_longitud();
                    println!("Resultado de la nueva cadena: ");
                    resultado.mostrar();
                }
            }
            "20" => {
                if c.obtener_longitud() == 0 {
                    println!("La cadena esta vacia");
                } else {
                    println!("Ingresa la subcadena a eliminar: ");
                    let entrada = leer_linea();
                    let mut subcadena_eliminar = Cadena::new();
                    for ch in entrada.chars() {
                        subcadena_eliminar.add_char(ch);
                    }
                    c.eliminar_palabras_con_subcadena(&subcadena_eliminar);
                    println!("Resultado de la nueva cadena: ");
                    c.mostrar();
                }
            }
            "21" => {
                if c.obtener_longitud() == 0 {
                    println!("La cadena esta vacia");
                } else {
                    println!("Ingresa la subcadena a buscar: ");
                    let entrada = leer_linea();
                    let mut subcadena_buscar = Cadena::new();
                    for ch in entrada.chars() {
                        subcadena_buscar.add_char(ch);
                    }
                    let cantidad = c.contar_palabras_con_subcadena(&subcadena_buscar);
                    println!("Cantidad de palabras que contienen la subcadena: {}", cantidad);
                }
            }
            "22" => {
                if c.obtener_longitud() == 0 {
                    println!("La cadena esta vacia");
                } else {
                    println!("la cadena ordenada por longitud de menor a mayor es: ");
                    c.ordenar_leng_men();
                    c.mostrar();
                }
            }
            "23" => {
                if c.obtener_longitud() == 0 {
                    println!("La cadena esta vacia");
                } else {
                    c.eliminar_si_conso_rep_2();
                    println!("Resultado de la nueva cadena: ");
                    c.mostrar();
                }
            }
            "24" => {
                if c.obtener_longitud() == 0 {
                    println!("La cadena esta vacia");
                } else {
                    let resultado = c.devolver_menor_precio();
                    println!("Resultado de la nueva cadena: ");
                    resultado.mostrar();
                }
            }
            "q" | "Q" => { println!("\n  Hasta luego.\n"); break; }
            _          => println!("  Opción no válida."),
        }
    }
}
