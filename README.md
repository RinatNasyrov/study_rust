# Учусь расту делаю шнягу

## Пространства
- docs_examples - решения задачек из документации
- asteroids_project - пример простой игры на macroquad, на нем разбтрался как вынести код в отдельные .rs файлы
- my-game - разбираюсь с macroquad документацией

## Запуск
cargo build - сбилдить все
cargo run -p space_name --bin bin_name - Запуск мейна из бинарника с именем из Cargo.toml\
cargo run -p space_name - Запуск без имени бинарника, если в проекте один мейн\
*Пример команды: cargo run -p docs_examples --bin HelloWorld*

## Новый проект
cargo new project_name
