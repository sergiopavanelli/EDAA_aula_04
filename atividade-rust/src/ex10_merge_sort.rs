// Exercício 10 — Merge Sort
// Algoritmo de ordenação divide-e-conquista:
//   1. Divide a lista ao meio recursivamente até listas de tamanho 1.
//   2. Funde (merge) pares de listas ordenadas em uma lista maior ordenada.
// Complexidade: O(n log n) — log n níveis de recursão, cada nível processa n elementos.

pub fn merge_sort(lista: Vec<i32>) -> Vec<i32> {
    // Caso base: lista com 0 ou 1 elementos já está ordenada
    if lista.len() <= 1 {
        return lista; // devolve a posse sem modificar
    }

    let meio = lista.len() / 2;
    // Divide em duas metades (to_vec() cria novas alocações)
    let esquerda = merge_sort(lista[..meio].to_vec());
    let direita = merge_sort(lista[meio..].to_vec());

    // Funde as duas metades já ordenadas
    merge(esquerda, direita)
}

// Função auxiliar privada: funde dois vetores ordenados em um único vetor ordenado
fn merge(esquerda: Vec<i32>, direita: Vec<i32>) -> Vec<i32> {
    // Pré-aloca o espaço exato para evitar realocações
    let mut resultado = Vec::with_capacity(esquerda.len() + direita.len());
    let mut i = 0; // índice para esquerda
    let mut j = 0; // índice para direita

    // Compara e insere o menor elemento de cada metade
    while i < esquerda.len() && j < direita.len() {
        if esquerda[i] <= direita[j] {
            resultado.push(esquerda[i]);
            i += 1;
        } else {
            resultado.push(direita[j]);
            j += 1;
        }
    }

    // Adiciona os elementos restantes de cada metade (apenas uma delas terá elementos)
    resultado.extend_from_slice(&esquerda[i..]);
    resultado.extend_from_slice(&direita[j..]);
    resultado
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ordena_desordenado() {
        assert_eq!(merge_sort(vec![5, 3, 8, 1, 2]), vec![1, 2, 3, 5, 8]);
    }

    #[test]
    fn ja_ordenado() {
        assert_eq!(merge_sort(vec![1, 2, 3, 4, 5]), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn ordem_inversa() {
        assert_eq!(merge_sort(vec![5, 4, 3, 2, 1]), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn lista_vazia() {
        assert_eq!(merge_sort(vec![]), vec![]);
    }

    #[test]
    fn um_elemento() {
        assert_eq!(merge_sort(vec![42]), vec![42]);
    }

    #[test]
    fn numeros_negativos() {
        assert_eq!(merge_sort(vec![-3, 1, -1, 2, 0]), vec![-3, -1, 0, 1, 2]);
    }

    #[test]
    fn elementos_repetidos() {
        assert_eq!(merge_sort(vec![3, 1, 3, 2, 1]), vec![1, 1, 2, 3, 3]);
    }
}
