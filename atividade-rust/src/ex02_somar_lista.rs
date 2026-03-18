// Exercício 02 — Somar Lista
// Percorre todos os elementos da lista e retorna a soma total.
// Complexidade: O(n) — cada elemento é visitado exatamente uma vez.

pub fn somar_lista(lista: &[i32]) -> i32 {
    let mut total: i32 = 0; // variável mutável explícita (exigido pelo Rust)
    // &elemento desestrutura a referência, obtendo o valor diretamente
    for &elemento in lista {
        total += elemento;
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lista_vazia() {
        assert_eq!(somar_lista(&[]), 0);
    }

    #[test]
    fn soma_positivos() {
        assert_eq!(somar_lista(&[1, 2, 3, 4, 5]), 15);
    }

    #[test]
    fn soma_com_negativos() {
        assert_eq!(somar_lista(&[-1, 2, -3, 4]), 2);
    }

    #[test]
    fn soma_elemento_unico() {
        assert_eq!(somar_lista(&[7]), 7);
    }
}
