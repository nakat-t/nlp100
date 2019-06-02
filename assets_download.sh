#!/bin/bash

mkdir -p assets
wget -O assets/hightemp.txt http://www.cl.ecei.tohoku.ac.jp/nlp100/data/hightemp.txt
wget -O assets/jawiki-country.json.gz http://www.cl.ecei.tohoku.ac.jp/nlp100/data/jawiki-country.json.gz
gunzip assets/jawiki-country.json.gz
wget -O assets/neko.txt http://www.cl.ecei.tohoku.ac.jp/nlp100/data/neko.txt
mecab < assets/neko.txt > assets/neko.txt.mecab
cabocha -f 1 < assets/neko.txt > assets/neko.txt.cabocha
wget -O assets/nlp.txt http://www.cl.ecei.tohoku.ac.jp/nlp100/data/nlp.txt
wget -O assets/stanford-corenlp-full-2018-10-05.zip http://nlp.stanford.edu/software/stanford-corenlp-full-2018-10-05.zip
wget -O assets/stanford-english-corenlp-2018-10-05-models.jar http://nlp.stanford.edu/software/stanford-english-corenlp-2018-10-05-models.jar
unzip -d assets/assets/stanford-corenlp-full-2018-10-05.zip
mv assets/stanford-english-corenlp-2018-10-05-models.jar assets/stanford-corenlp-full-2018-10-05
cd assets/stanford-corenlp-full-2018-10-05
./corenlp.sh -file ../nlp.txt
mv nlp.txt.xml ../nlp.txt.xml
./corenlp.sh -file ../nlp.txt -annotators tokenize,ssplit,pos,lemma,ner,parse,dcoref
mv nlp.txt.xml ../nlp.txt.with_parse.xml
cd ../..
wget -O assets/artist.json.gz http://www.cl.ecei.tohoku.ac.jp/nlp100/data/artist.json.gz
gunzip assets/artist.json.gz
