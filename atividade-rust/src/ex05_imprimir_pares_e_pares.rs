// Exercício 05 — Imprimir Pares e Pares
// Dois blocos sequenciais:
//   Bloco 1: imprime cada elemento individualmente — O(n)
//   Bloco 2: imprime todos os pares (i, j), inclusive (i, i) — O(n²)
// Complexidade total: O(n²) — o bloco quadrático domina pela regra da soma.

pub fn imprimir_pares_e_pares(lista: &[i32]) {
    // Bloco 1 — O(n): percorre cada elemento uma única vez
    for &x in lista {
        println!("{}", x);
    }

    // Bloco 2 — O(n²): todos os pares ordenados (x, y), incluindo x == y
    for &x in lista {
        for &y in lista {
            println!("({}, {})", x, y);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Verificamos apenas que a função executa sem pânico (saída vai ao terminal)
    #[test]
    fn executa_sem_panico_lista_normal() {
        imprimir_pares_e_pares(&[1, 2, 3]);
    }

    #[test]
    fn executa_sem_panico_lista_vazia() {
        imprimir_pares_e_pares(&[]);
    }

    #[test]
    fn executa_sem_panico_elemento_unico() {
        imprimir_pares_e_pares(&[5]);
    }
}
