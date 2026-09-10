/* src/main.rs */

/** `tep-plant` — a aplicação: monta a Simulation da Tennessee Eastman Plant. `Simulation` cuida de
StateRegistry/thread por dentro — esse binário não monta isso manualmente.

NOTA (2026-08-13, branch feat/proc-macro-components): não existe mais `build_tep()`/`set_model()`
aqui — Reactor/Separator/Stripper/Compressor (agora `#[dynamic_model]`), os 12 atuadores, os 6
sensores e o controller se auto-registram via `inventory::submit!` escondido; `Simulation::run()`
descobre e monta a árvore de avaliação inteira sozinho. Este binário só diz ONDE está a condição
inicial (`set_config_path`) — "de onde vem o arquivo é problema da aplicação, não do modelo/
framework" continua valendo, só que agora é o único trabalho que sobra aqui.

NOTA (2026-08-15): adaptador OPC-UA ligado sob a feature `opcua` (default OFF) — expõe os
Sensors/Actuators já catalogados em StateRegistry via `monjolo::adapter::opcua`. Sem a feature, este
binário integra a planta no tempo sem expor nada pra fora, como antes.

NOTA (2026-09-09, issue #67): `Simulation` deixou de gerenciar o adaptador de rede — quem faz isso
agora é `monjolo::runtime::Runtime`, um supervisor persistente que sobrevive a qualquer número de
`reset()`s da simulação, trocando atomicamente pra qual instância de planta o adaptador aponta (o
servidor OPC-UA, uma vez subido, nunca mais cai). Este binário não chama mais `Simulation::run()`
direto — em vez disso, dá a `Runtime::new()` uma fábrica (`Fn() -> Simulation`, chamada de novo a
cada reset) e bloqueia em `Runtime::wait_for_shutdown()`, que só retorna quando `control.shutdown`
for chamado via OPC-UA. Desenho completo em
`spec-tennessee-eastman/docs/issue61_runtime_supervisor/nota_runtime_supervisor.md`.

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

use monjolo::numerical_method::NumericalMethod;
use monjolo::simulation::Simulation;

const CONFIG_PATH: &str = "config/application.toml";
/* Porta IANA padrão de OPC-UA. Host é `127.0.0.1`, não `0.0.0.0` — de propósito: async-opcua-server
não distingue "endereço de bind" de "endereço anunciado" (`ServerInfo::base_endpoint()`, em
async-opcua-server/src/info.rs, monta o EndpointUrl que o servidor devolve em GetEndpoints/
FindServers a partir do MESMO `tcp_config.host` do bind). Com `0.0.0.0`, o servidor aceita conexão
em qualquer interface, mas anuncia a si mesmo como "opc.tcp://0.0.0.0:...", um endereço que nenhum
cliente consegue discar de verdade — quebra qualquer client que confie no EndpointUrl reportado pra
reconectar (ex.: UaExpert), mesmo que a conexão inicial/manual funcione (ex.: opcua-commander,
conexão direta sem essa segunda etapa). Se um dia isso precisar ser alcançável de outra máquina na
rede, precisa virar configurável (bind em 0.0.0.0, anunciar o IP real da máquina) — não dá pra ter
os dois com essa constante sozinha hoje.
*/
#[cfg(feature = "opcua")]
const OPCUA_ENDPOINT: &str = "opc.tcp://127.0.0.1:4840/tep/server/";

fn main() {
    let runtime = monjolo::runtime::Runtime::new(|| {
        let mut simulation = Simulation::new();
        simulation.set_config_path(CONFIG_PATH);
        simulation.set_numerical_method(NumericalMethod::RK4);
        simulation
    })
    .expect("Runtime::new encerrou com erro");

    let _ = &runtime; // só usado dentro dos blocos #[cfg(feature = "opcua")] abaixo
    #[cfg(feature = "opcua")]
    runtime.spawn_opcua_adapter(OPCUA_ENDPOINT);

    /* Sem a feature `opcua`, não há nenhum jeito de pedir shutdown de fora — este binário não tem
    mais nada a fazer além de manter a planta rodando indefinidamente. `park()` em loop em vez de
    `wait_for_shutdown()` evita bloquear pra sempre numa Condvar que, sem adapter, nunca ninguém vai
    notificar.
    */
    #[cfg(feature = "opcua")]
    runtime.wait_for_shutdown();
    #[cfg(not(feature = "opcua"))]
    loop {
        std::thread::park();
    }
}
