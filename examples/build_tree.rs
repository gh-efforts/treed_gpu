use std::path::PathBuf;

fn main() {
    let mut args= std::env::args();
    args.next();

    let in_path = args.next().expect("must need in file path");

    let out_path = PathBuf::from(format!("{}_treed", in_path));
    let in_path = PathBuf::from(in_path);

    treed_gpu::build_treed(&in_path, &out_path).unwrap();
}