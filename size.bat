@ECHO OFF
cargo build --release
cd ./target/release

echo[
dir | findstr smolrpg.exe
