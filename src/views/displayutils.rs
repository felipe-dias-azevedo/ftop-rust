use crate::monitor::MonitorData;


pub fn start_message(system: &String) {
    println!("Starting monitoring on {}...", system);
}

pub fn list_components(components: Vec<MonitorData>) {
    for component in components {
        println!("---------- {} ----------", component.kind);
        for c in component.data {
            println!("{} - {}", c.id, c.name)
        }
    }
}