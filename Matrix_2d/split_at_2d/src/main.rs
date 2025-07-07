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
pub fn split_at_2d(a: &Vec<Vec<f64>>, b: &Vec<Vec<f64>>, c: &mut Vec<Vec<f64>>, n: usize) {
    let (b_geteilt, _): (&[Vec<f64>], &[Vec<f64>]) = b.split_at(n);

    for i in 0..n {
        // aufteilen der aktuellen Zeilen in a und c
        let (a_zeile, _): (&[f64], &[f64]) = a[i].split_at(n);
        let (c_zeile, _): (&mut [f64], &mut [f64]) = c[i].split_at_mut(n);

        for j in 0..n {
            let mut summe = 0.0;
            for k in 0..n {
                // a_zeile[k] enthält das k-te Element in Zeile i
                summe = summe + a_zeile[k] * b_geteilt[k][j];
            }
            // Ersetzen des aktuellen Werts in Zeile in bei Position j
            c_zeile[j] = summe;
        }
    }
}

/*
    Erstellt eine 2D Matrix mit Zufallswerten im Bereich [-1.0,1.0]
*/
pub fn zufallsmatrix_2d(n: usize) -> Vec<Vec<f64>> {
    let mut matrix: Vec<Vec<f64>> = vec![vec![0.0; n]; n];
    for i in 0..n {
        for j in 0..n {
            matrix[i][j] = random_range(-1.0..=1.0);
        }
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
            let a: Vec<Vec<f64>> = zufallsmatrix_2d(n);
            let b: Vec<Vec<f64>> = zufallsmatrix_2d(n);
            let mut c: Vec<Vec<f64>>= vec![vec![0.0; n]; n];

            split_at_2d(&a, &b, &mut c, n);
        }
    }
    println!("Benchmark fertig");
}
