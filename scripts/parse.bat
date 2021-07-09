@REM @echo off & setlocal
set "location=http://127.0.0.1:8000/parse"
echo %location%
curl --location --request GET %location% --header "Content-Type: application/json" -d @Frau_ParseInput.js