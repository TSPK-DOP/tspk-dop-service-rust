# Мультиэтапная сборка для минимального образа
# Этап 1: Сборка приложения
FROM rust:1.74 as builder

# Устанавливаем рабочую директорию
WORKDIR /usr/src/app

# Копируем Cargo.toml и Cargo.lock для кэширования зависимостей
COPY Cargo.toml Cargo.lock ./

# Устанавливаем зависимости
RUN cargo fetch

# Копируем исходный код
COPY . .

# Сборка релизного бинарного файла
RUN cargo build --release

# Этап 2: Минимальный образ с бинарником
FROM debian:buster-slim

# Копируем бинарный файл из предыдущего этапа
COPY --from=builder /usr/src/app/target/release/rusty-chat /usr/local/bin/

# Устанавливаем права на выполнение
RUN chmod +x /usr/local/bin/rusty-chat

# Указываем команду для запуска контейнера
CMD ["rusty-chat"]