use rand::random_range;
use core_affinity::{CoreId, get_core_ids, set_for_current};
use std::env::args;
mod test;

/*
    split_at für sicheren Zugriff

    split_at(n) gibt ein Tupel aus Slices zurück:
    - Das erste Element ist ein Slice (Referenz) auf die Zeilen 0 bis n-1
    - Das zweite Element ist ein Slice (Referenz) ab Zeile n bis zur letzten Zeile der Matrix

    --> da die Matrizen immer nur n Zeilen haben, ist das zweite Slice überflüssig
    
*/
pub fn split_at_1d(mut a: &[f64], b: &[f64], mut c: &mut [f64], n: usize) {
    for _ in 0..n {
        // aufteilen der aktuellen Zeilen in a und c
        let (a_zeile, a_rest): (&[f64], &[f64]) = a.split_at(n);
        let (ergebnis, c_rest): (&mut [f64], &mut [f64]) = c.split_at_mut(n); 

        for j in 0..n {
            let mut summe: f64 = 0.0;
            for k in 0..n {
                // a_zeile[k] enthält das k-te Element in Zeile i
                // für b lohnt sich split_at nicht da wir spaltenweise und nicht zeilenweise zugreifen
                summe = summe + a_zeile[k] * b[k * n + j];
            }
            // Ersetzen des aktuellen Werts in Zeile in bei Position j
            ergebnis[j] = summe;
        }
        // Zeiger updaten für nächste Iteration
        a = a_rest;
        c = c_rest;
    }
}

/*
    Erstellt eine 1D Matrix mit Datentyp f64 mit Zufallswerten im Bereich [-1.0,1.0]
*/
pub fn zufallsmatrix_1d(n: usize) -> Vec<f64> {
    let mut matrix: Vec<f64> = vec![0.0; n * n];
    for i in 0..(n * n) {
            matrix[i] = random_range(-1.0..=1.0);
        }
    matrix
}

fn main() {
    let eingabe : Vec<String> = args().collect();

    // auf Korrektheit getestet werden
    if eingabe.len() > 1 {
        test::testen();
        return;
    }

    // alle logischen Kerne für Pinning
    let kerne: Vec<CoreId> = get_core_ids().unwrap();
    set_for_current(kerne[0]);

    let größen: &[usize] = &[4, 8, 11, 16, 25, 32, 64, 91, 128, 256, 357, 512, 667, 780, 888, 951, 1024, 1211, 1380, 1499, 1555, 1600];  

    // Benchmark mit den einzelnen Matrixgrößen durchführen
    for _ in 0..20 {
        for &n in größen {
            // Matrizen intialisieren
            let a: Vec<f64> = zufallsmatrix_1d(n);
            let b: Vec<f64> = zufallsmatrix_1d(n);
            let mut c: Vec<f64> = vec![0.0; n * n];

            split_at_1d(&a, &b, &mut c, n);
        }
    }
    println!("Benchmark fertig");
}
