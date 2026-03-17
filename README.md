# Tennessee Eastman Digital Twin Lab

Implementação de um digital twin executável da planta Tennessee Eastman (TEP) em Rust, baseada no modelo clássico de Downs & Vogel (1993).

**Foco:** fidelidade dinâmica, simulação determinística, separação clara entre modelo da planta e camada de controle.

---

## Estrutura do Repositório

```
tennessee-eastman-process/    ← modelo FORTRAN original (referência)
tennessee-eastman-service/    ← digital twin em Rust
  ├── core/                   ← modelo matemático da planta (EDOs, integrador)
  └── service/                ← executável: runtime, controladores, dashboard
       └── src/
            ├── controllers/  ← trait Controller + ControllerBank + implementações
            ├── runtime.rs    ← loop de simulação (planta + controle + CSV logger)
            └── main.rs       ← configuração do experimento e injeção de controladores
analysis/                     ← pacote Python para visualização (plots dos CSVs)
docs/                         ← documentação técnica do projeto
```

## Documentação

| Arquivo                                       | Conteúdo                                                                    |
| --------------------------------------------- | --------------------------------------------------------------------------- |
| [01-premissas.md](docs/01-premissas.md)       | Premissas de modelagem: válvulas, cold start, ordem do loop, desacoplamento |
| [02-glossario.md](docs/02-glossario.md)       | Glossário de termos e nomenclatura do TEP                                   |
| [03-falhas.md](docs/03-falhas.md)             | Relatório de falhas e troubleshooting da simulação                          |
| [04-experimentos.md](docs/04-experimentos.md) | Registro científico de experimentos (Obs → Hip → Int → Res → Conc)          |
| [05-disturbios.md](docs/05-disturbios.md)     | Referência dos 20 distúrbios IDV do TEP                                     |
| [06-controle.md](docs/06-controle.md)         | Camada de controle: arquitetura injetável, malhas ativas, XMEAS/XMV         |

## Análise e Visualização

O diretório `analysis/` contém o pacote Python `tep-analysis` para gerar plots dos CSVs de simulação. Detalhes em [analysis/README.md](analysis/README.md).

```bash
cd analysis
poetry install
poetry run plot --csv ../tennessee-eastman-service/simulation_log.csv
```

## Princípios de Arquitetura

- A **planta é determinística** e não contém lógica de controle
- O **controle é injetável** via trait `Controller` + `ControllerBank`
- O tempo avança de forma explícita via integração numérica (RK4)
- Controladores são configuráveis sem modificar o runtime
- Futuramente: gestão externa de controladores via Kubernetes CRDs

## Referências

- Downs, J. J., & Vogel, E. F. (1993). *A Plant-Wide Industrial Process Control Problem*. Computers & Chemical Engineering, 17(3), 245-255.

## Status

Baseline estável (Exp 10/11, 20h sem ISD). Controladores desacoplados e injetáveis. Próximo milestone: testes de rejeição de distúrbio com a nova arquitetura.
