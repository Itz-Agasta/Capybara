// NAIF IDs
// Used to disambiguate planet names from barycenters in JPL Horizons.
pub const NAIF_IDS: &[(&str, &str)] = &[
    ("sun", "10"),
    ("mercury", "199"),
    ("venus", "299"),
    ("earth", "399"),
    ("moon", "301"),
    ("mars", "499"),
    ("jupiter", "599"),
    ("saturn", "699"),
    ("uranus", "799"),
    ("neptune", "899"),
    ("pluto", "999"),
    ("ceres", "2000001"),
    ("vesta", "2000004"),
    ("io", "501"),
    ("europa", "502"),
    ("ganymede", "503"),
    ("callisto", "504"),
    ("titan", "606"),
    ("triton", "801"),
];

pub fn lookup_naif(name: &str) -> Option<&'static str> {
    NAIF_IDS
        .iter()
        .find(|(n, _)| n.eq_ignore_ascii_case(name))
        .map(|(_, id)| *id)
}

// TODO: implement resolve(), _resolve_horizons(), _resolve_simbad()
