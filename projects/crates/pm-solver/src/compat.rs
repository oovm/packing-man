use pm_types::{
    AlgorithmKind, Objective, Problem, ProblemArchetype, ProblemFamily, PmError, PmResult,
    SolverId,
};

pub fn validate(problem: &Problem, solver: SolverId) -> PmResult<()> {
    if problem.family != solver.family {
        return Err(PmError::UnsupportedSolver {
            family: problem.family,
            algorithm: format!("{:?}", solver.algorithm),
        });
    }
    if !algorithm_supported(problem, solver.algorithm) {
        return Err(PmError::IncompatibleProblem(format!(
            "algorithm {:?} incompatible with archetype {:?} objective {:?}",
            solver.algorithm,
            problem.archetype,
            problem.objective
        )));
    }
    Ok(())
}

fn algorithm_supported(problem: &Problem, algorithm: AlgorithmKind) -> bool {
    match problem.family {
        ProblemFamily::CircleSpherePacking => match algorithm {
            AlgorithmKind::GreedyInsertion | AlgorithmKind::ForceRelaxation => true,
            AlgorithmKind::AnalyticalConcentricRing => matches!(
                problem.archetype,
                Some(ProblemArchetype::EqualCirclesInCircle)
                    | Some(ProblemArchetype::CongruentCirclesCountBound)
                    | None
            ),
            AlgorithmKind::NlpLocalSearch => problem.objective == Objective::MaxRadiusSum,
            AlgorithmKind::GpuForceRelaxation => true,
            _ => false,
        },
        ProblemFamily::ManufacturerPalletLoading => matches!(
            algorithm,
            AlgorithmKind::FirstFitDecreasing
                | AlgorithmKind::BestFitDecreasing
                | AlgorithmKind::SkylineStrip
                | AlgorithmKind::ExtremePointBlf
        ),
        _ => false,
    }
}
