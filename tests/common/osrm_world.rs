use std::{collections::{HashMap, HashSet}, fs::File, path::PathBuf};
use crate::Point;
use cucumber::World;

use super::{osm::OSMNode, osm_db::OSMDb};

#[derive(Debug, Default, World)]
pub struct OSRMWorld {
    pub feature_path: Option<PathBuf>,
    pub scenario_id: String,
    pub feature_digest: String,
    pub osrm_digest: String,
    pub osm_id: u64,
    pub profile: String,

    pub known_osm_nodes: HashSet<char>,
    pub known_locations: HashMap<char, Point>,

    pub osm_db: OSMDb,
}

impl OSRMWorld {
    pub fn set_scenario_specific_paths_and_digests(&mut self, path: Option<PathBuf>) {
        self.feature_path.clone_from(&path);

        let file = File::open(path.clone().unwrap())
            .unwrap_or_else(|_| panic!("filesystem broken? can't open file {:?}", path));
        self.feature_digest = chksum_md5::chksum(file)
            .expect("md5 could not be computed")
            .to_hex_lowercase();
    }

    pub fn make_osm_id(&mut self) -> u64 {
        // number implicitly starts a 1. This is in line with previous implementations
        self.osm_id += 1;
        self.osm_id
    }

    pub fn add_osm_node(&mut self, name: char, location: Point, id: Option<u64>) {
        if self.known_osm_nodes.contains(&name) {
            panic!("duplicate node: {name}");
        }
        let id = match id {
            Some(id) => id,
            None => self.make_osm_id(),
        };
        let node = OSMNode {
            id,
            lat: location.y(),
            lon: location.x(),
            tags: HashMap::from([("name".to_string(), name.to_string())]),
        };

        self.known_osm_nodes.insert(name);
        self.osm_db.add_node(node);
    }

    pub fn add_location(&mut self, name: char, location: Point) {
        if self.known_locations.contains_key(&name) {
            panic!("duplicate location: {name}")
        }
        self.known_locations.insert(name, location);
    }
}