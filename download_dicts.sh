#!/usr/bin/env bash

wget ftp://ftp.edrdg.org/pub/Nihongo/JMdict_e.gz
wget ftp://ftp.edrdg.org/pub/Nihongo/JMnedict.xml.gz
wget ftp://ftp.edrdg.org/pub/Nihongo/kanjidic2.xml.gz
wget https://raw.githubusercontent.com/echamudi/kanji-data-media/master/language-data/ka_data.csv

gunzip JMdict_e.gz JMnedict.xml.gz kanjidic2.xml.gz
mkdir dict_data
mv JMdict_e JMnedict.xml kanjidic2.xml ka_data.csv ./dict_data
