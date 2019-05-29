use std::u64;

pub fn selle(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
     let mut res: Vec<(usize, usize)> = Vec::new();
     for i in 0..input.len() {
         let mut value = u64::MIN;
         let mut candidates: Vec<(usize, usize)> = Vec::new();
         for j in 0 ..input[i].len() {
             if input[i][j] > value {
                 if candidates.len() > 0 {
                     candidates.clear();
                 }
                 candidates.push((i, j));
                 value = input[i][j];
             }
             else if input[i][j] == value {
                 candidates.push((i, j));
             }
         }
         'outer: for c in 0..candidates.len() {
             for ii in 0..input.len() {
                 if ii != candidates[c].0 && input[ii][candidates[c].1] < value {
                     continue 'outer;
                 }
             }
             println!("{:?}", candidates[c]);
             res.push(candidates[c]);
         }
     }
     res
}


#[cfg(test)]
mod tests {

use super::*;

fn ordina_selle(i: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut result = selle(i);
    result.sort();
    result
}

#[test]
fn singola_sella() {
    let i = vec![vec![9, 8, 7], vec![5, 3, 2], vec![6, 6, 7]];
    assert_eq!(vec![(1, 0)], selle(&i));
}

#[test]
fn matrice_vuota() {
    let i = vec![vec![], vec![], vec![]];
    let exp: Vec<(usize, usize)> = Vec::new();
    assert_eq!(exp, selle(&i));
}

#[test]
fn nessuna_sella() {
    let i = vec![vec![1, 2, 3], vec![3, 1, 2], vec![2, 3, 1]];
    let exp: Vec<(usize, usize)> = Vec::new();
    assert_eq!(exp, selle(&i));
}

#[test]
fn multiple_selle_in_col() {
    let i = vec![vec![4, 5, 4], vec![3, 5, 5], vec![1, 5, 4]];
    assert_eq!(
        vec![(0, 1), (1, 1), (2, 1)],
        ordina_selle(&i)
    );
}

#[test]
fn multiple_selle_in_riga() {
    let i = vec![vec![6, 7, 8], vec![5, 5, 5], vec![7, 5, 6]];
    assert_eq!(
        vec![(1, 0), (1, 1), (1, 2)],
        ordina_selle(&i)
    );
}

#[test]
fn sella_in_vertice() {
    let i = vec![vec![8, 7, 9], vec![6, 7, 6], vec![3, 2, 5]];
    assert_eq!(vec![(2, 2)], selle(&i));
}

#[test]
fn matrice_rettangolare_alta() {
    let i = vec![vec![1, 5], vec![3, 6], vec![2, 7], vec![3, 8]];
    assert_eq!(vec![(0, 1)], selle(&i));
}

#[test]
fn matrice_rettangolare_larga() {
    let i = vec![vec![3, 1, 3], vec![3, 2, 4]];
    assert_eq!(vec![(0, 0), (0, 2)], ordina_selle(&i));
}

#[test]
fn matrice_colonna() {
    let i = vec![vec![2], vec![1], vec![4], vec![1]];
    assert_eq!(vec![(1, 0), (3, 0)], ordina_selle(&i));
}

#[test]
fn matrice_riga() {
    let i = vec![vec![2, 5, 3, 5]];
    assert_eq!(vec![(0, 1), (0, 3)], ordina_selle(&i));
}

#[test]
fn tutte_selle() {
    let i = vec![vec![5, 5, 5], vec![5, 5, 5], vec![5, 5, 5]];
    assert_eq!(
        vec![
            (0, 0),
            (0, 1),
            (0, 2),
            (1, 0),
            (1, 1),
            (1, 2),
            (2, 0),
            (2, 1),
            (2, 2)
        ],
        ordina_selle(&i)
    );
}

}
