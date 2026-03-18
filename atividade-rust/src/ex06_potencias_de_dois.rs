// Exercício 06 — Potências de Dois
// Imprime 1, 2, 4, 8, 16, ... enquanto o valor for menor que n.
// Complexidade: O(log n) — i dobra a cada iteração, portanto são necessários
// log₂(n) passos para que i ultrapasse n.

pub fn potencias_de_dois(n: u64) {
    let mut i: u64 = 1; // começa em 2⁰ = 1
    while i < n {
        println!("{}", i);
        i *= 2; // dobra o valor: equivalente a i = i * 2
    }
}

// Versão auxiliar que retorna os valores para facilitar testes
pub fn potencias_de_dois_vec(n: u64) -> Vec<u64> {
    let mut resultado = Vec::new();
    let mut i: u64 = 1;
    while i < n {
        resultado.push(i);
        i *= 2;
    }
    resultado
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn potencias_ate_16() {
        // Potências menores que 16: 1, 2, 4, 8
        assert_eq!(potencias_de_dois_vec(16), vec![1, 2, 4, 8]);
    }

    #[test]
    fn potencias_ate_1() {
        // Nenhuma potência é menor que 1
        assert_eq!(potencias_de_dois_vec(1), vec![]);
    }

    #[test]
    fn potencias_ate_32() {
        assert_eq!(potencias_de_dois_vec(32), vec![1, 2, 4, 8, 16]);
    }

    #[test]
    fn potencias_ate_100() {
        assert_eq!(potencias_de_dois_vec(100), vec![1, 2, 4, 8, 16, 32, 64]);
    }
}
