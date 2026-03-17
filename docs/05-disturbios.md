# Distúrbios do TEP (IDV 1–20)

Referência: Downs & Vogel (1993). Cada IDV é ativado via `active_idv: vec![N]` em `main.rs`.

---

## IDV(1) — Step: razão A/C no feed combinado (stream 4)
A fração molar de A na alimentação A&C muda em step, mantendo B constante. Altera diretamente a estequiometria da reação (A+C→G, A+C→H). Aumenta a taxa de geração de gás e pressão no reator. **Testado no Exp 11 (Kp=0.1, ISD em 2h) e Exp 12 (Kp=1.0, pendente).**

## IDV(2) — Step: composição de B no feed (stream 4)
A fração molar do inerte B aumenta em step na alimentação A&C. B não reage, acumula no loop de reciclo e é removido apenas pelo purge. Eleva o inventário de inerte, aumenta pressão e reduz concentração de reagentes no reator.

## IDV(3) — Step: temperatura do D feed (stream 2)
Temperatura de alimentação de D sobe em step. D é alimentado como líquido; temperatura mais alta muda o enthalpy de entrada e a taxa de vaporização dentro do reator. Efeito moderado sobre temperatura e pressão do reator.

## IDV(4) — Step: temperatura de entrada da água de resfriamento do reator (+5°C)
A água de resfriamento do reator entra 5°C mais quente, reduzindo a capacidade de remoção de calor. Tende a elevar a temperatura do reator e deslocar o equilíbrio vapor-líquido. **Foi o distúrbio usado nos Exps 2 e 3 deste projeto (inadvertidamente ativo).**

## IDV(5) — Step: temperatura de entrada da água de resfriamento do condensador (+5°C)
Mesmo mecanismo do IDV(4), mas no condensador do separador. Reduz a condensação no separador, aumenta a fração de vapor no reciclo e eleva a carga sobre o compressor.

## IDV(6) — Step: perda total do A feed (stream 1)
Válvula do A feed fecha completamente. Sem A no reator, a reação para gradualmente. O inventário líquido cai (sem produto G/H sendo gerado) e a composição do loop muda drasticamente. Distúrbio severo — a planta sem controle adequado atinge ISD por nível baixo.

## IDV(7) — Step: queda de pressão no header de C (stream 4)
A pressão de fornecimento de C cai, reduzindo o fluxo de C para o reator. Efeito similar ao IDV(6) para C: menos reagente disponível, reação desbalanceada. Menos severo que o IDV(6) porque a redução é parcial.

## IDV(8) — Aleatório: variação na composição A/B/C do feed (stream 4)
Ruído randômico contínuo na composição da alimentação combinada. Mais difícil de rejeitar do que distúrbios em step porque não tem ponto de operação estacionário alternativo — exige malhas de controle robustas a variação persistente.

## IDV(9) — Aleatório: variação na temperatura do D feed (stream 2)
Ruído randômico na temperatura de entrada de D. Efeito entálpico contínuo sobre o reator. Geralmente mais brando que os distúrbios de composição porque a entalpia de D é uma perturbação de segundo ordem.

## IDV(10) — Aleatório: variação na temperatura do C feed (stream 4)
Ruído randômico na temperatura de C na alimentação combinada A&C. Análogo ao IDV(9) mas para o outro reagente gasoso principal.

## IDV(11) — Aleatório: variação na temperatura de entrada da água de resfriamento do reator
Flutuação contínua na temperatura de entrada do CW do reator. Torna o controle de temperatura do reator inerentemente mais difícil — o controlador de CW precisa compensar uma perturbação de entrada variável.

## IDV(12) — Aleatório: variação na temperatura de entrada da água de resfriamento do condensador
Análogo ao IDV(11) para o condensador. Afeta a eficiência de separação e a temperatura do separador (XMEAS(11)).

## IDV(13) — Deriva lenta: cinética de reação
A constante de velocidade da reação deriva lentamente ao longo do tempo (horas). Simula envelhecimento de catalisador ou mudança de condições de processo. Difícil de detectar sem instrumentação adequada; a planta opera aparentemente normal e só falha em horizonte longo.

## IDV(14) — Válvula travada: CW do reator (XMV(10))
A válvula de água de resfriamento do reator trava em sua posição atual. O controlador de temperatura (se existir) perde autoridade. A temperatura do reator passa a derivar conforme o calor de reação acumula sem ser removido.

## IDV(15) — Válvula travada: CW do condensador (XMV(11))
A válvula de resfriamento do condensador trava. A capacidade de condensação no separador fica fixa independentemente da demanda. Com variações de carga, o separador superaquece ou superesfria.

## IDV(16) — Válvula travada: D feed (XMV(1))
A válvula de alimentação de D trava. O feed de D fica fixo no valor do instante do travamento, independente de qualquer ação de controle.

## IDV(17) — Válvula travada: C feed / A&C feed (XMV(4))
A válvula de alimentação A&C trava. Afeta diretamente os dois reagentes principais A e C, tornando impossível ajustar a estequiometria via controle de feed.

## IDV(18) — Válvula travada: A feed (XMV(3))
A válvula de A feed trava. Combinado com uma malha de controle que tenta ajustar A feed para controlar nível ou composição, resulta em integrador windup.

## IDV(19) — Válvula travada: válvula de reciclo do compressor (XMV(5))
A válvula de reciclo do compressor trava. O operador perde controle sobre o bypass do compressor, afetando a pressão de sucção e a vazão do loop de reciclo.

## IDV(20) — Válvula travada: válvula de produto do stripper (XMV(8))
A válvula de produto final trava. A remoção de produto G/H fica fixa, levando a acúmulo ou escassez de produto no stripper dependendo da taxa de produção no reator.
