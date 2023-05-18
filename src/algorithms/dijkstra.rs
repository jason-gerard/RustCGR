use crate::contact::{Contact, Node};
use crate::route::Route;
use std::cmp;
use std::collections::HashMap;
use uuid::Uuid;

pub fn cgr_dijkstra(
    root_contact: &mut Contact,
    destination: Node,
    mut contact_plan: Vec<Contact>,
) -> Option<Route> {
    let mut adjacency_list = HashMap::<Node, Vec<Uuid>>::new();
    contact_plan.iter().for_each(|contact| {
        adjacency_list
            .entry(contact.from)
            .or_insert(Vec::new())
            .push(contact.uuid)
    });

    // dbg!(&adjacency_list);

    let root_contact_id = root_contact.uuid.clone();
    // Map containing all the details of the contacts, this is so we don't have interior mutability
    // issues, all other data structures just contain the UUID for a specific contact
    let mut contact_details = HashMap::<Uuid, &mut Contact>::new();
    contact_details.insert(root_contact.uuid, root_contact);
    contact_plan.iter_mut().for_each(|contact| {
        contact_details.insert(contact.uuid, contact);
    });

    // dbg!(&contact_details);

    let mut final_contact = None;
    let mut earliest_final_arrival_time = u32::MAX;

    contact_details
        .entry(root_contact_id)
        .and_modify(|contact| contact.visited_nodes.push(contact.to));

    let mut current_contact_id = root_contact_id;
    loop {
        let next_ids = adjacency_list
            .entry(contact_details.get(&current_contact_id)?.to)
            .or_default();
        for next_contact_id in next_ids.clone().iter() {
            let current_contact = contact_details.get(&current_contact_id)?;
            let mut next_contact = (*contact_details.get(&next_contact_id)?).clone();

            if next_contact.visited
                || current_contact.visited_nodes.contains(&next_contact.to)
                || next_contact.end <= current_contact.arrival_time
                || (current_contact.from == next_contact.to
                    && current_contact.to == next_contact.from)
                || next_contact.suppressed
                || current_contact.suppressed_next_hop.contains(&next_contact_id)
            {
                continue;
            }

            let arrival_time =
                cmp::max(current_contact.arrival_time, next_contact.start) + next_contact.range;
            if arrival_time <= next_contact.arrival_time {
                next_contact.arrival_time = arrival_time;
                next_contact.predecessor = Some(current_contact_id);
                next_contact.visited_nodes = current_contact.visited_nodes.clone();
                next_contact.visited_nodes.push(next_contact.to);

                if next_contact.to == destination
                    && next_contact.arrival_time < earliest_final_arrival_time
                {
                    earliest_final_arrival_time = next_contact.arrival_time;
                    final_contact = Some(next_contact_id.clone());
                }
            }

            contact_details
                .entry(next_contact_id.clone())
                .and_modify(|contact| **contact = next_contact);
        }

        contact_details
            .entry(current_contact_id)
            .and_modify(|contact| contact.visited = true);

        let mut earliest_arrival_time = u32::MAX;
        let mut next_contact_id = None;

        for contact in contact_details.values() {
            if contact.visited || contact.arrival_time > earliest_final_arrival_time {
                continue;
            }

            if contact.arrival_time < earliest_arrival_time {
                earliest_arrival_time = contact.arrival_time;
                next_contact_id = Some(contact.uuid);
            }
        }

        current_contact_id = match next_contact_id {
            Some(id) => id,
            _ => break, // No next contact in route found
        };
    }

    return match final_contact {
        Some(final_contact_id) => {
            let mut hops = Vec::<Contact>::new();
            let mut contact_id = final_contact_id;

            while contact_id != root_contact_id {
                let contact = contact_details.get(&contact_id)?;

                hops.insert(0, (*contact).clone());
                contact_id = contact.predecessor?;
            }

            Some(Route::new(hops))
        }
        _ => None,
    };
}
