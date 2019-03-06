#!/bin/bash

mkdir -p assets
wget -O assets/hightemp.txt http://www.cl.ecei.tohoku.ac.jp/nlp100/data/hightemp.txt
wget -O assets/jawiki-country.json.gz http://www.cl.ecei.tohoku.ac.jp/nlp100/data/jawiki-country.json.gz
gunzip assets/jawiki-country.json.gz
