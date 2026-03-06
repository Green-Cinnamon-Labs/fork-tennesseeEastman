
//! ## Sobre índices de variáveis (TEP × Rust)
//!
//! No 'Tennessee Eastman Process (TEP)' original (artigo e código FORTRAN),
//! as variáveis são numeradas a partir de 1:
//!
//! - XMEAS(1), XMEAS(2), ..., XMEAS(41)
//! - XMV(1), XMV(2), ..., XMV(12)
//!
//! Já em Rust, vetores (`Vec<T>`) são indexados a partir de 0:
//!
//! - outputs[0], outputs[1], ..., outputs[n-1]
//!
//! ### Regra obrigatória neste código
//!
//! - `index` em `VarMeta` é **sempre 0-based**
//! - `tag` (`"XMEAS(9)"`, `"XMV(3)"`, etc.) é **apenas referência humana**
//!
//! Exemplo:
//!
//! ```text
//! Artigo TEP : XMEAS(9)  → Reactor temperature
//! Código Rust: outputs[8]
//! ```
//!
//! Usar índices 1-based no código causará acesso à variável errada.
//! Esta convenção é fixa e deve ser respeitada em todo o projeto.

#[derive(Debug, Clone, Copy)]
pub enum VarKind {
    State,
    Measurement,   // XMEAS
    Manipulated,   // XMV
}

#[derive(Debug, Clone, Copy)]
pub struct VarMeta {
    pub index: usize,
    pub name: &'static str,
    pub tag: &'static str,
    pub unit: &'static str,
    pub kind: VarKind,
}   

pub const MEASUREMENTS: &[VarMeta] = &[
    VarMeta {
        index: 8,
        name: "Reactor temperature",
        tag: "XMEAS(9)",
        unit: "°C",
        kind: VarKind::Measurement,
    },
    VarMeta {
        index: 6,
        name: "Reactor pressure",
        tag: "XMEAS(7)",
        unit: "kPa",
        kind: VarKind::Measurement,
    },
];

pub const MANIPULATED: &[VarMeta] = &[
    VarMeta {
        index: 8,
        name: "Stripper steam valve",
        tag: "XMV(9)",
        unit: "%",
        kind: VarKind::Manipulated,
    },
];