// Exercício 07 — Fibonacci Recursivo
// Calcula o n-ésimo número de Fibonacci por recursão dupla.
// Complexidade: O(2ⁿ) — cada chamada gera duas novas chamadas recursivas,
// formando uma árvore de chamadas com altura n e ~2ⁿ nós no pior caso.
//
// ATENÇÃO: não testar com n > 40, pois o tempo de execução se torna muito alto.

pub fn fibonacci_recursivo(n: u64) -> u64 {
    // Caso base: F(0) = 0, F(1) = 1 — âncora da recursão
    if n <= 1 {
        return n;
    }
    // Caso recursivo: F(n) = F(n-1) + F(n-2)
    fibonacci_recursivo(n - 1) + fibonacci_recursivo(n - 2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn casos_base() {
        assert_eq!(fibonacci_recursivo(0), 0);
        assert_eq!(fibonacci_recursivo(1), 1);
    }

    #[test]
    fn sequencia_fibonacci() {
        // F: 0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55
        assert_eq!(fibonacci_recursivo(2), 1);
        assert_eq!(fibonacci_recursivo(3), 2);
        assert_eq!(fibonacci_recursivo(5), 5);
        assert_eq!(fibonacci_recursivo(7), 13);
        assert_eq!(fibonacci_recursivo(10), 55);
    }
}
