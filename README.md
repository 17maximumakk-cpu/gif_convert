🎬 Конвертер видео (GIF) — создавайте анимации с лёгкостью
Версия: 1.0.0 | Лицензия: MIT | Статус: ✅ Активная разработка

https://img.shields.io/github/repo-size/yourusername/video-to-gif https://img.shields.io/github/last-commit/yourusername/video-to-gif https://img.shields.io/github/languages/count/yourusername/video-to-gif

🎞️ Описание
Конвертер видео (GIF) — это консольная утилита для преобразования видеофайлов (MP4, AVI, MOV, MKV и др.) в анимированные GIF-изображения. Программа предоставляет богатый набор опций для настройки конечного результата:

✅ Настройка частоты кадров (FPS)

✅ Изменение размера (пропорциональное или точное)

✅ Выбор временного отрезка (начало и длительность)

✅ Настройка качества GIF (уровень сжатия)

✅ Применение фильтров (черно-белый, сепия, инверсия)

✅ Реверсирование анимации

✅ Пакетная обработка нескольких файлов (опционально)

✅ Прогресс-бар и подробные логи

Проект содержит 8 полноценных реализаций на разных языках программирования. Все версии используют FFmpeg в качестве бэкенда — одного из самых мощных инструментов для работы с видео, доступного для всех платформ.

✨ Возможности
Функция	Описание
Конвертация видео → GIF	Преобразование с использованием FFmpeg
FPS	Задание частоты кадров (по умолчанию 10)
Размер	Указание ширины и высоты, пропорциональное масштабирование
Обрезка	Начало и длительность (в секундах)
Качество	Уровень сжатия (1 – лучшее качество, 31 – сильное сжатие)
Фильтры	grayscale, sepia, negate
Реверс	Обратный порядок кадров
Пакетная обработка	Обработка всех видео в папке (опционально)
Кроссплатформенность	Работает везде, где есть FFmpeg
📦 Установка и запуск
Общие требования
Для работы всех реализаций необходим установленный FFmpeg:

bash
# Ubuntu/Debian
sudo apt install ffmpeg

# macOS
brew install ffmpeg

# Windows
# Скачайте с https://ffmpeg.org/download.html и добавьте в PATH
Запуск на разных языках
Язык	Файл	Команда запуска
Python	gif_convert.py	python3 gif_convert.py input.mp4 --output out.gif
Go	gif_convert.go	go run gif_convert.go input.mp4 --output out.gif
Rust	gif_convert.rs	cargo run -- input.mp4 --output out.gif
C++	gif_convert.cpp	g++ -std=c++17 -o gif_convert gif_convert.cpp && ./gif_convert input.mp4 --output out.gif
Java	GifConvert.java	javac GifConvert.java && java GifConvert input.mp4 --output out.gif
C#	gif_convert.cs	dotnet run input.mp4 --output out.gif
Ruby	gif_convert.rb	ruby gif_convert.rb input.mp4 --output out.gif
Node.js	gif_convert.js	node gif_convert.js input.mp4 --output out.gif
📂 Структура репозитория
text
.
├── README.md
├── python/
│   └── gif_convert.py
├── go/
│   └── gif_convert.go
├── rust/
│   ├── Cargo.toml
│   └── src/
│       └── main.rs
├── cpp/
│   └── gif_convert.cpp
├── java/
│   └── GifConvert.java
├── csharp/
│   └── gif_convert.cs
├── ruby/
│   └── gif_convert.rb
└── javascript/
    ├── package.json
    └── gif_convert.js
🎮 Использование
bash
# Базовая конвертация
gif_convert input.mp4 --output output.gif

# Установить FPS и размер
gif_convert input.mp4 --fps 15 --scale 480x320

# Вырезать фрагмент с 10-й секунды длительностью 3 секунды
gif_convert input.mp4 --start 10 --duration 3

# Чёрно-белый GIF
gif_convert input.mp4 --filter grayscale

# Реверсивная анимация
gif_convert input.mp4 --reverse

# Качество (меньше = лучше)
gif_convert input.mp4 --quality 5
🛠️ Особенности реализаций
Python – использует subprocess и argparse; простой и понятный код.

Go – os/exec и flag; быстрый старт.

Rust – std::process::Command и clap (или ручной парсинг).

C++ – system() или popen; компактно.

Java – ProcessBuilder; кроссплатформенный.

C# – System.Diagnostics.Process.

Ruby – system или %x.

Node.js – child_process.exec и yargs для опций.

Все реализации используют FFmpeg, что обеспечивает единообразие и высокое качество конвертации.

🤝 Вклад
PR и issues приветствуются. Добавляйте поддержку новых фильтров, улучшайте производительность, расширяйте функциональность.

📄 Лицензия
MIT License.
