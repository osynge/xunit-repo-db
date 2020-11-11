Test commands:


curl -v -X POST -d '{ "sk": "mykey", "identiifier": "identiifier2", "human_name" : "human_name" }' -H 'Content-Type: application/json' http://127.0.0.1:8888/project_add
curl -v -X POST -d '{  "sk": "f8f1208d-bf03-4daf-b919-ab18c20138b0" }' -H 'Content-Type: application/json' http://127.0.0.1:8888/project_add


curl -v -X POST -d '{  "key": "HOME", "value" : "/home/username" }' -H 'Content-Type: application/json' http://127.0.0.1:8888/keyvalue_add



curl -v -X POST -d '{  "key_value": [{  "key": "HOME", "value" : "/home/username" }] }' -H 'Content-Type: application/json' http://127.0.0.1:8888/enviroment_add


curl -v -X POST -d '{ "client_identifier" : "1"  }' -H 'Content-Type: application/json' http://127.0.0.1:8888/run_add
