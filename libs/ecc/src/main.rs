mod generate_keypair;
mod verify_sig;
mod verify_sig_w_v;

fn main() {
    generate_keypair::main();
    verify_sig::main();
    verify_sig_w_v::main();
}
