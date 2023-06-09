use std::fs;
use rand::Rng;
use uuid::Uuid;

pub type Node = u32;

#[derive(Debug, Clone, PartialEq)]
pub struct Contact {
    pub uuid: Uuid, // A unique identifier for each contact
    pub start: u32,
    pub end: u32,
    pub from: Node,
    pub to: Node,
    pub rate: u32,
    pub range: u32,
    pub arrival_time: u32,
    pub visited: bool,
    pub visited_nodes: Vec<Node>,
    pub predecessor: Option<Uuid>,
    pub suppressed: bool,
    pub suppressed_next_hop: Vec<Uuid>,
}

impl Contact {
    pub fn new(start: u32, end: u32, from: Node, to: Node, rate: u32, range: u32) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            start,
            end,
            from,
            to,
            rate,
            range,
            arrival_time: u32::MAX,
            visited: false,
            visited_nodes: Vec::new(),
            predecessor: None,
            suppressed: false,
            suppressed_next_hop: Vec::new(),
        }
    }

    pub fn load_from_file(file_name: &str) -> Vec<Contact> {
        let path = format!("contact_plans/{file_name}");

        let text = fs::read_to_string(&path).expect(&format!("File at path {path} not found"));
        let contact_plan = text
            .lines()
            .filter(|line| !line.starts_with("#"))
            .map(
                |line| match line.split_whitespace().collect::<Vec<_>>()[..] {
                    [start, end, from, to, rate, range] => Contact::new(
                        start.parse().unwrap(),
                        end.parse().unwrap(),
                        from.parse().unwrap(),
                        to.parse().unwrap(),
                        rate.parse().unwrap(),
                        range.parse().unwrap(),
                    ),
                    _ => panic!("Invalid line in contact plan\n{line}"),
                },
            )
            .collect::<Vec<_>>();

        println!(
            "Contact plan was successfully loaded from {} with {} number of contact",
            &path,
            contact_plan.len()
        );
        return contact_plan;
    }
    
    pub fn load_random(n_contacts: u32, n_nodes: u32) -> Vec<Contact> {
        let mut rng = rand::thread_rng();
        
        let mut contact_plan = Vec::new();
        for _ in 0..n_contacts {
            let start = rng.gen_range(0..999);
            let end = start + rng.gen_range(1..100);
            let from = rng.gen_range(1..=n_nodes);
            let mut to = rng.gen_range(1..=n_nodes);
            
            while to == from {
                to = rng.gen_range(1..n_contacts); 
            } 
            
            contact_plan.push(Contact::new(start, end, from, to, 1, 1));
        }
        
        return contact_plan;
    }
}
