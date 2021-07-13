#!/bin/bash

#js=\'$(cat ./Frau_ParseInput.js)\'
#echo $js
url_adress='http://127.0.0.1:8000/parse'
#echo $url_adress

curl -i \
--location --request GET ${url_adress} \
--header 'Content-Type: application/json' \
--data-binary "@./Frau_ParseInput.js"
