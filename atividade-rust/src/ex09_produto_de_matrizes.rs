// Exercício 09 — Produto de Matrizes
// Multiplica duas matrizes quadradas n×n usando o algoritmo clássico.
// Complexidade: O(n³) — três loops aninhados, cada um de 0 até n.

pub fn produto_de_matrizes(a: &[Vec<i64>], b: &[Vec<i64>]) -> Vec<Vec<i64>> {
    let n = a.len();
    // Cria a matriz resultado C inicializada com zeros (n linhas × n colunas)
    let mut c = vec![vec![0i64; n]; n];

    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                // C[i][j] acumula o produto escalar da linha i de A com a coluna j de B
                c[i][j] += a[i][k] * b[k][j];
            }
        }
    }
    c // retorna com posse — o chamador recebe o resultado
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multiplicar_por_identidade() {
        let a = vec![vec![1, 0], vec![0, 1]]; // matriz identidade
        let b = vec![vec![5, 6], vec![7, 8]];
        let c = produto_de_matrizes(&a, &b);
        assert_eq!(c, vec![vec![5, 6], vec![7, 8]]);
    }

    #[test]
    fn multiplicacao_2x2() {
        let a = vec![vec![1, 2], vec![3, 4]];
        let b = vec![vec![5, 6], vec![7, 8]];
        let c = produto_de_matrizes(&a, &b);
        // [1*5+2*7, 1*6+2*8] = [19, 22]
        // [3*5+4*7, 3*6+4*8] = [43, 50]
        assert_eq!(c, vec![vec![19, 22], vec![43, 50]]);
    }

    #[test]
    fn multiplicacao_3x3() {
        let a = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let b = vec![vec![9, 8, 7], vec![6, 5, 4], vec![3, 2, 1]];
        let c = produto_de_matrizes(&a, &b);
        assert_eq!(c[0], vec![30, 24, 18]);
        assert_eq!(c[1], vec![84, 69, 54]);
        assert_eq!(c[2], vec![138, 114, 90]);
    }
}
