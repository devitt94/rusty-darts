use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use rand::prelude::*;
use rand_distr::Normal;
use std::collections::HashMap;

const INNER_BULLSEYE_RADIUS_MM: f64 = 6.35;
const OUTER_BULLSEYE_RADIUS_MM: f64 = 16.0;
const DOUBLE_RING_INNER_RADIUS_MM: f64 = 162.0;
const DOUBLE_RING_OUTER_RADIUS_MM: f64 = 170.0;
const TRIPLE_RING_INNER_RADIUS_MM: f64 = 99.0;
const TRIPLE_RING_OUTER_RADIUS_MM: f64 = 107.0;

#[derive(Clone)]
#[pyclass(get_all)]
pub struct DartThrow {
    pub x: f64,
    pub y: f64,
}

impl std::fmt::Display for DartThrow {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

pub const BULLSEYE: DartThrow = DartThrow {
    x: 0.0,
    y: 0.0,
};

pub const TREBLE_20: DartThrow = DartThrow {
    x: 0.0,
    y: 103.0,
};

pub const TREBLE_19: DartThrow = DartThrow {
    x: -31.83,
    y: -97.96,
};

pub const TREBLE_18: DartThrow = DartThrow {
    x: 60.54,
    y: 83.33,
};

pub const TREBLE_17: DartThrow = DartThrow {
    x: 31.83,
    y: -97.96,
};

pub const TREBLE_16: DartThrow = DartThrow {
    x: -83.33,
    y: -60.54,
};

pub const TREBLE_15: DartThrow = DartThrow {
    x: 83.33,
    y: -60.54,
};


pub const TREBLE_14: DartThrow = DartThrow {
    x: -97.96,
    y: 31.83,
};


#[derive(Eq, Hash, PartialEq, Clone)]
#[pyclass(get_all)]
struct Score {
    value: u8,
    representation: String,
}

#[pymethods]
impl Score {
    fn __str__(slf: PyRef<'_, Self>) -> PyResult<String>  {
        Ok(slf.representation.clone())
    }

    fn __repr__(slf: PyRef<'_, Self>) -> PyResult<String>  {
        Ok(slf.representation.clone())
    }
}

fn throw_dart(dispersion_mm: f64, target: &DartThrow) -> DartThrow {
    let x: f64 = thread_rng().sample(Normal::new(target.x, dispersion_mm).unwrap());
    let y: f64 = thread_rng().sample(Normal::new(target.y, dispersion_mm).unwrap());
    DartThrow {
        x,
        y,
    }
}

fn compute_score(throw: DartThrow) -> Score {
    let distance_from_center: f64 = (throw.x.powi(2) + throw.y.powi(2)).sqrt();
    let value: u8;
    let representation: String;
    if distance_from_center <= INNER_BULLSEYE_RADIUS_MM {
        value = 50;
        representation = "Inner Bullseye".to_string();
    } else if distance_from_center <= OUTER_BULLSEYE_RADIUS_MM {
        value = 25;
        representation = "Outer Bullseye".to_string();
    } else if distance_from_center >= DOUBLE_RING_INNER_RADIUS_MM {
        value = 0;
        representation = "Miss".to_string();
    } else {
        let segment = compute_segment(throw);
        if in_treble_ring(distance_from_center) {
            value = segment * 3;
            representation = format!("Triple {}", segment);
        } else if in_double_ring(distance_from_center) {
            value = segment * 2;
            representation = format!("Double {}", segment);
        } else {
            value = segment;
            representation = format!("Single {}", segment);
        }
    }

    Score {
        value,
        representation,
    }
}

fn in_treble_ring(distance: f64) -> bool {
    TRIPLE_RING_INNER_RADIUS_MM <= distance && distance <= TRIPLE_RING_OUTER_RADIUS_MM
}

fn in_double_ring(distance: f64) -> bool {
    DOUBLE_RING_INNER_RADIUS_MM <= distance && distance <= DOUBLE_RING_OUTER_RADIUS_MM
}

fn compute_segment(throw: DartThrow) -> u8 {
    const SEGMENTS: [u8; 20] = [
        20, 1, 18, 4, 13, 6, 10, 15, 2, 17, 3, 19, 7, 16, 8, 11, 14, 9, 12, 5,
    ];
    let angle = (throw.x.atan2(throw.y).to_degrees() + 360.0) % 360.0;
    let segment = ((angle + 9.0) / 18.0) % 20.0;
    SEGMENTS[segment as usize]
}

struct SimulationInput {
    aim: DartThrow,
    dispersion_mm: f64,
    n_sims: i32,
}

#[derive(Clone)]
#[pyclass(get_all)]
pub struct SimulationResult {
    average_score: f64,
    score_counts: HashMap<Score, i32>,
    std_dev: f64,
}

impl std::fmt::Display for SimulationResult {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Average score: {}", self.average_score)
    }

}

#[pymethods]
impl SimulationResult {
    fn __str__(slf: PyRef<'_, Self>) -> PyResult<String>  {
        Ok(format!("SimulationResult(average_score={}, std_dev={})", slf.average_score, slf.std_dev))
    }

    fn __repr__(slf: PyRef<'_, Self>) -> PyResult<String>  {
        Ok(format!("SimulationResult(average_score={}, std_dev={})", slf.average_score, slf.std_dev))
    }
}

fn run_simulation(sim_input: &SimulationInput) -> SimulationResult {
    let mut total_score: i32 = 0;
    let mut score_counts: HashMap<Score, i32> = HashMap::new();
    for _i in 0..sim_input.n_sims {
        let throw = throw_dart(sim_input.dispersion_mm, &sim_input.aim);
        let score = compute_score(throw);
        total_score += score.value as i32;
        if score_counts.contains_key(&score) {
            let count = score_counts.get_mut(&score).unwrap();
            *count += 1;
        } else {
            score_counts.insert(score, 1);
        }
    }
    let average_score: f64 = f64::from(total_score) / sim_input.n_sims as f64;
    let std_dev: f64 = score_counts
        .iter()
        .map(|(score, count)| {
            let score_diff = score.value as f64 - average_score;
            score_diff.powi(2) * *count as f64
        })
        .sum::<f64>()
        .sqrt()
        / sim_input.n_sims as f64;
    SimulationResult {
        average_score,
        std_dev,
        score_counts,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_segment_due_north_equals_20() {
        let throw = DartThrow {
            x: 0.0,
            y: 1.0,
        };
        assert_eq!(compute_segment(throw), 20);
    }

    #[test]
    fn test_compute_segment_due_south_equals_3() {
        let throw = DartThrow {
            x: 0.0,
            y: -1.0,
        };
        assert_eq!(compute_segment(throw), 3);
    }

    #[test]
    fn test_compute_segment_due_east_equals_6() {
        let throw = DartThrow {
            x: 1.0,
            y: 0.0,
        };
        assert_eq!(compute_segment(throw), 6);
    }

    #[test]
    fn test_compute_segment_due_west_equals_11() {
        let throw = DartThrow {
            x: -1.0,
            y: 0.0,
        };
        assert_eq!(compute_segment(throw), 11);
    }
}

#[pyfunction]
fn simulate(n_sims: i32, dispersion_mm: f64, aim: DartThrow) -> PyResult<SimulationResult> {
    let sim_input = SimulationInput {
        aim,
        dispersion_mm,
        n_sims,
    };
    let sim_result = run_simulation(&sim_input);

    Ok(sim_result)
}

/// A Python module implemented in Rust.
#[pymodule]
#[pyo3(name = "rusty_darts")]
fn _internal(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(simulate, m)?)?;
    
    m.add_class::<DartThrow>()?;
    m.add_class::<SimulationResult>()?;
    m.add_class::<Score>()?;
    m.add("BULLSEYE", BULLSEYE)?;
    m.add("TREBLE_20", TREBLE_20)?;
    m.add("TREBLE_19", TREBLE_19)?;
    m.add("TREBLE_18", TREBLE_18)?;
    m.add("TREBLE_17", TREBLE_17)?;
    m.add("TREBLE_16", TREBLE_16)?;
    m.add("TREBLE_15", TREBLE_15)?;
    m.add("TREBLE_14", TREBLE_14)?;

    Ok(())
}
