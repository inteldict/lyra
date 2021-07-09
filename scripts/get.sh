#!/bin/bash

url_adress='http://127.0.0.1:8000/json/'${1} 
#echo $url_adress

curl --location --request GET ${url_adress} \
--header 'Content-Type: application/json' \
--header 'Content-Type: text/plain' \
--data-raw '{
    "id": 0,
    "message": "Some message"
}'


#curl -i -X GET http://127.0.0.1:8000/json/${1}
