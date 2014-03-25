use pm_types::{
    BinPlacement1d, ItemModel, Objective, Problem, ProblemFamily, Solution, SolutionMetrics,
    SolveMeta,
};

pub fn solve_ffd(problem: &Problem) -> Option<Solution> {
    solve(problem, true)
}

pub fn solve_bfd(problem: &Problem) -> Option<Solution> {
    solve(problem, false)
}

fn solve(problem: &Problem, first_fit: bool) -> Option<Solution> {
    if problem.family != ProblemFamily::ManufacturerPalletLoading
        || problem.objective != Objective::MinBins
    {
        return None;
    }
    let sizes = match &problem.items.model {
        ItemModel::OneDimItems { sizes } => sizes.clone(),
        _ => return None,
    };
    if sizes.is_empty() {
        return Some(empty_bins(problem, first_fit));
    }

    let mut indexed: Vec<(usize, f64)> = sizes.iter().copied().enumerate().collect();
    indexed.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

    let capacity = 1.0;
    let mut bins: Vec<Vec<usize>> = vec![];
    let mut remainders: Vec<f64> = vec![];

    for (idx, size) in indexed {
        if size > capacity {
            return None;
        }
        let bin_idx = if first_fit {
            remainders.iter().position(|r| *r >= size)
        } else {
            remainders
                .iter()
                .enumerate()
                .filter(|(_, r)| **r >= size)
                .min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
                .map(|(i, _)| i)
        };
        match bin_idx {
            Some(i) => {
                bins[i].push(idx);
                remainders[i] -= size;
            }
            None => {
                bins.push(vec![idx]);
                remainders.push(capacity - size);
            }
        }
    }

    let bins_1d: Vec<BinPlacement1d> = bins
        .iter()
        .enumerate()
        .map(|(i, items)| {
            let used = capacity - remainders[i];
            BinPlacement1d {
                bin_id: i as u32,
                item_indices: items.clone(),
                used,
            }
        })
        .collect();

    let bin_count = bins.len() as u32;
    Some(Solution {
        family: problem.family,
        placements: vec![],
        rect_placements: vec![],
        placements_3d: vec![],
        bins_1d,
        bins: vec![],
        metrics: SolutionMetrics {
            bin_count,
            objective_value: bin_count as f64,
            ..SolutionMetrics::default()
        },
        feasible: true,
        meta: SolveMeta {
            algorithm: if first_fit {
                "first_fit_decreasing".to_string()
            } else {
                "best_fit_decreasing".to_string()
            },
            backend: "cpu".to_string(),
            iterations: 0,
            elapsed_ms: 0,
        },
    })
}

fn empty_bins(problem: &Problem, first_fit: bool) -> Solution {
    Solution {
        family: problem.family,
        placements: vec![],
        rect_placements: vec![],
        placements_3d: vec![],
        bins_1d: vec![],
        bins: vec![],
        metrics: SolutionMetrics::default(),
        feasible: true,
        meta: SolveMeta {
            algorithm: if first_fit {
                "first_fit_decreasing".to_string()
            } else {
                "best_fit_decreasing".to_string()
            },
            backend: "cpu".to_string(),
            iterations: 0,
            elapsed_ms: 0,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ffd_dosa_tight() {
        let sizes = vec![
            0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.4, 0.4, 0.4, 0.4, 0.4,
            0.4,
        ];
        let p = Problem::one_dim_bin_packing(sizes);
        let sol = solve_ffd(&p).unwrap();
        assert_eq!(sol.metrics.bin_count, 9);
        let opt = 9.0;
        assert!(sol.metrics.bin_count as f64 <= (11.0 / 9.0) * opt + 6.0 / 9.0 + 0.01);
    }
}
