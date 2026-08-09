# gif_convert.py
import subprocess
import argparse
import os
import sys
import shutil
import tempfile

def check_ffmpeg():
    """Проверяет наличие ffmpeg в PATH."""
    if shutil.which("ffmpeg") is None:
        print("Ошибка: ffmpeg не найден. Установите FFmpeg и добавьте в PATH.", file=sys.stderr)
        sys.exit(1)

def build_ffmpeg_cmd(input_file, output_file, fps, scale, start, duration, quality, filter_type, reverse):
    """
    Формирует команду ffmpeg на основе опций.
    Возвращает список аргументов для subprocess.
    """
    cmd = ["ffmpeg", "-i", input_file, "-y"]
    filter_parts = []

    # Обрезка по времени
    if start is not None:
        cmd.extend(["-ss", str(start)])
    if duration is not None:
        cmd.extend(["-t", str(duration)])

    # Фильтры: fps, scale, filter, reverse
    if fps is not None:
        filter_parts.append(f"fps={fps}")
    if scale is not None:
        filter_parts.append(f"scale={scale}")
    if filter_type:
        if filter_type == "grayscale":
            filter_parts.append("hue=s=0")
        elif filter_type == "sepia":
            filter_parts.append("colorchannelmixer=.393:.769:.189:.349:.686:.168:.272:.534:.131")
        elif filter_type == "negate":
            filter_parts.append("negate")
    if reverse:
        # Реверс требует сначала генерацию палитры, но мы упростим:
        # Используем фильтр reverse после всех остальных
        filter_parts.append("reverse")

    if filter_parts:
        cmd.extend(["-vf", ",".join(filter_parts)])

    # Качество GIF (q:v от 2 до 31, 2 - лучшее качество)
    if quality is not None:
        # Ограничим качество диапазоном 1-31
        q = max(1, min(31, quality))
        cmd.extend(["-q:v", str(q)])
    else:
        cmd.extend(["-q:v", "10"])

    # Кодек и выход
    cmd.extend(["-c:v", "gif", output_file])
    return cmd

def convert(input_file, output_file, fps, scale, start, duration, quality, filter_type, reverse):
    """Запускает конвертацию."""
    check_ffmpeg()
    cmd = build_ffmpeg_cmd(input_file, output_file, fps, scale, start, duration, quality, filter_type, reverse)
    print("Выполняется команда:", " ".join(cmd))
    try:
        subprocess.run(cmd, check=True)
        print(f"✅ GIF создан: {output_file}")
    except subprocess.CalledProcessError as e:
        print(f"❌ Ошибка конвертации: {e}", file=sys.stderr)
        sys.exit(1)

def main():
    parser = argparse.ArgumentParser(description="Конвертер видео в GIF с использованием FFmpeg")
    parser.add_argument("input", help="Входной видеофайл")
    parser.add_argument("--output", "-o", default="output.gif", help="Выходной GIF-файл")
    parser.add_argument("--fps", type=int, default=10, help="Частота кадров (по умолчанию 10)")
    parser.add_argument("--scale", help="Размер в формате WIDTHxHEIGHT (например, 480x320)")
    parser.add_argument("--start", type=float, help="Начало в секундах")
    parser.add_argument("--duration", type=float, help="Длительность в секундах")
    parser.add_argument("--quality", type=int, default=10, help="Качество (1-31, меньше = лучше)")
    parser.add_argument("--filter", choices=["grayscale", "sepia", "negate"], help="Применить фильтр")
    parser.add_argument("--reverse", action="store_true", help="Реверсировать анимацию")
    args = parser.parse_args()

    if not os.path.isfile(args.input):
        print(f"Файл {args.input} не найден.", file=sys.stderr)
        sys.exit(1)

    convert(args.input, args.output, args.fps, args.scale, args.start,
            args.duration, args.quality, args.filter, args.reverse)

if __name__ == "__main__":
    main()
