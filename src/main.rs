use rust_cgr::algorithms::dijkstra::cgr_dijkstra;
use rust_cgr::contact::Contact;

fn main() {
    let plan = Contact::load_from_file("sample_plan.txt");

    let source = 1;
    let destination = 5;
    let current_time = 0;

    let mut root_contact = Contact::new(0, u32::MAX, source, source, 100, 0);
    root_contact.arrival_time = current_time;

    let route = cgr_dijkstra(&mut root_contact, destination, plan);
    if let Some(route) = route {
        dbg!(route);
    } else {
        println!("No route was generated");
    }
}
