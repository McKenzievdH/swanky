//! Benchmark code of garbling / evaluating using Nigel's circuits.

use criterion::{criterion_group, criterion_main, Criterion};
use fancy_garbling::{circuit::BinaryCircuit, classic::garble, AllWire, WireMod2};
use std::{fs::File, io::BufReader, time::Duration};

fn circuit(fname: &str) -> BinaryCircuit {
    let circ = BinaryCircuit::parse(BufReader::new(File::open(fname).unwrap())).unwrap();
    // println!("{}", fname);
    // circ.print_info().unwrap();
    circ
}
fn bench_garble_aes_binary(c: &mut Criterion) {
    let circ = circuit("circuits/AES-non-expanded.txt");
    c.bench_function("garble::aes-binary", move |bench| {
        bench.iter(|| garble::<WireMod2, _>(&circ));
    });
}

fn bench_eval_aes_binary(c: &mut Criterion) {
    let circ = circuit("circuits/AES-non-expanded.txt");
    let (en, gc) = garble::<WireMod2, _>(&circ).unwrap();
    let gb = en.encode_garbler_inputs(&vec![0u16; 128]);
    let ev = en.encode_evaluator_inputs(&vec![0u16; 128]);
    c.bench_function("eval::aes-binary", move |bench| {
        bench.iter(|| gc.eval(&circ, &gb, &ev));
    });
}

fn bench_garble_sha_1_binary(c: &mut Criterion) {
    let circ = circuit("circuits/sha-1.txt");
    c.bench_function("garble::sha-1-binary", move |bench| {
        bench.iter(|| garble::<WireMod2, _>(&circ));
    });
}

fn bench_eval_sha_1_binary(c: &mut Criterion) {
    let circ = circuit("circuits/sha-1.txt");
    let (en, gc) = garble::<WireMod2, _>(&circ).unwrap();
    let gb = en.encode_garbler_inputs(&vec![0u16; 512]);
    let ev = en.encode_evaluator_inputs(&vec![]);
    c.bench_function("eval::sha-1-binary", move |bench| {
        bench.iter(|| gc.eval(&circ, &gb, &ev));
    });
}

fn bench_garble_sha_256_binary(c: &mut Criterion) {
    let circ = circuit("circuits/sha-256.txt");
    c.bench_function("garble::sha-256-binary", move |bench| {
        bench.iter(|| garble::<WireMod2, _>(&circ));
    });
}

fn bench_eval_sha_256_binary(c: &mut Criterion) {
    let circ = circuit("circuits/sha-256.txt");
    let (en, gc) = garble::<WireMod2, _>(&circ).unwrap();
    let gb = en.encode_garbler_inputs(&vec![0u16; 512]);
    let ev = en.encode_evaluator_inputs(&vec![]);
    c.bench_function("eval::sha-256-binary", move |bench| {
        bench.iter(|| gc.eval(&circ, &gb, &ev));
    });
}

fn bench_garble_aes_arithmetic(c: &mut Criterion) {
    let circ = circuit("circuits/AES-non-expanded.txt");
    c.bench_function("garble::aes-arithmetic", move |bench| {
        bench.iter(|| garble::<AllWire, _>(&circ));
    });
}

fn bench_eval_aes_arithmetic(c: &mut Criterion) {
    let circ = circuit("circuits/AES-non-expanded.txt");
    let (en, gc) = garble::<AllWire, _>(&circ).unwrap();
    let gb = en.encode_garbler_inputs(&vec![0u16; 128]);
    let ev = en.encode_evaluator_inputs(&vec![0u16; 128]);
    c.bench_function("eval::aes-arithmetic", move |bench| {
        bench.iter(|| gc.eval(&circ, &gb, &ev));
    });
}

fn bench_garble_sha_1_arithmetic(c: &mut Criterion) {
    let circ = circuit("circuits/sha-1.txt");
    c.bench_function("garble::sha-1-arithmetic", move |bench| {
        bench.iter(|| garble::<AllWire, _>(&circ));
    });
}

fn bench_eval_sha_1_arithmetic(c: &mut Criterion) {
    let circ = circuit("circuits/sha-1.txt");
    let (en, gc) = garble::<AllWire, _>(&circ).unwrap();
    let gb = en.encode_garbler_inputs(&vec![0u16; 512]);
    let ev = en.encode_evaluator_inputs(&vec![]);
    c.bench_function("eval::sha-1-arithmetic", move |bench| {
        bench.iter(|| gc.eval(&circ, &gb, &ev));
    });
}

fn bench_garble_sha_256_arithmetic(c: &mut Criterion) {
    let circ = circuit("circuits/sha-256.txt");
    c.bench_function("garble::sha-256-arithmetic", move |bench| {
        bench.iter(|| garble::<AllWire, _>(&circ));
    });
}

fn bench_eval_sha_256_arithmetic(c: &mut Criterion) {
    let circ = circuit("circuits/sha-256.txt");
    let (en, gc) = garble::<AllWire, _>(&circ).unwrap();
    let gb = en.encode_garbler_inputs(&vec![0u16; 512]);
    let ev = en.encode_evaluator_inputs(&vec![]);
    c.bench_function("eval::sha-256-arithmetic", move |bench| {
        bench.iter(|| gc.eval(&circ, &gb, &ev));
    });
}

fn bench_garble_c17(c: &mut Criterion) {
    let circ = circuit("../../TFHE_repo/Garbled/circuits/ISCAS89/c17.txt");
    c.bench_function("garble::c17", move |bench| {
        bench.iter(|| garble::<WireMod2, _>(&circ));
    });
}

fn bench_eval_c17(c: &mut Criterion) {
    let circ = circuit("../../TFHE_repo/Garbled/circuits/ISCAS89/c17.txt");
    let (en, gc) = garble::<WireMod2, _>(&circ).unwrap();
    let gb = en.encode_garbler_inputs(&vec![0u16; 5]);
    let ev = en.encode_evaluator_inputs(&vec![]);
    c.bench_function("eval::c17", move |bench| {
        bench.iter(|| gc.eval(&circ, &gb, &ev));
    });
}

fn bench_garble_k6(c: &mut Criterion) {
    let circ = circuit("../../TFHE_repo/Garbled/circuits/strm_join/k6_v1_n1.txt");
    c.bench_function("garble::strm_join_k6", move |bench| {
        bench.iter(|| garble::<WireMod2, _>(&circ));
    });
}
fn bench_eval_k6(c: &mut Criterion) {
    let circ = circuit("../../TFHE_repo/Garbled/circuits/strm_join/k6_v1_n1.txt");
    let (en, gc) = garble::<WireMod2, _>(&circ).unwrap();
    let gb = en.encode_garbler_inputs(&vec![0u16; 23]);
    let ev = en.encode_evaluator_inputs(&vec![]);
    c.bench_function("eval::strm_join_k6", move |bench| {
        bench.iter(|| gc.eval(&circ, &gb, &ev));
    });
}
fn bench_garble_k7(c: &mut Criterion) {
    let circ = circuit("../../TFHE_repo/Garbled/circuits/strm_join/k7_v1_n2.txt");
    c.bench_function("garble::strm_join_k7", move |bench| {
        bench.iter(|| garble::<WireMod2, _>(&circ));
    });
}
fn bench_eval_k7(c: &mut Criterion) {
    let circ = circuit("../../TFHE_repo/Garbled/circuits/strm_join/k7_v1_n2.txt");
    let (en, gc) = garble::<WireMod2, _>(&circ).unwrap();
    let gb = en.encode_garbler_inputs(&vec![0u16; 52]);
    let ev = en.encode_evaluator_inputs(&vec![]);
    c.bench_function("eval::strm_join_k7", move |bench| {
        bench.iter(|| gc.eval(&circ, &gb, &ev));
    });
}
fn bench_garble_k8(c: &mut Criterion) {
    let circ = circuit("../../TFHE_repo/Garbled/circuits/strm_join/k8_v1_n3.txt");
    c.bench_function("garble::strm_join_k8", move |bench| {
        bench.iter(|| garble::<WireMod2, _>(&circ));
    });
}
fn bench_eval_k8(c: &mut Criterion) {
    let circ = circuit("../../TFHE_repo/Garbled/circuits/strm_join/k8_v1_n3.txt");
    let (en, gc) = garble::<WireMod2, _>(&circ).unwrap();
    let gb = en.encode_garbler_inputs(&vec![0u16; 87]);
    let ev = en.encode_evaluator_inputs(&vec![]);
    c.bench_function("eval::strm_join_k8", move |bench| {
        bench.iter(|| gc.eval(&circ, &gb, &ev));
    });
}
fn bench_garble_k9(c: &mut Criterion) {
    let circ = circuit("../../TFHE_repo/Garbled/circuits/strm_join/k9_v1_n6.txt");
    c.bench_function("garble::strm_join_k9", move |bench| {
        bench.iter(|| garble::<WireMod2, _>(&circ));
    });
}
fn bench_eval_k9(c: &mut Criterion) {
    let circ = circuit("../../TFHE_repo/Garbled/circuits/strm_join/k9_v1_n6.txt");
    let (en, gc) = garble::<WireMod2, _>(&circ).unwrap();
    let gb = en.encode_garbler_inputs(&vec![0u16; 188]);
    let ev = en.encode_evaluator_inputs(&vec![]);
    c.bench_function("eval::strm_join_k9", move |bench| {
        bench.iter(|| gc.eval(&circ, &gb, &ev));
    });
}
fn bench_garble_k10(c: &mut Criterion) {
    let circ = circuit("../../TFHE_repo/Garbled/circuits/strm_join/k10_v1_n11.txt");
    c.bench_function("garble::strm_join_k10", move |bench| {
        bench.iter(|| garble::<WireMod2, _>(&circ));
    });
}
fn bench_eval_k10(c: &mut Criterion) {
    let circ = circuit("../../TFHE_repo/Garbled/circuits/strm_join/k10_v1_n11.txt");
    let (en, gc) = garble::<WireMod2, _>(&circ).unwrap();
    let gb = en.encode_garbler_inputs(&vec![0u16; 373]);
    let ev = en.encode_evaluator_inputs(&vec![]);
    c.bench_function("eval::strm_join_k10", move |bench| {
        bench.iter(|| gc.eval(&circ, &gb, &ev));
    });
}

criterion_group! {
    name = parsing;
    config = Criterion::default().warm_up_time(Duration::from_millis(100));
    targets = bench_garble_k6, bench_eval_k6, bench_garble_k7, bench_eval_k7, bench_garble_k8, bench_eval_k8,
    bench_garble_k9, bench_eval_k9, bench_garble_k10, bench_eval_k10
}

criterion_main!(parsing);
