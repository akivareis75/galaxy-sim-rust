# Galaxy Collision Simulator (N-Body)

Uma simulação de dinâmica galáctica desenvolvida em **Rust**, focada no colapso gravitacional de sistemas esféricos com **momento angular nulo**. Este projeto utiliza o motor gráfico `macroquad` para visualização interativa em tempo real.

## 🌌 1. Sobre a Simulação

Diferente de simulações de galáxias espirais que dependem de velocidades orbitais para estabilidade, este modelo foca no **relaxamento violento** e no colapso radial de nuvens estelares em direção ao centro de massa comum.

### Especificações Físicas:
* **Modelo de N-Corpos:** Simulação de partículas onde estrelas são tratadas como massas de teste influenciadas pelos núcleos galácticos massivos.
* **Momento Angular Nulo ($L=0$):** As estrelas iniciam sem velocidade orbital em relação ao núcleo, resultando em um colapso puramente gravitacional em direção ao centro de massa.
* **Softening Gravitacional:** Implementação de um parâmetro de *softening* ($\epsilon$) para evitar singularidades matemáticas e acelerações infinitas durante encontros próximos entre partículas.
* **Integração de Euler:** A evolução temporal é calculada através de um passo de tempo fixo para garantir a estabilidade visual e física, independente da taxa de atualização do hardware.

## 🚀 2. Tecnologias Utilizadas

* **Linguagem:** [Rust](https://www.rust-lang.org/) (Edition 2021)
* **Framework Gráfico:** [macroquad](https://macroquad.rs/)
* **Física:** Integração N-Body customizada com tratamento de vetores 2D.

## 🛠️ 3. Como Executar

Certifique-se de ter o Rust e o Cargo instalados no seu sistema (ambiente otimizado para macOS/Apple Silicon).

1. **Clonar o repositório:**
   ```bash
   git clone [https://github.com/akivareis75/galaxy-sim-rust.git](https://github.com/akivareis75/galaxy-sim-rust.git)
   cd galaxy-sim-rust

2. Executar o projeto:
    ```bash
    cargo run --release
    ```

## 📊 4. Parâmetros de Simulação
    No arquivo src/main.rs, você pode ajustar as constantes fundamentais para observar diferentes fenômenos físicos:

Constante,Descrição,Impacto na Simulação
G,Constante Gravitacional,Define a intensidade da atração e a velocidade da fusão.
SOFTENING,Parâmetro de Suavização,"Evita que estrelas sejam ""projetadas"" no impacto central."
mass,Massa dos Núcleos,Controla a escala da atração mútua entre as galáxias.
dt,Delta Time,Controla a velocidade de passagem do tempo na simulação.

## 🧬 5. Objetivos Acadêmicos

Este repositório serve como base para estudos em Astroinformática, permitindo observar:

1. A formação de estruturas densas após o primeiro impacto.

2. O comportamento de relaxamento em sistemas de baixa rotação.

3. Eficiência algorítmica em simulações dinâmicas utilizando Rust.

## 🧹 6. Manutenção do Repositório

Para manter o repositório limpo, a pasta target/ deve ser ignorada para evitar o upload de arquivos binários pesados. Utilize o arquivo .gitignore incluído.

Desenvolvido como parte de estudos em Computação Científica, Astronomia e Integração de Sistemas.

### Comandos finais para atualizar o GitHub:
1. Abra o terminal na pasta do projeto.
2. Digite:
   ```bash
   git add README.md
   git commit -m "docs: Adiciona conteúdo completo e formatado ao README"
   git push origin main

# Galaxy Collision Simulator (N-Body)

![Simulação de Colisão](./screenshot/gc-rust.gif)