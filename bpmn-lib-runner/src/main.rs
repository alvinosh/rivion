use std::fs::read_to_string;
use std::io::Write;
use std::io::{stdin, stdout};

use bpmn_engine::{engine::BPMNEngine, parse_xml};

fn main() {
    let input = read_to_string("./resources/pizza.bpmn").unwrap();
    let definitions = parse_xml(&input);
    let mut engine = BPMNEngine::new(definitions, None);

    let mut command = String::new();

    loop {
        command.clear();

        let opts = engine.options();
        println!("{:?}", opts);

        for (idx, opt) in opts.iter().enumerate() {
            let (id, name) = opt.get_id_name();
            println!(
                "    IDX: {:small$} ID : {:width$} Name : {}",
                idx,
                id.unwrap(),
                name.unwrap(),
                width = 12,
                small = 3
            )
        }

        print!("  > ");
        stdout().flush().unwrap();
        stdin().read_line(&mut command).unwrap();

        match command.trim().parse::<usize>() {
            Ok(val) => {
                let (id, _) = opts[val].get_id_name();
                engine.next(id.unwrap())
            }
            Err(e) => {
                eprintln!("{}", e);
            }
        }
    }
}
