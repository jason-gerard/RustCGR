use rust_cgr::algorithms::dijkstra::cgr_dijkstra;
use rust_cgr::algorithms::yen::cgr_yen;
use rust_cgr::contact::Contact;

fn main() {
    // let plan = Contact::load_from_file("sample_plan.txt");
    let plan = Contact::load_random(1000, 10);

    let source = 1;
    let destination = 5;
    let current_time = 0;

    let mut root_contact = Contact::new(0, u32::MAX, source, source, 100, 0);
    root_contact.arrival_time = current_time;

    // let route = cgr_dijkstra(&mut root_contact, destination, plan);
    // if let Some(route) = route {
    //     dbg!(route);
    // } else {
    //     println!("No route was generated");
    // }

    let routes = cgr_yen(&mut root_contact, destination, plan, 5);
    
    println!("route count: {}", routes.len());
    for route in routes.iter() {
        println!("route with arrival time {}", route.hops.iter().map(|contact| contact.arrival_time).sum::<u32>());
        print!("{}::{}::{}::{}, ", route.hops.get(0).unwrap().from, route.hops.get(0).unwrap().rate, route.hops.get(0).unwrap().start, route.hops.get(0).unwrap().end);
        route.hops.iter().for_each(|contact| print!("{}::{}::{}::{}, ", contact.to, contact.rate, contact.start, contact.end));
        println!();
    }
}
