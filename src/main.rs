/* src/main.rs */

/** `tep-plant` — a aplicação: monta a Simulation da Tennessee Eastman Plant. `Simulation` cuida de
StateRegistry/thread por dentro — esse binário não monta isso manualmente.

NOTA (2026-08-13, branch feat/proc-macro-components): não existe mais `build_tep()`/`set_model()`
aqui — Reactor/Separator/Stripper/Compressor (agora `#[dynamic_model]`), os 12 atuadores, os 6
sensores e o controller se auto-registram via `inventory::submit!` escondido; `Simulation::run()`
descobre e monta a árvore de avaliação inteira sozinho.

NOTA (2026-09-09, issue #67): `Simulation` deixou de gerenciar o adaptador de rede — quem faz isso
agora é `monjolo::runtime::Runtime`, um supervisor persistente que sobrevive a qualquer número de
`reset()`s da simulação, trocando atomicamente pra qual instância de planta o adaptador aponta (o
servidor OPC-UA, uma vez subido, nunca mais cai). Desenho completo em
`spec-tennessee-eastman/docs/issue61_runtime_supervisor/nota_runtime_supervisor.md`.

NOTA (2026-09-10, issue #68): este binário parou de escolher `NumericalMethod`, caminho de config, e
endpoint OPC-UA em código — isso tudo é `Runtime::bootstrap()` (`monjolo::runtime`) fazendo, por
convenção: `application.toml` sempre na raiz do projeto, lido sozinho, sem caminho passado por
ninguém; `[monjolo].numerical_method`/`[monjolo.opcua].endpoint` como configuração, não Rust; e
`main()` sem NENHUM `#[cfg(feature = "opcua")]` — `Runtime::run()` decide sozinho como bloquear em
cada configuração de feature. Igual ao papel de `SpringApplication.run(Application.class, args)`.

Roda com: cargo run --bin tep-plant [--features opcua]
*/

/* `extern crate tennessee_eastman_process;` não é resquício de edição antiga (2018+ não precisa
disso pra USAR um crate) — é a única forma de o linker incluir o crate no binário final quando NADA
aqui referencia um símbolo dele por nome. Sem isso, `cargo build` compila a lib normalmente, mas o
`.exe` final não puxa o código de `actuators/`/`sensors/`/`controllers/`/`units/` do `.rlib` —
cada `inventory::submit!` escondido nesses módulos nunca roda, e `Simulation::run()` descobre zero
componentes (não é bug de `inventory`; é assim que Rust sempre linkou binário↔biblioteca — só ficou
visível agora que main() não chama mais nada da lib por nome).
*/
extern crate tennessee_eastman_process;

fn main() {
    monjolo::runtime::Runtime::bootstrap()
        .expect("Runtime::bootstrap encerrou com erro")
        .run();
}
