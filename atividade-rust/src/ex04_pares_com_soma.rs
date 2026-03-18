// Exercício 04 — Pares com Soma
// Encontra e imprime todos os pares (a, b) com a < b cujos valores somam ao alvo.
// Complexidade: O(n²) — dois loops aninhados percorrem todos os pares possíveis.

pub fn pares_com_soma(lista: &[i32], alvo: i32) {
    let n = lista.len();
    // Loop externo: percorre cada elemento como primeiro do par
    for i in 0..n {
        // Loop interno: começa em i+1 para evitar pares duplicados e (a, a)
        for j in (i + 1)..n {
            if lista[i] + lista[j] == alvo {
                println!("{} + {} = {}", lista[i], lista[j], alvo);
            }
        }
    }
}

// Versão auxiliar que retorna os pares em um Vec para facilitar testes
pub fn pares_com_soma_vec(lista: &[i32], alvo: i32) -> Vec<(i32, i32)> {
    let n = lista.len();
    let mut resultado = Vec::new();
    for i in 0..n {
        for j in (i + 1)..n {
            if lista[i] + lista[j] == alvo {
                resultado.push((lista[i], lista[j]));
            }
        }
    }
    resultado
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pares_encontrados() {
        let pares = pares_com_soma_vec(&[1, 2, 3, 4, 5], 5);
        assert_eq!(pares, vec![(1, 4), (2, 3)]);
    }

    #[test]
    fn sem_pares() {
        let pares = pares_com_soma_vec(&[1, 2, 3], 10);
        assert!(pares.is_empty());
    }

    #[test]
    fn lista_vazia() {
        let pares = pares_com_soma_vec(&[], 5);
        assert!(pares.is_empty());
    }

    #[test]
    fn par_unico() {
        let pares = pares_com_soma_vec(&[3, 7], 10);
        assert_eq!(pares, vec![(3, 7)]);
    }
}
