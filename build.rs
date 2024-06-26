fn main() {
    #[cfg(not(feature = "doc-gen"))]
    {
        println!("cargo:rerun-if-changed=src/cuda/treed.cu");

        cc::Build::new()
            .cuda(true)
            .include("src/cuda")
            .file("src/cuda/treed.cu")
            .compile("treed");
    }
}