use fancy_garbling::{
    circuit::{BinaryCircuit as Circuit, EvaluableCircuit},
    twopac::semihonest::{Evaluator, Garbler},
    FancyInput, WireMod2,
};
use ocelot::ot::{AlszReceiver as OtReceiver, AlszSender as OtSender};
use scuttlebutt::{unix_channel_pair, AesRng, UnixChannel};
use std::{fs::File, io::BufReader, io::BufRead, time::SystemTime};

use std::env;
use std::str::FromStr;

fn circuit(fname: &str) -> Circuit {
    println!("* Circuit: {}", fname);
    Circuit::parse(BufReader::new(File::open(fname).unwrap())).unwrap()
}

fn run_circuit(circ: &mut Circuit, gb_inputs: Vec<u16>, ev_inputs: Vec<u16>) {
    let circ_ = circ.clone();
    let (sender, receiver) = unix_channel_pair();
    let n_gb_inputs = gb_inputs.len();
    let n_ev_inputs = ev_inputs.len();
    let total = SystemTime::now();
    let handle = std::thread::spawn(move || {
        let rng = AesRng::new();
        let start = SystemTime::now();
        let mut gb = Garbler::<UnixChannel, AesRng, OtSender, WireMod2>::new(sender, rng).unwrap();
        println!(
            "Garbler :: Initialization: {} ms",
            start.elapsed().unwrap().as_millis()
        );
        let start = SystemTime::now();
        let xs = gb.encode_many(&gb_inputs, &vec![2; n_gb_inputs]).unwrap();
        let ys = gb.receive_many(&vec![2; n_ev_inputs]).unwrap();
        println!(
            "Garbler :: Encoding inputs: {} ns",
            start.elapsed().unwrap().as_nanos()
        );
        let start = SystemTime::now();
        circ_.eval(&mut gb, &xs, &ys).unwrap();
        println!(
            "Garbler :: Circuit garbling: {} ns",
            start.elapsed().unwrap().as_nanos()
        );
    });
    let rng = AesRng::new();
    let start = SystemTime::now();
    let mut ev =
        Evaluator::<UnixChannel, AesRng, OtReceiver, WireMod2>::new(receiver, rng).unwrap();
    println!(
        "Evaluator :: Initialization: {} ms",
        start.elapsed().unwrap().as_millis()
    );
    let start = SystemTime::now();
    let xs = ev.receive_many(&vec![2; n_gb_inputs]).unwrap();
    let ys = ev.encode_many(&ev_inputs, &vec![2; n_ev_inputs]).unwrap();
    println!(
        "Evaluator :: Encoding inputs: {} ns",
        start.elapsed().unwrap().as_nanos()
    );
    let start = SystemTime::now();
    circ.eval(&mut ev, &xs, &ys).unwrap();
    println!(
        "Evaluator :: Circuit evaluation: {} ns",
        start.elapsed().unwrap().as_nanos()
    );
    handle.join().unwrap();
    println!("Total: {} ms", total.elapsed().unwrap().as_millis());
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let circ_fname = &args[1];

    // Open the circuit file to get inputs
    let reader = BufReader::new(File::open(circ_fname).unwrap());
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>().unwrap();
    let tokens: Vec<&str> = lines[0].split(" ").collect();
    let gate_count: usize = usize::from_str(tokens[0]).unwrap();
    let wire_count: usize = usize::from_str(tokens[1]).unwrap();
    println!("Found {} gates & {} wires", gate_count, wire_count);

    let tokens: Vec<&str> = lines[1].split(" ").collect();
    let gb_input_count: usize = usize::from_str(tokens[0]).unwrap();
    let ev_input_count: usize = usize::from_str(tokens[1]).unwrap();
    let output_count: usize = usize::from_str(tokens[2]).unwrap();
    println!("Found {} gb_inputs & {} ev_inputs -> {} outputs", gb_input_count, ev_input_count, output_count);
    // Run the circuit
    let mut circ = circuit(circ_fname);
    run_circuit(&mut circ, vec![0; gb_input_count], vec![0; ev_input_count]);
}
