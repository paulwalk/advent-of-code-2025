use std::collections::{HashMap, HashSet};
use uuid::Uuid;
use crate::models::coords_3d::{Coord3D, Coord3DPair};



pub struct Circuits {
    pub circuits: HashMap<uuid::Uuid, Circuit>,
}

impl Circuits {
    pub fn new() -> Self {
        Circuits { circuits: HashMap::new() }
    }

    pub fn add_pair(&mut self, pair: Coord3DPair) -> u8{
        let mut number_of_jbs_added_to_a_circuit: u8 = 0;
        let circuit_id_containing_p: Option<Uuid> = self.contains_junction_box(&pair.p);
        let circuit_id_containing_q: Option<Uuid> = self.contains_junction_box(&pair.q);
        if circuit_id_containing_p.is_none() && circuit_id_containing_q.is_none() {
            let circuit = Circuit::new(pair);
            self.circuits.insert(circuit.id, circuit);
            number_of_jbs_added_to_a_circuit = 2;
        } else if circuit_id_containing_p.is_some() && circuit_id_containing_q.is_none() {
            self.add_junction_box_to_circuit(circuit_id_containing_p.unwrap(), pair.q);
            number_of_jbs_added_to_a_circuit = 1;
        } else if circuit_id_containing_p.is_none() && circuit_id_containing_q.is_some() {
            self.add_junction_box_to_circuit(circuit_id_containing_q.unwrap(), pair.p);
            number_of_jbs_added_to_a_circuit = 1;
        } else {
            number_of_jbs_added_to_a_circuit = 0;
            // Both junction boxes are already in circuits
            if circuit_id_containing_p.unwrap() != circuit_id_containing_q.unwrap() {
                let mut new_circuit = Circuit {
                    id: uuid::Uuid::new_v4(),
                    junction_boxes: HashSet::new(),
                };
                for jb in self.circuits[&circuit_id_containing_p.unwrap()].junction_boxes.clone() {
                    new_circuit.add_junction_box(jb.clone());
                }
                for jb in self.circuits[&circuit_id_containing_q.unwrap()].junction_boxes.clone() {
                    new_circuit.add_junction_box(jb.clone());
                }
                self.circuits.insert(new_circuit.clone().id, new_circuit.clone());
                self.circuits.remove(&circuit_id_containing_p.unwrap());
                self.circuits.remove(&circuit_id_containing_q.unwrap());
            } else {
                // Both junction boxes are already in the same circuit, do nothing
            }
        }
        number_of_jbs_added_to_a_circuit
    }

    pub fn contains_junction_box(&self, jb: &Coord3D) -> Option<Uuid> {
        for circuit in self.circuits.values() {
            if circuit.contains(jb) {
                return Some(circuit.id);
            }
        }
        None
    }

    pub fn add_junction_box_to_circuit(&mut self, circuit_id: uuid::Uuid, jb: Coord3D) {
        if let Some(circuit) = self.circuits.get_mut(&circuit_id) {
            circuit.add_junction_box(jb);
        }
    }
}

#[derive(Clone)]
pub struct Circuit {
    pub id: uuid::Uuid,
    pub junction_boxes: HashSet<Coord3D>,
}

impl Circuit {
    pub fn new(pair: Coord3DPair) -> Self {
        let mut hash_set = HashSet::new();
        hash_set.insert(pair.p);
        hash_set.insert(pair.q);
        Circuit {
            id: uuid::Uuid::new_v4(),
            junction_boxes: hash_set,
        }
    }

    pub fn contains(&self, jb: &Coord3D) -> bool {
        self.junction_boxes.contains(&jb)
    }

    pub fn add_junction_box(&mut self, jb: Coord3D) {
        self.junction_boxes.insert(jb);
    }
}

