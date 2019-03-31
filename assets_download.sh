#!/bin/bash

mkdir -p assets
wget -O assets/hightemp.txt http://www.cl.ecei.tohoku.ac.jp/nlp100/data/hightemp.txt
wget -O assets/jawiki-country.json.gz http://www.cl.ecei.tohoku.ac.jp/nlp100/data/jawiki-country.json.gz
gunzip assets/jawiki-country.json.gz
wget -O assets/neko.txt http://www.cl.ecei.tohoku.ac.jp/nlp100/data/neko.txt
mecab < assets/neko.txt > assets/neko.txt.mecab
cabocha -f 1 < assets/neko.txt > assets/neko.txt.cabocha
