use rand::random_range;
use crate::unsicher_1d;

/*
    Testet ob die Matrixmultiplikation korrekt implementiert wurde
*/
pub fn testen() {
    // Mix aus Matrizen mit gerader und ungerader Größe
    let größen: Vec<usize> = vec![6, 7, 32, 37, 64, 77, 90, 141, 180, 187, 200, 273, 300, 411, 460, 673, 690, 899, 906, 1000];

    for &n in &größen {
        let a: Vec<Vec<f64>> = zufallsmatrix_2d(n);
        let b: Vec<Vec<f64>> = zufallsmatrix_2d(n);

        // vergleich aller Implemtierungen mit der Standardimplementierung im 2D Fall
        let mut v: Vec<Vec<f64>> = vec![vec![0.0; n]; n];
        basis_2d(&a, &b, &mut v, n);

        // umwandeln in 1d Matrizen
        let d: Vec<f64> = a.iter().flatten().copied().collect();
        let e: Vec<f64> = b.iter().flatten().copied().collect();
        let mut ergebnis: Vec<f64> = vec![0.0; n * n];

        unsicher_1d(&d, &e, &mut ergebnis, n);
        assert!(vergleich(&umwandeln(&ergebnis, n), &v, n), "unsicher_1d.rs ist falsch für n = {}", n);
    }
    println!("Alle Tests sind erfolgreich");
}

// wandelt einen 1D Vektor zu nxn 2D Matrix um
fn umwandeln(v: &Vec<f64>, n: usize) -> Vec<Vec<f64>> {
    let mut matrix: Vec<Vec<f64>> = vec![vec![0f64; n]; n];
    for i in 0..n {
        for j in 0..n {
            matrix[i][j] = v[i * n + j];
        }
    }
    matrix
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
    Erstellt eine 2D Matrix mit Zufallswerten im Bereich [-1.0,1.0]
*/
fn zufallsmatrix_2d(n: usize) -> Vec<Vec<f64>> {
    let mut matrix: Vec<Vec<f64>> = vec![vec![0.0; n]; n];
    for i in 0..n {
        for j in 0..n {
            matrix[i][j] = random_range(-1.0..=1.0);
        }
    }
    matrix
}

/*
    Standardimplementation
*/
fn basis_2d(a: &Vec<Vec<f64>>, b: &Vec<Vec<f64>>, c: &mut Vec<Vec<f64>>, n: usize) {
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