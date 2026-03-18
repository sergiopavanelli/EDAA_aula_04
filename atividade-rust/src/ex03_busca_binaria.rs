// Exercício 03 — Busca Binária
// Busca um elemento em uma lista ORDENADA dividindo o intervalo ao meio a cada passo.
// Retorna Some(índice) se encontrado, ou None caso contrário.
// Complexidade: O(log n) — o espaço de busca é cortado pela metade a cada iteração.
//
// ATENÇÃO: a lista deve estar em ordem crescente para o algoritmo funcionar corretamente.

pub fn busca_binaria(lista: &[i32], alvo: i32) -> Option<usize> {
    // Usamos isize porque direita pode chegar a -1 quando o alvo não existe
    let mut esquerda: isize = 0;
    let mut direita: isize = lista.len() as isize - 1;

    while esquerda <= direita {
        // Calcula o índice do meio evitando overflow
        let meio = (esquerda + direita) / 2;
        let idx = meio as usize; // converte para usize ao acessar o slice

        if lista[idx] == alvo {
            return Some(idx); // elemento encontrado
        } else if lista[idx] < alvo {
            esquerda = meio + 1; // descarta metade esquerda
        } else {
            direita = meio - 1; // descarta metade direita
        }
    }
    None // elemento não encontrado
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encontra_elemento_no_meio() {
        assert_eq!(busca_binaria(&[1, 3, 5, 7, 9], 5), Some(2));
    }

    #[test]
    fn elemento_nao_existe() {
        assert_eq!(busca_binaria(&[1, 3, 5, 7, 9], 4), None);
    }

    #[test]
    fn lista_vazia() {
        assert_eq!(busca_binaria(&[], 1), None);
    }

    #[test]
    fn primeiro_elemento() {
        assert_eq!(busca_binaria(&[1, 3, 5], 1), Some(0));
    }

    #[test]
    fn ultimo_elemento() {
        assert_eq!(busca_binaria(&[1, 3, 5], 5), Some(2));
    }

    #[test]
    fn lista_com_um_elemento_encontrado() {
        assert_eq!(busca_binaria(&[42], 42), Some(0));
    }

    #[test]
    fn lista_com_um_elemento_nao_encontrado() {
        assert_eq!(busca_binaria(&[42], 7), None);
    }
}
