use crate::algorithms::dijkstra::cgr_dijkstra;
use crate::contact::{Contact, Node};
use crate::route::Route;

pub fn cgr_yen(
    root_contact: &mut Contact,
    destination: Node,
    mut contact_plan: Vec<Contact>,
    top_k: u32,
) -> Vec<Route> {
    let mut routes = Vec::<Route>::new();
    let mut potential_routes = Vec::<Route>::new();
    
    let route = cgr_dijkstra(&mut root_contact.clone(), destination, contact_plan.clone());
    match route {
        Some(mut route) => {
            // Add the root contact as the first hop in the best route returned from dijkstra
            route.hops.insert(0, root_contact.clone());
            routes.push(route);
        },
        None => return routes,
    };
    
    for _ in 0..top_k-1 {
        let last_route = routes.last().unwrap();
        let num_hops = last_route.hops.len();
        
        for spur_contact in last_route.hops.iter().take(num_hops - 1) {
            let spur_contact_index = last_route.hops.iter().position(|contact| contact == spur_contact).unwrap();
            
            let mut hops = Vec::new();
            hops.push(last_route.hops.first().unwrap().clone());
            hops.append(&mut last_route.hops[1..spur_contact_index+1].to_vec());
            
            let mut root_path = Route::new(hops);
            root_path.hops.iter_mut().for_each(|contact| contact.suppressed = true);
            
            let mut spur_contact_mut = spur_contact.clone();
            for route in routes.iter() {
                if root_path.hops == route.hops.clone().into_iter().take(root_path.hops.len()).collect::<Vec<_>>() && !spur_contact.suppressed_next_hop.contains(&route.hops.get(root_path.hops.len()).unwrap().uuid) {
                    spur_contact_mut.suppressed_next_hop.push(route.hops.get(root_path.hops.len()).unwrap().uuid.clone());
                }
            }
            
            for hop in root_path.hops.iter() {
                spur_contact_mut.visited_nodes.push(hop.to);
            }
            
            let spur_path = cgr_dijkstra(&mut spur_contact_mut, destination, contact_plan.clone());
            if let Some(spur_path) = spur_path {
                let mut hops = Vec::new();
                hops.append(&mut root_path.hops.clone());
                hops.append(&mut spur_path.hops.clone());
                
                let route = Route::new(hops);
                potential_routes.push(route);
            }
        }
        
        if potential_routes.is_empty() {
            break
        }
        
        potential_routes.sort_by_key(|route| route.hops.iter().map(|contact| contact.arrival_time).sum::<u32>());
        let best_route = potential_routes.pop().unwrap();
        routes.push(best_route);
    }
    
    // Post processing to remove root contact from routes
    for route in routes.iter_mut() {
        route.hops.remove(0);
    }
    
    return routes;
}