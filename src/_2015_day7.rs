use std::collections::HashMap;

pub fn solve(input: &str){
    let mut circuit = HashMap::new();

    let mut instructions = input.lines().collect::<Vec<&str>>();

    while instructions.len() > 0 {
        let mut unprocessed :Vec<&str> = Vec::new();

        instructions.iter().for_each(|l| {
            //println!("{}", l);
            let mut parts = l.split(" -> ");
            let input = parts.next().unwrap().trim();
            let receiver = parts.next().unwrap();

            let output = get_output(input, &circuit);

            match output {
                Ok(o) => {
                    circuit.insert(receiver.to_string(), o);
                },
                Err(_) => {unprocessed.push(l);}
            };
        });

        //println!("Still need to process {} wires.. ", unprocessed.len());
        instructions = unprocessed;
    }

    //println!("{:?}", circuit);
    println!("A: {}", circuit.get("a").unwrap())
}

fn get_output(input : &str, circuit : &HashMap<String, u16>) -> Result<u16, WireNotActivated> {
    //println!("{}", input);
    match input.parse::<u16>() {
        Ok(i) => return Ok(i),
        Err(_) => {
            if input.starts_with("NOT") {
                let tmp =  input.replace("NOT", "");
                let wire = tmp.trim();

                return match wire.parse::<u16>() {
                    Ok(v) => Ok(v),
                    Err(_) => match circuit.get(wire) {
                        Some(v) => Ok(!v.clone()),
                        None => Err(WireNotActivated)
                    }
                }
            }

            if input.contains("AND") {
                let mut wires = input.split(" AND ");
                let wire_a = wires.next().unwrap().trim();
                let wire_b = wires.next().unwrap().trim();

                let wire_a_sig = match wire_a.parse::<u16>() {
                    Ok(v) => v,
                    Err(_) => {
                        match circuit.get(wire_a) {
                            Some(wire_a_sig) => wire_a_sig.clone(),
                            None => {
                                return Err(WireNotActivated);
                            }
                        }
                    }
                };

                let wire_b_sig = match wire_b.parse::<u16>() {
                    Ok(v) => v,
                    Err(_) => {
                        match circuit.get(wire_b) {
                            Some(wire_b_sig) => wire_b_sig.clone(),
                            None => {
                                return Err(WireNotActivated);
                            }
                        }
                    }
                };

                return Ok(wire_a_sig & wire_b_sig);
            }

            if input.contains("LSHIFT") {
                let mut parts = input.split(" LSHIFT ");
                let wire = parts.next().unwrap().trim();
                let shift_val = parts.next().unwrap().trim().parse::<u16>().unwrap();

                let wire_sig = match wire.parse::<u16>() {
                    Ok(v) => v,
                    Err(_) => match circuit.get(wire) {
                        Some(v) => v.clone(),
                        None => {
                            return Err(WireNotActivated);
                        }
                    }
                };

                return Ok(wire_sig << shift_val);
            }

            if input.contains("RSHIFT") {
                let mut parts = input.split(" RSHIFT ");
                let wire = parts.next().unwrap().trim();
                let shift_val = parts.next().unwrap().trim().parse::<u16>().unwrap();

                let wire_sig = match wire.parse::<u16>() {
                    Ok(v) => v,
                    Err(_) => match circuit.get(wire) {
                        Some(v) => v.clone(),
                        None => {
                            return Err(WireNotActivated);
                        }
                    }
                };

                return Ok(wire_sig >> shift_val);
            }

            if input.contains(" OR ") {
                let mut wires = input.split(" OR ");

                let wire_a = wires.next().unwrap().trim();
                let wire_b = wires.next().unwrap().trim();

                let wire_a_sig = match wire_a.parse::<u16>() {
                    Ok(v) => v,
                    Err(_) => {
                        match circuit.get(wire_a) {
                            Some(wire_a_sig) => wire_a_sig.clone(),
                            None => {
                                return Err(WireNotActivated);
                            }
                        }
                    }
                };

                let wire_b_sig = match wire_b.parse::<u16>() {
                    Ok(v) => v,
                    Err(_) => {
                        match circuit.get(wire_b) {
                            Some(wire_b_sig) => wire_b_sig.clone(),
                            None => {
                                return Err(WireNotActivated);
                            }
                        }
                    }
                };

                return Ok(wire_a_sig | wire_b_sig);
            }

            //Wire gets signal directly from another wire:
            return match circuit.get(input) {
                Some(v) => Ok(v.clone()),
                None => Err(WireNotActivated)
            }
        }
    };
}

#[derive(Debug, Clone)]
struct WireNotActivated;