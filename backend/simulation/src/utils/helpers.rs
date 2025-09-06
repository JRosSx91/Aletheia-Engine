pub fn analyze_universe_type(level: u8) -> &'static str {
    match level {
        0 => "Estéril",
        1 => "Químico",
        2 => "Nuclear",
        3 => "Estelar",
        4 => "Complejo",
        5 => "Reproductivo",
        _ => "Trascendente",
    }
}