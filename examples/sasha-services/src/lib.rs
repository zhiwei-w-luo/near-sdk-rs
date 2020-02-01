use borsh::{BorshDeserialize, BorshSerialize};
use near_bindgen::collections::{Map, Set};
use near_bindgen::near_bindgen;
use std::collections::{HashMap, HashSet};

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

type Fullname = String;
type Graph = Vec<Fullname>;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Network {
    /// AccountId -> SerializedGraph
    pub edges: Map<Fullname, Set<Fullname>>,
    pub on_chain_edges: Map<Fullname, Set<Fullname>>,
    pub on_chain: Set<Fullname>,
}

#[near_bindgen(init => new)]
impl Network {
    pub fn new() -> Self {
        Self {
            edges: Map::new(b"g:".to_vec()),
            on_chain_edges: Map::new(b"oe:".to_vec()),
            on_chain: Set::new(b"oc:".to_vec()),
        }
    }

    pub fn add_edges(&mut self, fullname: Fullname, edges: Graph) {
        self.on_chain.insert(&fullname);
        for edge in edges.iter() {
            self.add_edge(&fullname, &edge);
            self.add_edge(&edge, &fullname);

            // Adding on chain edges
            if self.on_chain.contains(&edge) {
                self.add_on_chain_edge(&fullname, &edge);
            }
            self.add_on_chain_edge(&edge, &fullname);
        }
    }

    pub fn search(&self, from: Fullname, to: Fullname) -> Vec<Vec<Fullname>> {
        let mut res = HashSet::new();
        // 0
        if from == to {
            return vec![vec![from.clone()]];
        }
        // 1
        if self.get_edges(&from).contains(&to) {
            return vec![vec![from.clone(), to.clone()]];
        }
        let degree_from_1 = self.generate_first(&from);
        let degree_from_2_map = self.generate_second(&from);
        let degree_from_2: HashSet<Fullname> = degree_from_2_map.keys().cloned().collect();
        let degree_to_1 = self.generate_first(&to);
        let degree_to_2_map = self.generate_second(&to);
        let degree_to_2: HashSet<Fullname> = degree_to_2_map.keys().cloned().collect();
        // 2
        for a in degree_from_1.intersection(&degree_to_1) {
            res.insert(vec![from.clone(), a.clone(), to.clone()]);
        }
        // 3
        for a in degree_from_2.intersection(&degree_to_1).cloned() {
            for b in degree_from_2_map.get(&a).unwrap().iter() {
                res.insert(vec![from.clone(), b.clone(), a.clone(), to.clone()]);
            }
        }
        for a in degree_from_1.intersection(&degree_to_2).cloned() {
            for b in degree_to_2_map.get(&a).unwrap().iter() {
                res.insert(vec![from.clone(), a.clone(), b.clone(), to.clone()]);
            }
        }
        // 4
        for a in degree_from_2.intersection(&degree_to_2).cloned() {
            for b in degree_from_2_map.get(&a).unwrap().iter() {
                for c in degree_to_2_map.get(&a).unwrap().iter() {
                    res.insert(vec![from.clone(), b.clone(), a.clone(), c.clone(), to.clone()]);
                }
            }
        }
        res.into_iter().collect()
    }

    pub fn get_graph(&self, fullname: Fullname) -> Graph {
        self.get_edges(&fullname).to_vec()
    }

    pub fn get_on_chain_graph(&self, fullname: Fullname) -> Graph {
        self.get_on_chain_edges(&fullname).to_vec()
    }

    pub fn is_on_chain(&self, fullname: Fullname) -> bool {
        self.on_chain.contains(&fullname)
    }
}

impl Network {
    fn get_edges(&self, edge: &Fullname) -> Set<Fullname> {
        self.edges.get(&edge).unwrap_or_else(|| {
            let mut vec_id = Vec::with_capacity(10);
            vec_id.extend(b"b:");
            vec_id.extend_from_slice(&self.edges.len().to_le_bytes());
            Set::new(vec_id)
        })
    }

    fn get_on_chain_edges(&self, edge: &Fullname) -> Set<Fullname> {
        self.on_chain_edges.get(&edge).unwrap_or_else(|| {
            let mut vec_id = Vec::with_capacity(10);
            vec_id.extend(b"c:");
            vec_id.extend_from_slice(&self.on_chain_edges.len().to_le_bytes());
            Set::new(vec_id)
        })
    }

    fn generate_first(&self, a: &Fullname) -> HashSet<String> {
        self.get_edges(&a).iter().collect()
    }

    fn generate_second(&self, a: &Fullname) -> HashMap<String, Vec<String>> {
        let mut res = HashMap::new();
        for b in self.get_on_chain_edges(&a).iter() {
            let edges = self.generate_first(&b);
            for c in edges {
                if a != &c {
                    res.entry(c).or_insert_with(Vec::new).push(b.clone());
                }
            }
        }
        res
    }

    fn add_edge(&mut self, a: &Fullname, b: &Fullname) {
        let mut edges = self.get_edges(&a);
        edges.insert(&b);
        self.edges.insert(&a, &edges);
    }

    fn add_on_chain_edge(&mut self, a: &Fullname, b: &Fullname) {
        let mut edges = self.get_on_chain_edges(&a);
        edges.insert(&b);
        self.on_chain_edges.insert(&a, &edges);
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests {
    use super::*;
    use near_bindgen::MockedBlockchain;
    use near_bindgen::{testing_env, VMContext};

    fn alice() -> String {
        "alice.near".to_string()
    }
    fn bob() -> String {
        "bob.near".to_string()
    }

    fn get_context(predecessor_account_id: String) -> VMContext {
        VMContext {
            current_account_id: bob(),
            signer_account_id: predecessor_account_id.clone(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id,
            input: vec![],
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage: 10u64.pow(6),
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view: false,
            output_data_receivers: vec![],
        }
    }

    #[test]
    fn test_get_graph() {
        testing_env!(get_context(alice()));
        let mut contract = Network::new();
        assert_eq!(contract.get_graph("Potato".to_string()), Graph::new());
        contract.add_edges("Potato".to_string(), vec!["Yulik".to_string(), "Bulik".to_string()]);
        assert_eq!(
            contract.get_graph("Potato".to_string()),
            vec!["Yulik".to_string(), "Bulik".to_string()]
        );
    }

    #[test]
    fn test_get_on_chain_edges() {
        testing_env!(get_context(alice()));
        let mut contract = Network::new();
        contract.add_edges("Potato".to_string(), vec!["Yulik".to_string(), "Sasha".to_string()]);
        contract.add_edges(
            "Sasha".to_string(),
            vec!["Yulik".to_string(), "Bulik".to_string(), "Potato".to_string()],
        );
        assert_eq!(contract.get_on_chain_graph("Potato".to_string()), vec!["Sasha".to_string()]);
    }

    #[test]
    fn test_generate_second() {
        testing_env!(get_context(alice()));
        let mut contract = Network::new();
        assert_eq!(contract.get_graph("Potato".to_string()), Graph::new());
        contract.add_edges("Potato".to_string(), vec!["Yulik".to_string(), "Sasha".to_string()]);
        contract.add_edges("Sasha".to_string(), vec!["Yulik".to_string(), "Bulik".to_string()]);
        assert_eq!(contract.generate_second(&"Potato".to_string()), HashMap::new());
    }

    #[test]
    fn test_search_1st() {
        testing_env!(get_context(alice()));
        let mut contract = Network::new();
        assert_eq!(contract.get_graph("Potato".to_string()), Graph::new());
        contract.add_edges("Potato".to_string(), vec!["Yulik".to_string(), "Bulik".to_string()]);
        assert_eq!(
            contract.search("Potato".to_string(), "Bulik".to_string()),
            vec![vec!["Potato".to_string(), "Bulik".to_string()]]
        );
    }

    #[test]
    fn test_search_2nd() {
        testing_env!(get_context(alice()));
        let mut contract = Network::new();
        assert_eq!(contract.get_graph("Potato".to_string()), Graph::new());
        contract.add_edges("Potato".to_string(), vec!["Yulik".to_string(), "Bulik".to_string()]);
        contract.add_edges("Sasha".to_string(), vec!["Yulik".to_string(), "Bulik".to_string()]);
        let actual = contract
            .search("Potato".to_string(), "Sasha".to_string())
            .into_iter()
            .collect::<HashSet<_>>();
        assert_eq!(
            actual,
            vec![
                vec!["Potato".to_string(), "Bulik".to_string(), "Sasha".to_string()],
                vec!["Potato".to_string(), "Yulik".to_string(), "Sasha".to_string()],
            ]
            .into_iter()
            .collect::<HashSet<_>>()
        );
    }

    #[test]
    fn test_search_2nd_v2() {
        testing_env!(get_context(alice()));
        let mut contract = Network::new();
        assert_eq!(contract.get_graph("Potato".to_string()), Graph::new());
        contract.add_edges("Potato".to_string(), vec!["Yulik".to_string(), "Bulik".to_string()]);
        contract.add_edges("Sasha".to_string(), vec!["Yulik".to_string()]);
        assert_eq!(
            contract.search("Potato".to_string(), "Sasha".to_string()),
            vec![vec!["Potato".to_string(), "Yulik".to_string(), "Sasha".to_string()],]
        );
    }

    #[test]
    fn test_search_3nd_v2() {
        testing_env!(get_context(alice()));
        let mut contract = Network::new();
        assert_eq!(contract.get_graph("Potato".to_string()), Graph::new());
        contract.add_edges("Potato".to_string(), vec!["Yulik".to_string(), "Sasha".to_string()]);
        contract.add_edges("Sasha".to_string(), vec!["Boris".to_string(), "Potato".to_string()]);
        contract.add_edges("Boris".to_string(), vec!["Alex".to_string(), "Sasha".to_string()]);
        assert_eq!(
            contract.search("Potato".to_string(), "Alex".to_string()),
            vec![vec![
                "Potato".to_string(),
                "Sasha".to_string(),
                "Boris".to_string(),
                "Alex".to_string()
            ]]
        );
    }

    #[test]
    fn test_search_4nd() {
        testing_env!(get_context(alice()));
        let mut contract = Network::new();
        assert_eq!(contract.get_graph("Potato".to_string()), Graph::new());
        contract.add_edges("Potato".to_string(), vec!["Yulik".to_string(), "Sasha".to_string()]);
        contract.add_edges("Sasha".to_string(), vec!["Boris".to_string(), "Potato".to_string()]);
        contract.add_edges("Boris".to_string(), vec!["Alex".to_string(), "Sasha".to_string()]);
        contract.add_edges("Alex".to_string(), vec!["Carl".to_string(), "Boris".to_string()]);
        assert_eq!(
            contract.search("Potato".to_string(), "Carl".to_string()),
            vec![vec![
                "Potato".to_string(),
                "Sasha".to_string(),
                "Boris".to_string(),
                "Alex".to_string(),
                "Carl".to_string(),
            ]]
        );
    }
}
