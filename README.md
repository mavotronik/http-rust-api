# http-rust-api
Simple API written on Rust
---
По умолчанию Api запускается на порту `8080` и сохраняет данныe в файл `data.json`. Настраивается в проекте. 
---
Api реализут 6 эндпоинтов при помощи которых можно взаимодействовать с данными: 
- Получение некоторой информации о системе: ```curl localhost:8080/system```
- Получение обьектов: ```curl localhost:8080/items```
- Добавление обьектов: ```curl -G "localhost:8080/add" --data-urlencode "id=1" --data-urlencode "name=tst" --data-urlencode "status=active" --data-urlencode "stream_url=http://test.ts"```
- Изменение обьектов: ```curl -G "localhost:8080/update" --data-urlencode "id=1" --data-urlencode "name=Пицца" --data-urlencode "status=inactive" --data-urlencode "stream_url=http://test.ts"```
- Удаление обьектов: ```curl -G "localhost:8080/delete" --data-urlencode "id=1"```