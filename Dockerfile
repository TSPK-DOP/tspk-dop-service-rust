# Мультиэтапная сборка для минимального образа
# Этап 1: Сборка приложения
FROM rust:1.74 as builder
RUN apt-get update && apt-get install -y libpq-dev pkg-config build-essential

# Устанавливаем рабочую директорию
WORKDIR /usr/src/app

# Копируем только Cargo.toml и Cargo.lock для кэширования зависимостей
COPY Cargo.toml Cargo.lock ./

# Копируем все зависимости для кэширования
COPY entity/Cargo.toml ./entity/
COPY migration/Cargo.toml ./migration/

# Устанавливаем зависимости
RUN cargo fetch

# Копируем весь код проекта
COPY . .

# Сборка релизного бинарного файла
RUN cargo build --release

# Этап 2: Минимальный образ для запуска приложения
FROM debian:bookworm-slim 

# Установка зависимостей для PostgreSQL
RUN apt-get update && apt-get install -y libpq-dev

# Копируем бинарный файл из предыдущего этапа
COPY --from=builder /usr/src/app/target/release/service /usr/local/bin/

# Устанавливаем права на выполнение бинарного файла
RUN chmod +x /usr/local/bin/service

# Установка переменных окружения для подключения к базе данных
ENV DATABASE_URL=postgres://pweb:pweb@localhost/education
ENV PORT=8080
ENV ADDRESS=0.0.0.0

# Указываем команду для запуска приложения
CMD ["service"]
