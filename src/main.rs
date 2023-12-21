use rand::prelude::*;
use rand_distr::Normal;

const INNER_BULLSEYE_RADIUS_MM: f64 = 12.7;
const OUTER_BULLSEYE_RADIUS_MM: f64 = 32.0;
const DOUBLE_RING_INNER_RADIUS_MM: f64 = 162.0;
const DOUBLE_RING_OUTER_RADIUS_MM: f64 = 170.0;
const TRIPLE_RING_INNER_RADIUS_MM: f64 = 99.0;
const TRIPLE_RING_OUTER_RADIUS_MM: f64 = 107.0;


struct DartThrow {
    x: f64,
    y: f64,
}

struct Score {
    value: u8,
    representation: String,
}

fn main() {

    let aim_bullseye = DartThrow { x: 0.0, y: 0.0 };
    let aim_t20 = DartThrow { x: 0.0, y: 103.0 };
    let aim_t19 = DartThrow { x: -31.83, y: -97.96};
    let aim_t18 = DartThrow { x: 60.54, y: 83.33};
    let aim_t17 = DartThrow { x: 31.83, y: -97.96};
    let aim_t16 = DartThrow { x: -83.33, y: -60.54};
    let aim_t15 = DartThrow { x: 83.33, y: -60.54};
    let aim_t14 = DartThrow { x: -97.96, y: 31.83};


    let score = compute_score(aim_bullseye);
    println!("Aiming at bullseye: {} ({})", score.representation, score.value);

    let score = compute_score(aim_t20);
    println!("Aiming at T20: {} ({})", score.representation, score.value);

    let score = compute_score(aim_t19);
    println!("Aiming at T19: {} ({})", score.representation, score.value);

    let score = compute_score(aim_t18);
    println!("Aiming at T18: {} ({})", score.representation, score.value);

    let score = compute_score(aim_t17);
    println!("Aiming at T17: {} ({})", score.representation, score.value);

    let score = compute_score(aim_t16);
    println!("Aiming at T16: {} ({})", score.representation, score.value);

    let score = compute_score(aim_t15);
    println!("Aiming at T15: {} ({})", score.representation, score.value);

    let score = compute_score(aim_t14);
    println!("Aiming at T14: {} ({})", score.representation, score.value);
    
}


fn throw_dart(dispersion_mm: f64, target: DartThrow) -> DartThrow {
    let x: f64 = thread_rng().sample(Normal::new(target.x, dispersion_mm).unwrap());
    let y: f64 = thread_rng().sample(Normal::new(target.y, dispersion_mm).unwrap());
    DartThrow { x, y }
}

fn in_treble_ring(distance: f64) -> bool {
    TRIPLE_RING_INNER_RADIUS_MM <= distance && distance <= TRIPLE_RING_OUTER_RADIUS_MM
}

fn in_double_ring(distance: f64) -> bool {
    DOUBLE_RING_INNER_RADIUS_MM <= distance && distance <= DOUBLE_RING_OUTER_RADIUS_MM
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

fn compute_segment(throw: DartThrow) -> u8 {
    const SEGMENTS: [u8; 20] = [20, 1, 18, 4, 13, 6, 10, 15, 2, 17, 3, 19, 7, 16, 8, 11, 14, 9, 12, 5];
    let angle = (throw.x.atan2(throw.y).to_degrees() + 360.0) % 360.0;
    let segment = (angle + 9.0) / 18.0;
    SEGMENTS[segment as usize]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_segment_due_north_equals_20() {
        let throw = DartThrow{x: 0.0, y: 1.0};
        assert_eq!(compute_segment(throw), 20);
    }

    #[test]
    fn test_compute_segment_due_south_equals_3() {
        let throw = DartThrow{x: 0.0, y: -1.0};
        assert_eq!(compute_segment(throw), 3);
    }

    #[test]
    fn test_compute_segment_due_east_equals_6() {
        let throw = DartThrow{x: 1.0, y: 0.0};
        assert_eq!(compute_segment(throw), 6);
    }

    #[test]
    fn test_compute_segment_due_west_equals_11() {
        let throw = DartThrow{x: -1.0, y: 0.0};
        assert_eq!(compute_segment(throw), 11);
    }
}