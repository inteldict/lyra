#!/bin/sh

url_adress='http://127.0.0.1:8000/json/'

message='{"id":'${1}', "message": "Some message"}';
#echo $url_adress
#echo $message
#echo $url_adress

#curl -i -X POST -H 'Content-Type: application/json' -d ${url_adress}

#curl --location --request POST ${url_adress} \
#--header 'Content-Type: application/json' \
#--header 'Content-Type: text/plain' \
#--data-raw ${message}

curl --location --request POST ${url_adress} \
--header 'Content-Type: application/json' \
--header 'Content-Type: text/plain' \
--data-raw '{
    "id": 0,
    "message": "Some message"
}'
