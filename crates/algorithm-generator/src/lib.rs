use algorithm_core::Specification;

#[derive(Clone, Debug)]
pub struct GeneratedAlgorithm {
    pub name: String,
    pub steps: Vec<String>,
}

pub struct AlgorithmGenerator;

impl AlgorithmGenerator {
    pub fn generate(spec: &Specification) -> GeneratedAlgorithm {
        GeneratedAlgorithm {
            name: "GeneratedAlgorithm".into(),
            steps: vec![
                "validate inputs".into(),
                format!("solve: {}", spec.problem),
                "validate result".into(),
                "optimize execution plan".into(),
            ],
        }
    }
}
