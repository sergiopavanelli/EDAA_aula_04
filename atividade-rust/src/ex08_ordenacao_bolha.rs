// Exercício 08 — Ordenação Bolha (Bubble Sort)
// Ordena a lista in-place: a cada passagem, o maior elemento "sobe" para o fim.
// Complexidade: O(n²) — dois loops aninhados, cada um percorrendo até n elementos.

pub fn ordenacao_bolha(lista: &mut [i32]) {
    let n = lista.len();
    // Loop externo: n passagens pelo vetor
    for i in 0..n {
        // Loop interno: compara pares adjacentes, excluindo os já ordenados no final
        for j in 0..(n - i - 1) {
            if lista[j] > lista[j + 1] {
                // .swap() é o idioma Rust para trocar dois elementos in-place de forma segura
                lista.swap(j, j + 1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ordena_desordenado() {
        let mut lista = vec![5, 3, 8, 1, 2];
        ordenacao_bolha(&mut lista);
        assert_eq!(lista, vec![1, 2, 3, 5, 8]);
    }

    #[test]
    fn ja_ordenado() {
        let mut lista = vec![1, 2, 3, 4, 5];
        ordenacao_bolha(&mut lista);
        assert_eq!(lista, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn ordem_inversa() {
        let mut lista = vec![5, 4, 3, 2, 1];
        ordenacao_bolha(&mut lista);
        assert_eq!(lista, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn lista_vazia() {
        let mut lista: Vec<i32> = vec![];
        ordenacao_bolha(&mut lista);
        assert_eq!(lista, vec![]);
    }

    #[test]
    fn um_elemento() {
        let mut lista = vec![42];
        ordenacao_bolha(&mut lista);
        assert_eq!(lista, vec![42]);
    }

    #[test]
    fn elementos_repetidos() {
        let mut lista = vec![3, 1, 3, 2, 1];
        ordenacao_bolha(&mut lista);
        assert_eq!(lista, vec![1, 1, 2, 3, 3]);
    }
}
