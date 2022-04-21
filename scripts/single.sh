FILE=$1
SOURCE="./src"
BIN="./bin/"
if [ -f $SOURCE/$FILE.rs ]; then
	echo "compiling  $SOURCE/$FILE.rs"
	rustc $SOURCE/$FILE.rs --out-dir $BIN
fi
