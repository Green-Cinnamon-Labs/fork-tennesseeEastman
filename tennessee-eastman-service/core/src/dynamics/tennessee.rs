pub struct TennesseeEastmanModel {
    pub params: Params,
}

impl DynamicModel for TennesseeEastmanModel {
    fn derivatives(
        &self,
        state: &[f64],
        inputs: &Inputs,
        dx: &mut [f64],
    ) {
        // placeholder temporário
        for i in 0..state.len() {
            dx[i] = 0.0;
        }
    }
}