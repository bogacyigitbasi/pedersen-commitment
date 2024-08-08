use rand::Rng;

/// we will pick two generators g and h
/// ideally from a curve but its okay to pick some random numbers
/// to achieve same goal
/// then we will need another random number as blinding factor
/// and calculate g^m.h^r where m is our message to hide
///
///

// calculates g^x mod p
fn modular_exp(g: &u64, x: &u64, p: &u64) -> u64 {
    let mut result = 1;
    let mut g = g % p;
    let mut x = *x;

    while x > 0 {
        // if x is odd, multiply result
        if x % 2 == 1 {
            result = result * g;
        }
        // x = x/2
        x = x >> 1;
        // g to g^2
        g = g * g % p;
    }
    result % p
}

struct PedersenCommitment {
    g: u64,
    h: u64,
    p: u64,
}

impl PedersenCommitment {
    fn new(g: u64, h: u64, p: u64) -> Self {
        PedersenCommitment { g: g, h: h, p: p }
    }
    // committing to the value
    fn commit(&self, message: u64, blinding: u64) -> u64 {
        // calculate g^m and we will  have initated g and p already
        let gm = modular_exp(&self.g, &message, &self.p);
        // calculate h^r
        let hr = modular_exp(&self.h, &blinding, &self.p);

        (gm * hr) % &self.p
    }
    // reveal the m and r and check the committed value indeed the one received before
    fn verify(&self, commitment: u64, message: u64, blinding: u64) -> bool {
        if (commitment == self.commit(message, blinding)) {
            return true;
        }
        false
    }
}
//lets pick the modulus, secret, g, h and run the protocol
fn main() {
    // random number generator
    let mut rng = rand::thread_rng();
    // global param generator g (base)
    const g: u64 = 5;
    // global param generator h (base)
    const h: u64 = 7;
    // prime modular value to work in finite field
    const p: u64 = 101;
    // message
    let m = 9;
    // blinding factor
    let r = rng.gen_range(1..1000);

    let pedersen_commitment = PedersenCommitment::new(g, h, p);
    // calculate the commitment
    let commit = pedersen_commitment.commit(m, r);
    println!("Pedersen commitment : r = {}", commit);

    // verify the commitment after some processes
    println!(
        "Is commitment verified: {}",
        pedersen_commitment.verify(commit, m, r)
    );
}
