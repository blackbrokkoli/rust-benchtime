mod benchmark;

fn main() {
    let mut bm = benchmark::BenchmarkStorage::init();
    bm.add("Program Starts!");
    println!("Hello World!");

    bm.add("Calculation of Prime Number starts");
    primes::factors(3534651324735435371);
    bm.add("Calculation of Prime Number ends");

    bm.add("Program Ends!");
    bm.render();
}