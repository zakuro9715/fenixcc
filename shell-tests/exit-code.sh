robo compile-example expr.c
./a.out

actual="$?"
expected=7

if [ "$expected" = "$actual" ]
then
  echo "OK"
else
  echo ERROR: expected $expected, actual $actual
#  exit 1
fi
