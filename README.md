# netology_tt

## Задание
Написать на Rust RESTful веб-сервис. Сервис должен возвращать прогноз погоды (температуру) в заданном городе:
- на заданный день (текущий или следующие, с историческими данными работать не нужно)
- на ближайшую неделю (коллекция из 5 дней)
  В качестве источника данных нужно выбрать пару сторонних веб-сервисов (с открытым API). Нужно вычислить среднее значение по данным из обоих.

В реализации при выборе тех или иных решений, стоит ориентироваться на те, что вы предпочли бы использовать в реальном приложении.
Не обязательно но будет плюсом, если вы:
- покроете код юнит и функциональными тестами
- будете отдавать информативные ошибки на запросы API
- контенеризуете сервис (мы используем Docker и Kubernetes)

## Build and run
```shell
env RUST_LOG=debug OPENWEATHERMAP_KEY=key WEATHERAPICOM_KEY=key cargo test
env RUST_LOG=debug OPENWEATHERMAP_KEY=key WEATHERAPICOM_KEY=key cargo run
```

## Usage
```shell
curl http://localhost:9998/api/weather?city=Samara # Weather today
curl http://localhost:9998/api/weather?date=2021-06-26&city=Samara # Weather in date
curl http://localhost:9998/api/weather/week_ahead?city=Samara # Forecast
```

## Docker
```shell
docker buidl -t netology -f docker/Dockerfile .
cd docker
docker-compose up
```