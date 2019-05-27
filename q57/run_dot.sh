#!/bin/bash

for f in out/*.dot; do
	dot -Kdot -Tpng $f -o${f%.dot}.png
done
