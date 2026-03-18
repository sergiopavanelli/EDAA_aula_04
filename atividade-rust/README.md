# Reescrita de Algoritmos em Rust

**Disciplina:** Estruturas de Dados e Análise de Algoritmos  
**Aula:** 04 – Implementação de Algoritmos em Rust  
**Professor:** Alexandre de Oliveira  

---

## Estrutura do projeto

```
atividade-rust/
├── Cargo.toml
├── README.md
├── src/
│   ├── main.rs
│   ├── ex01_verificar_primeiro.rs
│   ├── ex02_somar_lista.rs
│   ├── ex03_busca_binaria.rs
│   ├── ex04_pares_com_soma.rs
│   ├── ex05_imprimir_pares_e_pares.rs
│   ├── ex06_potencias_de_dois.rs
│   ├── ex07_fibonacci_recursivo.rs
│   ├── ex08_ordenacao_bolha.rs
│   ├── ex09_produto_de_matrizes.rs
│   └── ex10_merge_sort.rs
└── prints_execucao/
    └── saida_cargo_test.txt
```

---

## Exercício 01 — Verificar Primeiro

**Complexidade:** O(1)

**Lógica do algoritmo:**
Verifica se a lista está vazia e, se não estiver, retorna o primeiro elemento diretamente pelo índice `lista[0]`. Nenhum percurso é realizado.

**Justificativa da complexidade:**
O algoritmo executa exatamente duas operações independentemente do tamanho da lista: verificar se está vazia e acessar o índice 0. O número de operações não cresce com `n`, portanto a complexidade é O(1) — tempo constante.

**Conceitos de Rust aplicados:**
- `Option<i32>`: representa a ausência de valor (`None`) de forma segura, substituindo o `None` do Python.
- `&[i32]`: referência imutável a um slice — a função apenas lê os dados, sem tomar posse.
- `.is_empty()`: método idiomático para verificar se o slice está vazio.

---

## Exercício 02 — Somar Lista

**Complexidade:** O(n)

**Lógica do algoritmo:**
Percorre todos os elementos da lista um a um, acumulando a soma em uma variável `total` inicializada em zero. Ao final do percurso, retorna o total.

**Justificativa da complexidade:**
Existe um único loop `for` que visita cada um dos `n` elementos exatamente uma vez. O número de operações cresce linearmente com `n`, portanto a complexidade é O(n).

**Conceitos de Rust aplicados:**
- `let mut total: i32 = 0`: mutabilidade explícita exigida pelo compilador.
- `for &elemento in lista`: desestrutura a referência com `&`, obtendo o valor inteiro diretamente em vez de uma referência.

---

## Exercício 03 — Busca Binária

**Complexidade:** O(log n)

**Lógica do algoritmo:**
Mantém dois ponteiros (`esquerda` e `direita`) delimitando o intervalo de busca. A cada iteração, calcula o índice do meio e compara com o alvo: se for igual, retorna o índice; se o alvo for maior, descarta a metade esquerda; se for menor, descarta a metade direita. Termina ao encontrar o alvo ou ao intervalo se tornar inválido.

**Justificativa da complexidade:**
A cada iteração do `while`, o intervalo de busca é dividido ao meio: n → n/2 → n/4 → … → 1. São necessários log₂(n) passos para reduzir o intervalo a zero. Para n = 1.000.000, isso representa apenas ~20 iterações.

**Conceitos de Rust aplicados:**
- `isize`: tipo com sinal usado para `esquerda` e `direita`, pois `direita` pode chegar a -1 sem gerar underflow.
- `meio as usize`: conversão explícita de tipo ao acessar o slice (índices em Rust são `usize`).
- `Option<usize>`: substitui o retorno `-1` do Python — `Some(idx)` quando encontrado, `None` quando não encontrado.

---

## Exercício 04 — Pares com Soma

**Complexidade:** O(n²)

**Lógica do algoritmo:**
Usa dois loops aninhados para examinar todos os pares possíveis `(lista[i], lista[j])` com `j > i`. Para cada par cuja soma é igual ao alvo, imprime os dois valores.

**Justificativa da complexidade:**
O loop externo executa `n` vezes. Para cada iteração do loop externo, o loop interno executa `n - i - 1` vezes. No total, o número de pares examinados é n(n-1)/2, que é O(n²). O crescimento é quadrático: dobrar `n` quadruplica o tempo de execução.

**Conceitos de Rust aplicados:**
- Ranges `(i+1)..n`: idiomáticos em Rust, sem custo de alocação extra.
- Versão auxiliar `pares_com_soma_vec` retorna `Vec<(i32, i32)>` para permitir testes automatizados.

---

## Exercício 05 — Imprimir Pares e Pares

**Complexidade:** O(n²)

**Lógica do algoritmo:**
Possui dois blocos sequenciais. O **Bloco 1** percorre a lista uma vez e imprime cada elemento. O **Bloco 2** usa dois loops aninhados para imprimir todos os pares `(x, y)` — inclusive `(x, x)` — formados pelos elementos da lista.

**Justificativa da complexidade:**
Pela **regra da soma**, a complexidade de blocos sequenciais é determinada pelo bloco dominante. O Bloco 1 é O(n) e o Bloco 2 é O(n²). Como O(n²) domina O(n), a complexidade total é O(n²).

**Conceitos de Rust aplicados:**
- Dois blocos `for` sequenciais sem nenhuma sintaxe especial — mesma lógica da análise Big-O.
- `for &x in lista`: desestrutura a referência para obter o valor diretamente.

---

## Exercício 06 — Potências de Dois

**Complexidade:** O(log n)

**Lógica do algoritmo:**
Começa com `i = 1` e dobra `i` a cada iteração (`i *= 2`) enquanto `i < n`. Imprime o valor de `i` em cada passo, gerando a sequência 1, 2, 4, 8, 16, ...

**Justificativa da complexidade:**
A cada iteração, `i` é multiplicado por 2. Para que `i` chegue de 1 até `n`, são necessários exatamente ⌊log₂(n)⌋ passos. O número de iterações cresce logaritmicamente com `n`, portanto a complexidade é O(log n).

**Conceitos de Rust aplicados:**
- `u64`: inteiro sem sinal de 64 bits, suportando valores até ~1,8 × 10¹⁹ — necessário pois potências de dois crescem rapidamente.
- `i *= 2`: operador de atribuição composta, idêntico ao Python.
- Rust detecta overflow de inteiros em modo debug (panic) e usa wrapping em release.

---

## Exercício 07 — Fibonacci Recursivo

**Complexidade:** O(2ⁿ)

**Lógica do algoritmo:**
Calcula F(n) pela definição matemática recursiva: F(0) = 0, F(1) = 1, F(n) = F(n-1) + F(n-2). Cada chamada para n > 1 gera duas chamadas recursivas, formando uma árvore binária de chamadas.

**Justificativa da complexidade:**
A árvore de chamadas recursivas tem altura `n` e, no pior caso, quase 2ⁿ nós — pois cada nó gera dois filhos e muitos subproblemas são recalculados repetidamente (sem memoização). Para n = 30, já são ~10⁹ operações. A complexidade é O(2ⁿ) — crescimento exponencial.

**Conceitos de Rust aplicados:**
- Recursão funciona exatamente como em Python — mesma sintaxe de chamada.
- `u64`: necessário pois Fibonacci cresce exponencialmente e ultrapassa rapidamente o limite de `i32`.
- Caso base `if n <= 1 { return n; }` é a âncora que impede recursão infinita.

---

## Exercício 08 — Ordenação Bolha (Bubble Sort)

**Complexidade:** O(n²)

**Lógica do algoritmo:**
Realiza `n` passagens pelo vetor. Em cada passagem, compara pares de elementos adjacentes e os troca se estiverem fora de ordem. Após cada passagem completa, o maior elemento não ordenado "sobe" para sua posição final. O loop interno encolhe a cada passagem pois o final já está ordenado.

**Justificativa da complexidade:**
O loop externo executa `n` vezes. O loop interno executa `n - i - 1` vezes para cada `i`. O total de comparações é: (n-1) + (n-2) + ... + 1 = n(n-1)/2 ≈ n²/2, que é O(n²). Dobrar `n` quadruplica o tempo de execução.

**Conceitos de Rust aplicados:**
- `&mut [i32]`: empréstimo mutável do slice — permite modificar o conteúdo in-place sem tomar posse.
- `lista.swap(j, j + 1)`: método idiomático e seguro para trocar dois elementos, substituindo a desestruturação `a, b = b, a` do Python.
- O compilador garante que não há dois empréstimos mutáveis simultâneos do mesmo dado.

---

## Exercício 09 — Produto de Matrizes

**Complexidade:** O(n³)

**Lógica do algoritmo:**
Implementa o algoritmo clássico de multiplicação de matrizes: C[i][j] é o produto escalar da linha `i` de A com a coluna `j` de B. Para cada par (i, j), percorre todos os `k` valores acumulando `A[i][k] * B[k][j]` em `C[i][j]`.

**Justificativa da complexidade:**
Há três loops aninhados, cada um executando `n` vezes: o loop de `i` (linhas de A), o loop de `j` (colunas de B) e o loop de `k` (produto escalar). O total de multiplicações é n × n × n = n³. Dobrar `n` multiplica o tempo por 8.

**Conceitos de Rust aplicados:**
- `Vec<Vec<i64>>`: representa uma matriz como vetor de vetores; `i64` evita overflow nas multiplicações.
- `vec![vec![0i64; n]; n]`: macro `vec!` para inicializar a matriz C com zeros.
- `a[i][k]`: acesso por índice funciona igualmente ao Python; Rust verifica os limites em modo debug.
- Retorna `Vec<Vec<i64>>` com posse — o chamador recebe e gerencia o resultado.

---

## Exercício 10 — Merge Sort

**Complexidade:** O(n log n)

**Lógica do algoritmo:**
Algoritmo de ordenação divide-e-conquista em duas fases:
1. **Divisão:** divide recursivamente a lista ao meio até obter sublistas de tamanho 1 (já ordenadas por definição).
2. **Fusão (merge):** combina pares de sublistas ordenadas em uma sublista maior ordenada, comparando os menores elementos de cada metade.

**Justificativa da complexidade:**
A divisão gera uma árvore de recursão com log₂(n) níveis (pois a lista é dividida ao meio a cada nível). Em cada nível, a fase de fusão processa todos os `n` elementos no total (distribuídos entre as chamadas daquele nível). Portanto: n elementos × log n níveis = O(n log n). É muito mais eficiente que O(n²) do Bubble Sort para entradas grandes.

**Conceitos de Rust aplicados:**
- `Vec<i32>` por valor (com posse): a função recebe, divide e retorna novos vetores — sem modificação in-place.
- `lista[..meio].to_vec()`: fatiamento e cópia para um novo `Vec` alocado no heap.
- `Vec::with_capacity(n)`: pré-aloca o espaço exato para evitar realocações durante a fusão.
- `extend_from_slice(&esquerda[i..])`: adiciona os elementos restantes de forma eficiente.
- Função auxiliar privada `fn merge(...)`: extrai a lógica de fusão, seguindo o princípio de responsabilidade única.
