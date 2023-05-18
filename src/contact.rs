use std::fs;
use uuid::Uuid;

pub type Node = u32;

#[derive(Debug, Clone)]
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
}
