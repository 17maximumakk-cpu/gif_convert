// GifConvert.java
import java.io.*;
import java.nio.file.*;
import java.util.*;

public class GifConvert {

    private static boolean checkFFmpeg() {
        try {
            Process p = Runtime.getRuntime().exec(new String[]{"ffmpeg", "-version"});
            return p.waitFor() == 0;
        } catch (Exception e) {
            return false;
        }
    }

    private static List<String> buildCmd(String input, String output, int fps, String scale,
                                         double start, double duration, int quality,
                                         String filter, boolean reverse) {
        List<String> cmd = new ArrayList<>();
        cmd.add("ffmpeg");
        cmd.add("-i");
        cmd.add(input);
        cmd.add("-y");
        if (start > 0) {
            cmd.add("-ss");
            cmd.add(String.valueOf(start));
        }
        if (duration > 0) {
            cmd.add("-t");
            cmd.add(String.valueOf(duration));
        }
        List<String> filters = new ArrayList<>();
        if (fps > 0) {
            filters.add("fps=" + fps);
        }
        if (scale != null && !scale.isEmpty()) {
            filters.add("scale=" + scale);
        }
        if (filter != null && !filter.isEmpty()) {
            switch (filter) {
                case "grayscale":
                    filters.add("hue=s=0");
                    break;
                case "sepia":
                    filters.add("colorchannelmixer=.393:.769:.189:.349:.686:.168:.272:.534:.131");
                    break;
                case "negate":
                    filters.add("negate");
                    break;
            }
        }
        if (reverse) {
            filters.add("reverse");
        }
        if (!filters.isEmpty()) {
            cmd.add("-vf");
            cmd.add(String.join(",", filters));
        }
        if (quality < 1) quality = 1;
        if (quality > 31) quality = 31;
        cmd.add("-q:v");
        cmd.add(String.valueOf(quality));
        cmd.add("-c:v");
        cmd.add("gif");
        cmd.add(output);
        return cmd;
    }

    private static void convert(String input, String output, int fps, String scale,
                                double start, double duration, int quality,
                                String filter, boolean reverse) throws Exception {
        if (!checkFFmpeg()) {
            System.err.println("Ошибка: ffmpeg не найден. Установите FFmpeg и добавьте в PATH.");
            System.exit(1);
        }
        List<String> cmdList = buildCmd(input, output, fps, scale, start, duration, quality, filter, reverse);
        System.out.println("Выполняется команда: " + String.join(" ", cmdList));
        ProcessBuilder pb = new ProcessBuilder(cmdList);
        pb.inheritIO();
        Process p = pb.start();
        int exitCode = p.waitFor();
        if (exitCode != 0) {
            System.err.println("Ошибка конвертации.");
            System.exit(1);
        }
        System.out.println("✅ GIF создан: " + output);
    }

    public static void main(String[] args) throws Exception {
        if (args.length < 1) {
            System.err.println("Использование: java GifConvert <input> [--output file] [--fps N] [--scale WxH] [--start S] [--duration D] [--quality Q] [--filter F] [--reverse]");
            System.exit(1);
        }
        String input = args[0];
        String output = "output.gif";
        int fps = 10;
        String scale = null;
        double start = 0.0;
        double duration = 0.0;
        int quality = 10;
        String filter = null;
        boolean reverse = false;

        for (int i = 1; i < args.length; i++) {
            switch (args[i]) {
                case "--output":
                    if (i+1 < args.length) output = args[++i];
                    break;
                case "--fps":
                    if (i+1 < args.length) fps = Integer.parseInt(args[++i]);
                    break;
                case "--scale":
                    if (i+1 < args.length) scale = args[++i];
                    break;
                case "--start":
                    if (i+1 < args.length) start = Double.parseDouble(args[++i]);
                    break;
                case "--duration":
                    if (i+1 < args.length) duration = Double.parseDouble(args[++i]);
                    break;
                case "--quality":
                    if (i+1 < args.length) quality = Integer.parseInt(args[++i]);
                    break;
                case "--filter":
                    if (i+1 < args.length) filter = args[++i];
                    break;
                case "--reverse":
                    reverse = true;
                    break;
                default:
                    System.err.println("Неизвестный аргумент: " + args[i]);
                    System.exit(1);
            }
        }
        if (!Files.exists(Paths.get(input))) {
            System.err.println("Файл " + input + " не найден.");
            System.exit(1);
        }
        convert(input, output, fps, scale, start, duration, quality, filter, reverse);
    }
}
