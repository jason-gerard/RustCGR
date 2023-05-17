use std::fs;

fn main() {
    let plan = Contact::load_from_file("sample_plan.txt");
}

struct Contact {
    start: i32,
    end: i32,
    from: u32,
    to: u32,
    rate: u32,
    range: u32,
}

impl Contact {
    fn new(
        start: i32,
        end: i32,
        from: u32,
        to: u32,
        rate: u32,
        range: u32,
    ) -> Self {
        Self {
            start,
            end,
            from,
            to,
            rate,
            range,
        }
    }
    
    fn load_from_file(file_name: &str) -> Vec<Contact> {
        let path = format!("contact_plans/{file_name}");
        
        let text = fs::read_to_string(&path).expect(&format!("File at path {path} not found"));
        let contact_plan = text.lines()
            .filter(|line| !line.starts_with("#"))
            .map(|line| match line.split_whitespace().collect::<Vec<_>>()[..] {
                [start, end, from, to, rate, range] => Contact::new(
                    start.parse().unwrap(),
                    end.parse().unwrap(),
                    from.parse().unwrap(),
                    to.parse().unwrap(),
                    rate.parse().unwrap(),
                    range.parse().unwrap(),
                ),
                _ => panic!("Invalid line in contact plan\n{line}")
            })
            .collect::<Vec<_>>();
        
        println!("Contact plan was successfully loaded from {} with {} number of contact", &path, contact_plan.len());
        return contact_plan;
    }
}