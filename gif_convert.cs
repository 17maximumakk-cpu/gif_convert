// gif_convert.cs
using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.IO;

class GifConvert
{
    static void CheckFFmpeg()
    {
        try
        {
            Process.Start(new ProcessStartInfo("ffmpeg", "-version") { RedirectStandardOutput = true }).WaitForExit();
        }
        catch
        {
            Console.Error.WriteLine("Ошибка: ffmpeg не найден. Установите FFmpeg и добавьте в PATH.");
            Environment.Exit(1);
        }
    }

    static string BuildCmd(string input, string output, int fps, string scale,
                           double start, double duration, int quality,
                           string filter, bool reverse)
    {
        var args = new List<string>();
        args.Add("-i");
        args.Add(input);
        args.Add("-y");
        if (start > 0)
        {
            args.Add("-ss");
            args.Add(start.ToString());
        }
        if (duration > 0)
        {
            args.Add("-t");
            args.Add(duration.ToString());
        }
        var filters = new List<string>();
        if (fps > 0)
            filters.Add($"fps={fps}");
        if (!string.IsNullOrEmpty(scale))
            filters.Add($"scale={scale}");
        if (!string.IsNullOrEmpty(filter))
        {
            switch (filter.ToLower())
            {
                case "grayscale":
                    filters.Add("hue=s=0");
                    break;
                case "sepia":
                    filters.Add("colorchannelmixer=.393:.769:.189:.349:.686:.168:.272:.534:.131");
                    break;
                case "negate":
                    filters.Add("negate");
                    break;
            }
        }
        if (reverse)
            filters.Add("reverse");
        if (filters.Count > 0)
        {
            args.Add("-vf");
            args.Add(string.Join(",", filters));
        }
        if (quality < 1) quality = 1;
        if (quality > 31) quality = 31;
        args.Add("-q:v");
        args.Add(quality.ToString());
        args.Add("-c:v");
        args.Add("gif");
        args.Add(output);
        return string.Join(" ", args);
    }

    static void Convert(string input, string output, int fps, string scale,
                        double start, double duration, int quality,
                        string filter, bool reverse)
    {
        CheckFFmpeg();
        string args = BuildCmd(input, output, fps, scale, start, duration, quality, filter, reverse);
        Console.WriteLine($"Выполняется команда: ffmpeg {args}");
        var psi = new ProcessStartInfo("ffmpeg", args)
        {
            UseShellExecute = false,
            RedirectStandardOutput = true,
            RedirectStandardError = true
        };
        using (var p = Process.Start(psi))
        {
            p.WaitForExit();
            if (p.ExitCode != 0)
            {
                Console.Error.WriteLine("Ошибка конвертации.");
                Environment.Exit(1);
            }
        }
        Console.WriteLine($"✅ GIF создан: {output}");
    }

    static void Main(string[] args)
    {
        if (args.Length < 1)
        {
            Console.Error.WriteLine("Использование: dotnet run <input> [--output file] [--fps N] [--scale WxH] [--start S] [--duration D] [--quality Q] [--filter F] [--reverse]");
            return;
        }
        string input = args[0];
        string output = "output.gif";
        int fps = 10;
        string scale = null;
        double start = 0.0;
        double duration = 0.0;
        int quality = 10;
        string filter = null;
        bool reverse = false;

        for (int i = 1; i < args.Length; i++)
        {
            switch (args[i])
            {
                case "--output": if (i+1 < args.Length) output = args[++i]; break;
                case "--fps": if (i+1 < args.Length) fps = int.Parse(args[++i]); break;
                case "--scale": if (i+1 < args.Length) scale = args[++i]; break;
                case "--start": if (i+1 < args.Length) start = double.Parse(args[++i]); break;
                case "--duration": if (i+1 < args.Length) duration = double.Parse(args[++i]); break;
                case "--quality": if (i+1 < args.Length) quality = int.Parse(args[++i]); break;
                case "--filter": if (i+1 < args.Length) filter = args[++i]; break;
                case "--reverse": reverse = true; break;
                default:
                    Console.Error.WriteLine($"Неизвестный аргумент: {args[i]}");
                    Environment.Exit(1);
                    break;
            }
        }
        if (!File.Exists(input))
        {
            Console.Error.WriteLine($"Файл {input} не найден.");
            Environment.Exit(1);
        }
        Convert(input, output, fps, scale, start, duration, quality, filter, reverse);
    }
}
