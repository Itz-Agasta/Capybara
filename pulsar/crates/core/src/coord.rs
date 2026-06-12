/// Coordinate conversion utilities.
///
/// J2000 3D unit vector:
///   x = cos(dec) * cos(ra)   — vernal equinox direction
///   y = cos(dec) * sin(ra)   — RA = 6h
///   z = sin(dec)              — north celestial pole
///
/// All trig uses radians. Input RA/Dec in degrees.

/// Convert RA/Dec (degrees) → J2000 3D unit vector [x, y, z].
///
/// ```
/// use pulsar_core::coord::ra_dec_to_j2000;
/// let vec = ra_dec_to_j2000(116.55, 21.62); // Jupiter
/// assert!((vec[0] - 0.521).abs() < 0.01);
/// ```
pub fn ra_dec_to_j2000(ra_deg: f64, dec_deg: f64) -> [f64; 3] {
    todo!("Implement: convert degrees to radians, compute x/y/z")
}

/// Convert J2000 3D unit vector [x, y, z] → RA/Dec (degrees).
///
/// ```
/// use pulsar_core::coord::j2000_to_ra_dec;
/// let (ra, dec) = j2000_to_ra_dec([0.521, 0.107, 0.870]);
/// assert!((ra - 116.55).abs() < 0.1);
/// ```
pub fn j2000_to_ra_dec(vec: [f64; 3]) -> (f64, f64) {
    todo!("Implement: atan2 for RA, asin for Dec, convert to degrees")
}

/// Compute angular distance between two RA/Dec positions in arcseconds.
///
/// Used in the calibration loop to check convergence:
///   error < 30 arcsec → done
pub fn angular_distance_arcsec(ra1: f64, dec1: f64, ra2: f64, dec2: f64) -> f64 {
    todo!("Implement: haversine or dot-product angular separation → arcseconds")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jupiter_vector() {
        let vec = ra_dec_to_j2000(116.55, 21.62);
        // Jupiter: roughly [0.521, 0.107, 0.870] from plan3.md
        assert!((vec[0] - 0.521).abs() < 0.01, "x should be ~0.521, got {}", vec[0]);
        assert!((vec[1] - 0.107).abs() < 0.01, "y should be ~0.107, got {}", vec[1]);
        assert!((vec[2] - 0.870).abs() < 0.01, "z should be ~0.870, got {}", vec[2]);
    }

    #[test]
    fn test_roundtrip() {
        let (ra, dec) = (84.14, -5.37); // M42
        let vec = ra_dec_to_j2000(ra, dec);
        let (ra2, dec2) = j2000_to_ra_dec(vec);
        assert!((ra - ra2).abs() < 0.001, "RA roundtrip failed");
        assert!((dec - dec2).abs() < 0.001, "Dec roundtrip failed");
    }

    #[test]
    fn test_angular_distance_zero() {
        let d = angular_distance_arcsec(100.0, 20.0, 100.0, 20.0);
        assert!(d < 0.001, "same point should have ~0 distance");
    }
}
