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
    let throw = throw_dart(50.0);
    println!("x: {}, y: {}", throw.x, throw.y);
    let score = compute_score(throw);
    println!("Score: {} ({})", score.value, score.representation);
}


fn throw_dart(dispersion_mm: f64) -> DartThrow {
    let x: f64 = thread_rng().sample(Normal::new(0.0, dispersion_mm).unwrap());
    let y: f64 = thread_rng().sample(Normal::new(0.0, dispersion_mm).unwrap());
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