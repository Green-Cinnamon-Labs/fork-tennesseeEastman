# Ruídos de Medição do TEP

Referência: Downs & Vogel (1993), TEINIT em `teprob.f`. Implementação: `model.rs`, constante `XNS`.

Todo XMEAS recebe ruído gaussiano aditivo a cada atualização. O ruído é gerado pela função `white_noise(std, ds)` — método Irwin-Hall (soma de 12 amostras uniformes, aproxima N(0, σ²)). O gerador de números aleatórios é o LCG de `teprob.f`, seed inicial `4_651_207_995`.

---

## Mecanismo de atualização

| Grupo                                        | XMEAS | Frequência de atualização                 | Observação                                                                   |
| -------------------------------------------- | ----- | ----------------------------------------- | ---------------------------------------------------------------------------- |
| Contínuos (Tabela 4)                         | 1–22  | A cada passo do integrador (dt = 0.001 h) | Ruído adicionado ao valor calculado no mesmo passo                           |
| Analisador de gás (Tabela 5, reator + purge) | 23–36 | A cada 0.1 h (≈ 6 min simulados)          | Valor atualizado em `tgas`; entre atualizações, XMEAS retém o valor anterior |
| Analisador de produto (Tabela 5, produto)    | 37–41 | A cada 0.25 h (15 min simulados)          | Valor atualizado em `tprod`; mesmo comportamento de retenção                 |

O atraso de amostragem dos analisadores (0.1 h e 0.25 h) é parte do modelo original e é importante para controle: o controlador vê a composição com atraso, não em tempo real.

---

## Desvios padrão por variável (XNS)

### Tabela 4 — Medições contínuas (XMEAS 1–22)

| XMEAS | Nome                   | Unidade | σ (XNS) |
| ----- | ---------------------- | ------- | ------- |
| 1     | A Feed                 | kscmh   | 0.0012  |
| 2     | D Feed                 | kg/hr   | 18.0    |
| 3     | E Feed                 | kg/hr   | 22.0    |
| 4     | A&C Feed               | kscmh   | 0.05    |
| 5     | Recycle Flow           | kscmh   | 0.2     |
| 6     | Reactor Feed Rate      | kscmh   | 0.21    |
| 7     | Reactor Pressure       | kPa     | 0.3     |
| 8     | Reactor Level          | %       | 0.5     |
| 9     | Reactor Temperature    | °C      | 0.01    |
| 10    | Purge Rate             | kscmh   | 0.0017  |
| 11    | Sep Temperature        | °C      | 0.01    |
| 12    | Sep Level              | %       | 1.0     |
| 13    | Sep Pressure           | kPa     | 0.3     |
| 14    | Sep Underflow          | m³/hr   | 0.125   |
| 15    | Stripper Level         | %       | 1.0     |
| 16    | Stripper Pressure      | kPa     | 0.3     |
| 17    | Stripper Underflow     | m³/hr   | 0.115   |
| 18    | Stripper Temperature   | °C      | 0.01    |
| 19    | Stripper Steam Flow    | kg/hr   | 1.15    |
| 20    | Compressor Work        | kW      | 0.2     |
| 21    | Reactor CW Outlet Temp | °C      | 0.01    |
| 22    | Sep CW Outlet Temp     | °C      | 0.01    |

### Tabela 5 — Analisador de gás: reator + purge (XMEAS 23–36, Δt = 0.1 h)

| XMEAS | Nome                  | Unidade | σ (XNS) |
| ----- | --------------------- | ------- | ------- |
| 23    | Reactor A Composition | mol%    | 0.25    |
| 24    | Reactor B Composition | mol%    | 0.1     |
| 25    | Reactor C Composition | mol%    | 0.25    |
| 26    | Reactor D Composition | mol%    | 0.1     |
| 27    | Reactor E Composition | mol%    | 0.25    |
| 28    | Reactor F Composition | mol%    | 0.025   |
| 29    | Purge A Composition   | mol%    | 0.25    |
| 30    | Purge B Composition   | mol%    | 0.1     |
| 31    | Purge C Composition   | mol%    | 0.25    |
| 32    | Purge D Composition   | mol%    | 0.1     |
| 33    | Purge E Composition   | mol%    | 0.25    |
| 34    | Purge F Composition   | mol%    | 0.025   |
| 35    | Purge G Composition   | mol%    | 0.05    |
| 36    | Purge H Composition   | mol%    | 0.05    |

### Tabela 5 — Analisador de produto (XMEAS 37–41, Δt = 0.25 h)

| XMEAS | Nome                  | Unidade | σ (XNS) |
| ----- | --------------------- | ------- | ------- |
| 37    | Product D Composition | mol%    | 0.01    |
| 38    | Product E Composition | mol%    | 0.01    |
| 39    | Product F Composition | mol%    | 0.01    |
| 40    | Product G Composition | mol%    | 0.5     |
| 41    | Product H Composition | mol%    | 0.5     |

---

## Observações práticas

**Variáveis mais ruidosas:** D Feed (σ=18 kg/hr), E Feed (σ=22 kg/hr), Stripper Steam Flow (σ=1.15 kg/hr), Sep Level e Stripper Level (σ=1.0 %). São as que mais oscilam visivelmente na IHM.

**Variáveis mais limpas:** Temperaturas em geral (σ=0.01 °C), Purge Rate (σ=0.0017 kscmh), A Feed (σ=0.0012 kscmh). Mudanças nessas variáveis indicam com mais fidelidade a dinâmica real do processo.

**Implicação para experimentos:** ao aplicar um IDV e medir o efeito, use médias em janelas de tempo (ex: média de 5 min simulados) para suprimir o ruído antes de comparar com o baseline. Variáveis com σ alto precisam de janelas maiores para detectar deslocamentos pequenos.

**Analisadores com atraso:** ao aplicar um distúrbio que afeta composição (IDV 1, 2, 8), o efeito nas XMEAS 23–41 só aparece na próxima atualização do analisador (até 15 min simulados de atraso). Não confundir ausência de resposta imediata com ausência de efeito.
