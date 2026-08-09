// gif_convert.go
package main

import (
	"flag"
	"fmt"
	"os"
	"os/exec"
	"path/filepath"
	"strconv"
	"strings"
)

func checkFFmpeg() {
	if _, err := exec.LookPath("ffmpeg"); err != nil {
		fmt.Fprintln(os.Stderr, "Ошибка: ffmpeg не найден. Установите FFmpeg и добавьте в PATH.")
		os.Exit(1)
	}
}

func buildCmd(input, output string, fps int, scale string, start, duration float64, quality int, filter string, reverse bool) []string {
	cmd := []string{"ffmpeg", "-i", input, "-y"}
	if start > 0 {
		cmd = append(cmd, "-ss", strconv.FormatFloat(start, 'f', -1, 64))
	}
	if duration > 0 {
		cmd = append(cmd, "-t", strconv.FormatFloat(duration, 'f', -1, 64))
	}
	filters := []string{}
	if fps > 0 {
		filters = append(filters, fmt.Sprintf("fps=%d", fps))
	}
	if scale != "" {
		filters = append(filters, fmt.Sprintf("scale=%s", scale))
	}
	if filter != "" {
		switch filter {
		case "grayscale":
			filters = append(filters, "hue=s=0")
		case "sepia":
			filters = append(filters, "colorchannelmixer=.393:.769:.189:.349:.686:.168:.272:.534:.131")
		case "negate":
			filters = append(filters, "negate")
		}
	}
	if reverse {
		filters = append(filters, "reverse")
	}
	if len(filters) > 0 {
		cmd = append(cmd, "-vf", strings.Join(filters, ","))
	}
	// Качество
	if quality < 1 {
		quality = 1
	} else if quality > 31 {
		quality = 31
	}
	cmd = append(cmd, "-q:v", strconv.Itoa(quality))
	cmd = append(cmd, "-c:v", "gif", output)
	return cmd
}

func convert(input, output string, fps int, scale string, start, duration float64, quality int, filter string, reverse bool) {
	checkFFmpeg()
	cmdArgs := buildCmd(input, output, fps, scale, start, duration, quality, filter, reverse)
	fmt.Println("Выполняется команда:", strings.Join(cmdArgs, " "))
	cmd := exec.Command(cmdArgs[0], cmdArgs[1:]...)
	cmd.Stdout = os.Stdout
	cmd.Stderr = os.Stderr
	if err := cmd.Run(); err != nil {
		fmt.Fprintf(os.Stderr, "Ошибка конвертации: %v\n", err)
		os.Exit(1)
	}
	fmt.Printf("✅ GIF создан: %s\n", output)
}

func main() {
	input := flag.String("input", "", "Входной видеофайл")
	output := flag.String("output", "output.gif", "Выходной GIF")
	fps := flag.Int("fps", 10, "Частота кадров")
	scale := flag.String("scale", "", "Размер (WxH)")
	start := flag.Float64("start", 0, "Начало в секундах")
	duration := flag.Float64("duration", 0, "Длительность в секундах")
	quality := flag.Int("quality", 10, "Качество (1-31)")
	filter := flag.String("filter", "", "Фильтр (grayscale/sepia/negate)")
	reverse := flag.Bool("reverse", false, "Реверсировать")

	flag.Parse()

	if *input == "" {
		fmt.Println("Не указан входной файл. Используйте -input <файл>")
		os.Exit(1)
	}
	if _, err := os.Stat(*input); os.IsNotExist(err) {
		fmt.Fprintf(os.Stderr, "Файл %s не найден.\n", *input)
		os.Exit(1)
	}
	convert(*input, *output, *fps, *scale, *start, *duration, *quality, *filter, *reverse)
}
