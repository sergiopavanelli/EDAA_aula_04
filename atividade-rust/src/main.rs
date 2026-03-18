mod ex01_verificar_primeiro;
mod ex02_somar_lista;
mod ex03_busca_binaria;
mod ex04_pares_com_soma;
mod ex05_imprimir_pares_e_pares;
mod ex06_potencias_de_dois;
mod ex07_fibonacci_recursivo;
mod ex08_ordenacao_bolha;
mod ex09_produto_de_matrizes;
mod ex10_merge_sort;

fn main() {
    // ── Exercício 01 ──────────────────────────────────────────────
    println!("=== Exercício 01 — Verificar Primeiro ===");
    println!("{:?}", ex01_verificar_primeiro::verificar_primeiro(&[10, 20, 30]));
    println!("{:?}", ex01_verificar_primeiro::verificar_primeiro(&[]));

    // ── Exercício 02 ──────────────────────────────────────────────
    println!("\n=== Exercício 02 — Somar Lista ===");
    println!("{}", ex02_somar_lista::somar_lista(&[1, 2, 3, 4, 5]));

    // ── Exercício 03 ──────────────────────────────────────────────
    println!("\n=== Exercício 03 — Busca Binária ===");
    println!("{:?}", ex03_busca_binaria::busca_binaria(&[1, 3, 5, 7, 9], 7));
    println!("{:?}", ex03_busca_binaria::busca_binaria(&[1, 3, 5, 7, 9], 4));

    // ── Exercício 04 ──────────────────────────────────────────────
    println!("\n=== Exercício 04 — Pares com Soma (alvo = 5) ===");
    ex04_pares_com_soma::pares_com_soma(&[1, 2, 3, 4, 5], 5);

    // ── Exercício 05 ──────────────────────────────────────────────
    println!("\n=== Exercício 05 — Imprimir Pares e Pares ===");
    ex05_imprimir_pares_e_pares::imprimir_pares_e_pares(&[1, 2, 3]);

    // ── Exercício 06 ──────────────────────────────────────────────
    println!("\n=== Exercício 06 — Potências de Dois (n = 64) ===");
    ex06_potencias_de_dois::potencias_de_dois(64);

    // ── Exercício 07 ──────────────────────────────────────────────
    println!("\n=== Exercício 07 — Fibonacci Recursivo (n = 0..10) ===");
    for i in 0..=10 {
        print!("F({})={} ", i, ex07_fibonacci_recursivo::fibonacci_recursivo(i));
    }
    println!();

    // ── Exercício 08 ──────────────────────────────────────────────
    println!("\n=== Exercício 08 — Ordenação Bolha ===");
    let mut lista = vec![64, 34, 25, 12, 22, 11, 90];
    ex08_ordenacao_bolha::ordenacao_bolha(&mut lista);
    println!("{:?}", lista);

    // ── Exercício 09 ──────────────────────────────────────────────
    println!("\n=== Exercício 09 — Produto de Matrizes ===");
    let a = vec![vec![1i64, 2], vec![3, 4]];
    let b = vec![vec![5i64, 6], vec![7, 8]];
    let c = ex09_produto_de_matrizes::produto_de_matrizes(&a, &b);
    for linha in &c {
        println!("{:?}", linha);
    }

    // ── Exercício 10 ──────────────────────────────────────────────
    println!("\n=== Exercício 10 — Merge Sort ===");
    let lista = vec![38, 27, 43, 3, 9, 82, 10];
    println!("{:?}", ex10_merge_sort::merge_sort(lista));
}
