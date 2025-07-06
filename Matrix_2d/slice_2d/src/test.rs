use crate::{slice_2d, zufallsmatrix_2d};

/*
    Testet ob die Matrixmultiplikation korrekt implementiert wurde
*/
pub fn testen() {
    // Mix aus Matrizen mit gerader und ungerader Größe
    let größen: Vec<usize> = vec![6, 7, 32, 37, 64, 77, 90, 141, 180, 187, 200, 273, 300, 411, 460, 673, 690, 899, 906, 1000];

    for &n in &größen {
            let a: Vec<Vec<f64>> = zufallsmatrix_2d(n);
            let b: Vec<Vec<f64>> = zufallsmatrix_2d(n);
            let mut ergebnis: Vec<Vec<f64>> = vec![vec![0.0; n]; n];

            // vergleich aller Implemtierungen mit der Standardimplementierung im 2D Fall
            let mut v: Vec<Vec<f64>> = vec![vec![0.0; n]; n];
            basis(&a, &b, &mut v, n);

            slice_2d(&a, &b, &mut ergebnis, n);
            assert!(vergleich(&ergebnis, &v, n), "slice_2d.rs ist falsch für n = {}", n);
    }
    println!("Alle Tests sind erfolgreich");
}

// vergleicht zwei 2D Matrizen 
fn vergleich(a: &Vec<Vec<f64>>, b: &Vec<Vec<f64>>, n: usize) -> bool {
    let genauigkeit = 1e-10;
    for i in 0..n {
        for j in 0..n {
            if (a[i][j] - b[i][j]).abs() > genauigkeit {
                return false;
            }
        }
    }
    true
}

/*
    Standardimplementation
*/
fn basis(a: &Vec<Vec<f64>>, b: &Vec<Vec<f64>>, c: &mut Vec<Vec<f64>>, n: usize) {
    for i in 0..n {
        for j in 0..n {
            let mut summe = 0.0;
            for k in 0..n {
                summe = summe + a[i][k] * b[k][j];
            }
            c[i][j] = summe;
        }
    }
}