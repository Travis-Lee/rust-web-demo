cargo clean 

cargo build --release  

echo "Final Build"

cp -rf ./target/release/rust-web-demo ./

echo "Final Copy Bin File"
