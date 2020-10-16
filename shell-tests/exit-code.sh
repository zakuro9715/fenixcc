cargo run 42 > tmp.s
cc tmp.s
./a.out

actual="$?"
expected=42

rm tmp.s a.out

if [ "$expected" = "$actual" ]
then
  echo "OK"
else
  echo ERROR: expected $expected, actual $actual
#  exit 1
fi
