// core/src/dynamics/tennessee/tesub.rs

use crate::dynamics::tep::constants::TepConstants;

// ======== ESTADO DOS DISTÚRBIOS TEMPORAIS (COMMON /WLK/ + /RANDSD/)

/** ## State of the "walking disturbances"
(disturbances that vary continuously over time)

Equivalent to the FORTRAN COMMON block /WLK/.
Each of the 12 disturbance channels has its own cubic polynomial
that is periodically recalculated by `tesub5`.

Also contains the pseudo-random number generator seed
(equivalent to COMMON /RANDSD/).
*/
pub struct WalkState {
    /// Coeficientes do polinômio cúbico de distúrbio por canal
    /// f(t) = adist + h*(bdist + h*(cdist + h*ddist))
    /// onde h = t - tlast
    pub adist:  [f64; 12],
    pub bdist:  [f64; 12],
    pub cdist:  [f64; 12],
    pub ddist:  [f64; 12],

    /// Tempo do último ponto de controle por canal
    pub tlast:  [f64; 12],
    /// Tempo do próximo ponto de controle por canal
    pub tnext:  [f64; 12],

    /// Parâmetros de forma do polinômio (vêm do TEINIT)
    pub hspan:  [f64; 12],
    pub hzero:  [f64; 12],
    pub sspan:  [f64; 12],
    pub szero:  [f64; 12],
    pub spspan: [f64; 12],

    /// Flag de distúrbio ativo por canal (0 ou 1)
    pub idvwlk: [i32; 12],

    /// Seed do gerador pseudo-aleatório (COMMON /RANDSD/ G)
    pub rand_seed: f64,
}

impl WalkState {
    /// Inicializa o estado de distúrbios com os valores do TEINIT (teprob.f)
    pub fn new() -> Self {
        // Valores de HSPAN, HZERO, SSPAN, SZERO, SPSPAN do TEINIT
        // Índice 0 = canal 1 do FORTRAN
        let hspan  = [0.2, 0.7, 0.25, 0.7, 0.15, 0.15, 1.0, 1.0, 0.4, 1.5, 2.0, 1.5];
        let hzero  = [0.5, 1.0, 0.5,  1.0, 0.25, 0.25, 2.0, 2.0, 0.5, 2.0, 3.0, 2.0];
        let sspan  = [0.03, 0.003, 10.0, 10.0, 10.0, 10.0, 0.25, 0.25, 0.25, 0.0, 0.0, 0.0];
        let szero  = [0.485, 0.005, 45.0, 45.0, 35.0, 40.0, 1.0,  1.0,  0.0,  0.0, 0.0, 0.0];
        let spspan = [0.0; 12];

        // Estado inicial: polinômio = valor base, sem variação
        let adist = szero;

        Self {
            adist,
            bdist:  [0.0; 12],
            cdist:  [0.0; 12],
            ddist:  [0.0; 12],
            tlast:  [0.0; 12],
            tnext:  [0.1; 12],
            hspan,
            hzero,
            sspan,
            szero,
            spspan,
            idvwlk: [0; 12],
            rand_seed: 4_651_207_995.0, // seed inicial do TEINIT
        }
    }
}

impl Default for WalkState {
    fn default() -> Self {
        Self::new()
    }
}

// ======== TESUB1–4: Termodinâmica (funções puras)

/** ## TESUB1 – Mixture enthalpy
 
 Given a vector of mole fractions `z[8]` and temperature `t` (°C),
 computes the total mixture enthalpy `h`.
 
 The `ity` flag selects the phase model:
   0 = liquid phase
   1 = vapor phase
   2 = vapor phase with ideal gas (PV) correction
 */
pub fn tesub1(z: &[f64; 8], t: f64, ity: i32, c: &TepConstants) -> f64 {
    let mut h = 0.0_f64;

    if ity == 0 {
        // Fase líquida: HI = T*(AH + BH*T/2 + CH*T²/3) * 1.8
        for i in 0..8 {
            let hi = t * (c.ah[i] + c.bh[i] * t / 2.0 + c.ch[i] * t * t / 3.0);
            h += z[i] * c.xmw[i] * 1.8 * hi;
        }
    } else {
        // Fase vapor: HI = T*(AG + BG*T/2 + CG*T²/3) * 1.8 + AV
        for i in 0..8 {
            let hi = t * (c.ag[i] + c.bg[i] * t / 2.0 + c.cg[i] * t * t / 3.0);
            h += z[i] * c.xmw[i] * (1.8 * hi + c.av[i]);
        }
    }

    // Correção gás ideal (ity == 2): H -= R*(T + 273.15)
    if ity == 2 {
        h -= 3.57696e-6 * (t + 273.15);
    }

    h
}

/** ## TESUB2 – Temperature from enthalpy (Newton–Raphson)
Solves for T such that `tesub1(z, T, ity) == h_target`.
Returns `t_init` if convergence is not achieved within 100 iterations.
*/
pub fn tesub2(z: &[f64; 8], t_init: f64, h_target: f64, ity: i32, c: &TepConstants) -> f64 {
    let mut t = t_init;

    for _ in 0..100 {
        let h_test = tesub1(z, t, ity, c);
        let dh = tesub3(z, t, ity, c);
        let dt = -(h_test - h_target) / dh;
        t += dt;
        if dt.abs() < 1.0e-12 {
            return t;
        }
    }

    t_init
}

/** ## TESUB3 – Derivada dH/dT
Jacobiano usado internamente pelo Newton-Raphson de TESUB2.
*/
pub fn tesub3(z: &[f64; 8], t: f64, ity: i32, c: &TepConstants) -> f64 {
    let mut dh = 0.0_f64;

    if ity == 0 {
        for i in 0..8 {
            let dhi = (c.ah[i] + c.bh[i] * t + c.ch[i] * t * t) * 1.8;
            dh += z[i] * c.xmw[i] * dhi;
        }
    } else {
        for i in 0..8 {
            let dhi = (c.ag[i] + c.bg[i] * t + c.cg[i] * t * t) * 1.8;
            dh += z[i] * c.xmw[i] * dhi;
        }
    }

    if ity == 2 {
        dh -= 3.57696e-6;
    }

    dh
}

/** ## TESUB4 – Densidade da mistura líquida 
ρ = 1 / Σ( x_i * XMW_i / (AD_i + (BD_i + CD_i*T)*T) )
*/
pub fn tesub4(x: &[f64; 8], t: f64, c: &TepConstants) -> f64 {
    let v: f64 = (0..8)
        .map(|i| x[i] * c.xmw[i] / (c.ad[i] + (c.bd[i] + c.cd[i] * t) * t))
        .sum();
    1.0 / v
}

// ======== TESUB5–8: Distúrbios temporais (requerem estado mutável) 

/** ## TESUB7 – Pseudo-random number generator
Linear congruential generator identical to the original FORTRAN implementation.
Updates the seed stored in `state` at each call.
- `i >= 0` → returns a value in [0, 1)
- `i < 0`  → returns a value in [-1, 1)
*/
pub fn tesub7(i: i32, state: &mut WalkState) -> f64 {
    const MOD: f64 = 4_294_967_296.0;
    state.rand_seed = (state.rand_seed * 9_228_907.0).rem_euclid(MOD);
    if i >= 0 {
        state.rand_seed / MOD
    } else {
        2.0 * state.rand_seed / MOD - 1.0
    }
}

/** ## TESUB6 – Approximate Gaussian white noise
Sums 12 uniform samples and centers the result — normal distribution
approximation via the Central Limit Theorem (Irwin–Hall method).
Returns noise with standard deviation `std`.
 */
pub fn tesub6(std: f64, state: &mut WalkState) -> f64 {
    let sum: f64 = (0..12).map(|_| tesub7(0, state)).sum();
    (sum - 6.0) * std
}

/** ## TESUB5 – Updates the cubic disturbance polynomial for channel `idx`
Generates a new C1-continuous cubic polynomial segment from the
current point `(s, sp)` to a randomly defined future point.
 */
pub fn tesub5(idx: usize, s: f64, sp: f64, state: &mut WalkState) {
    let h   = state.hspan[idx]  * tesub7(-1, state) + state.hzero[idx];
    let idvflag = state.idvwlk[idx] as f64;
    let s1  = state.sspan[idx]  * tesub7(-1, state) * idvflag + state.szero[idx];
    let s1p = state.spspan[idx] * tesub7(-1, state) * idvflag;

    state.adist[idx] = s;
    state.bdist[idx] = sp;
    state.cdist[idx] = (3.0 * (s1 - s) - h * (s1p + 2.0 * sp)) / (h * h);
    state.ddist[idx] = (2.0 * (s - s1) + h * (s1p + sp))        / (h * h * h);
    state.tnext[idx] = state.tlast[idx] + h;
}

/** ## TESUB8 – Evaluates the cubic disturbance polynomial at time `time`
f(t) = A + h*(B + h*(C + h*D))   where h = time - tlast[idx]
Does not appear as a SUBROUTINE in teprob.f, but is used as an external
function to evaluate the polynomial defined in the /WLK/ block.
`idx` is zero-based (channels 0..11).
 */
pub fn tesub8(idx: usize, time: f64, state: &WalkState) -> f64 {
    let h = time - state.tlast[idx];
    state.adist[idx] + h * (state.bdist[idx] + h * (state.cdist[idx] + h * state.ddist[idx]))
}