pub const LIMIT: f64 = 30_000_000.0;
pub const TARGET: f64 = LIMIT / 2.0;

pub fn next_base(base: f64, used: f64) -> f64 {
    base * (1.0 + (used - TARGET) / TARGET / 8.0)
}

/// Gas demand at a given fee: during a shock blocks are full, otherwise demand falls with price.
pub fn demand(base: f64, reference: f64, shock: bool) -> f64 {
    if shock {
        return LIMIT;
    }
    (TARGET * reference / base).clamp(0.0, LIMIT)
}

pub fn simulate(start: f64, shock_blocks: usize, blocks: usize) -> Vec<(f64, f64)> {
    let mut base = start;
    let mut out = Vec::with_capacity(blocks);
    for i in 0..blocks {
        let used = demand(base, start, i < shock_blocks);
        out.push((base, used / LIMIT));
        base = next_base(base, used);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn update_rule_bounds() {
        assert!((next_base(100.0, LIMIT) - 112.5).abs() < 1e-9);
        assert!((next_base(100.0, 0.0) - 87.5).abs() < 1e-9);
        assert!((next_base(100.0, TARGET) - 100.0).abs() < 1e-9);
    }

    #[test]
    fn shock_raises_then_fee_recovers() {
        let path = simulate(10.0, 10, 200);
        let peak = path.iter().map(|p| p.0).fold(0.0, f64::max);
        assert!(peak > 10.0 * 1.125f64.powi(9));
        assert!((path.last().unwrap().0 - 10.0).abs() / 10.0 < 0.05);
    }
}
