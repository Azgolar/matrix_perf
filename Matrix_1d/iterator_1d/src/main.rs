use rand::random_range;
use core_affinity::{CoreId, get_core_ids, set_for_current};
use std::env::args;
mod test;

/*
    1D Vektor mit Iterator
*/
pub fn iterator_1d(a: &Vec<f64>, b: &Vec<f64>, c: &mut Vec<f64>, n: usize) {

    // über alle Zeilen in Stücken der Größen n iterieren, dabei gilt (zeilenindex, aktuelle zeile)
    for (i, a_zeile) in a.chunks(n).enumerate() {
        for j in 0..n {
            let mut summe: f64 = 0.0;
            for k in 0..n {
                summe = summe + a_zeile[k] * b[k * n + j];
            }
            c[i * n + j] = summe;
        }
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

    let größen: &[usize] = &[4, 8, 11, 16, 25, 32, 64, 91, 128, 256, 357, 512, 787, 1024]; 

    // Benchmark mit den einzelnen Matrixgrößen durchführen
    println!("Durchlauf:");
    for i in 0..30 {
        for &n in größen {
            // Matrizen intialisieren
            let a: Vec<f64> = zufallsmatrix_1d(n);
            let b: Vec<f64> = zufallsmatrix_1d(n);
            let mut c: Vec<f64> = vec![0.0; n * n];

            iterator_1d(&a, &b, &mut c, n);
        }
        println!("{}/30", i);
    }
}
