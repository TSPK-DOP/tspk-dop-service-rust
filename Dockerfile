# Используем официальный образ Rust для сборки
FROM rust:1.74 as builder

# Устанавливаем необходимые зависимости для работы с PostgreSQL и другими библиотеками
RUN apt-get update && apt-get install -y libpq-dev pkg-config build-essential

# Устанавливаем рабочую директорию для проекта
WORKDIR /usr/src/app

# Копируем Cargo.toml и Cargo.lock отдельно для кэширования зависимостей
COPY Cargo.toml Cargo.lock ./

# Копируем остальные необходимые файлы для проекта
COPY entity ./entity
COPY migration ./migration
# Создаем пустой бинарник для кэширования зависимостей
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Сборка зависимостей
RUN cargo build --release

# Копируем файлы проекта
COPY . .

# Сборка релизного бинарного файла
RUN cargo build --release

# Используем минимальный образ для финальной сборки
FROM debian:bookworm-slim 

# Устанавливаем необходимые зависимости для работы с бинарником
RUN apt-get update && apt-get install -y libpq-dev ca-certificates && rm -rf /var/lib/apt/lists/*

# Устанавливаем рабочую директорию
WORKDIR /usr/src/app

# Копируем собранный бинарный файл из предыдущего этапа
COPY --from=builder /usr/src/app/target/release/rusty-chat .

# Указываем команду для запуска контейнера
CMD ["./rusty-chat"]
