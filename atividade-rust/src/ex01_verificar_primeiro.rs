// Exercício 01 — Verificar Primeiro
// Retorna o primeiro elemento da lista, ou None se a lista estiver vazia.
// Complexidade: O(1) — acesso direto ao índice 0, independente do tamanho da lista.

pub fn verificar_primeiro(lista: &[i32]) -> Option<i32> {
    // Se a lista estiver vazia, não há primeiro elemento
    if lista.is_empty() {
        return None;
    }
    // Retorna o primeiro elemento encapsulado em Some
    Some(lista[0])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lista_vazia() {
        assert_eq!(verificar_primeiro(&[]), None);
    }

    #[test]
    fn lista_com_elementos() {
        assert_eq!(verificar_primeiro(&[42, 7, 3]), Some(42));
    }

    #[test]
    fn lista_com_um_elemento() {
        assert_eq!(verificar_primeiro(&[99]), Some(99));
    }

    #[test]
    fn primeiro_elemento_negativo() {
        assert_eq!(verificar_primeiro(&[-5, 10, 20]), Some(-5));
    }
}
