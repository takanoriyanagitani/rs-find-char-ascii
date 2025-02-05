#!/bin/sh

echo helo | ENV_CHAR_TO_FIND_NEEDLE=N     ./rs-find-char-ascii /dev/stdin
echo helo | ENV_CHAR_TO_FIND_NEEDLE=l     ./rs-find-char-ascii /dev/stdin
echo '	' | ENV_CHAR_TO_FIND_NEEDLE='	' ./rs-find-char-ascii -
