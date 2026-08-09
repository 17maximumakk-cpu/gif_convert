# gif_convert.rb
#!/usr/bin/env ruby

require 'optparse'
require 'shellwords'

def check_ffmpeg
  system('ffmpeg -version > /dev/null 2>&1') or begin
    $stderr.puts "Ошибка: ffmpeg не найден. Установите FFmpeg и добавьте в PATH."
    exit 1
  end
end

def build_cmd(input, output, fps, scale, start, duration, quality, filter, reverse)
  cmd = ['ffmpeg', '-i', input, '-y']
  cmd += ['-ss', start.to_s] if start.to_f > 0
  cmd += ['-t', duration.to_s] if duration.to_f > 0

  filters = []
  filters << "fps=#{fps}" if fps.to_i > 0
  filters << "scale=#{scale}" if scale && !scale.empty?
  case filter
  when 'grayscale'
    filters << 'hue=s=0'
  when 'sepia'
    filters << 'colorchannelmixer=.393:.769:.189:.349:.686:.168:.272:.534:.131'
  when 'negate'
    filters << 'negate'
  end
  filters << 'reverse' if reverse
  cmd += ['-vf', filters.join(',')] unless filters.empty?

  q = quality.to_i
  q = 1 if q < 1
  q = 31 if q > 31
  cmd += ['-q:v', q.to_s, '-c:v', 'gif', output]
  Shellwords.join(cmd)
end

def convert(input, output, fps, scale, start, duration, quality, filter, reverse)
  check_ffmpeg
  cmd = build_cmd(input, output, fps, scale, start, duration, quality, filter, reverse)
  puts "Выполняется команда: #{cmd}"
  unless system(cmd)
    $stderr.puts "Ошибка конвертации."
    exit 1
  end
  puts "✅ GIF создан: #{output}"
end

options = {}
OptionParser.new do |opts|
  opts.banner = "Использование: ruby gif_convert.rb <input> [опции]"
  opts.on("--output FILE", "Выходной GIF") { |v| options[:output] = v }
  opts.on("--fps N", Integer, "Частота кадров") { |v| options[:fps] = v }
  opts.on("--scale WxH", "Размер") { |v| options[:scale] = v }
  opts.on("--start S", Float, "Начало в секундах") { |v| options[:start] = v }
  opts.on("--duration D", Float, "Длительность") { |v| options[:duration] = v }
  opts.on("--quality Q", Integer, "Качество (1-31)") { |v| options[:quality] = v }
  opts.on("--filter F", "Фильтр (grayscale/sepia/negate)") { |v| options[:filter] = v }
  opts.on("--reverse", "Реверсировать") { options[:reverse] = true }
end.parse!

input = ARGV[0]
if input.nil?
  puts "Не указан входной файл."
  exit 1
end
unless File.file?(input)
  puts "Файл #{input} не найден."
  exit 1
end

output = options[:output] || 'output.gif'
fps = options[:fps] || 10
scale = options[:scale]
start = options[:start] || 0.0
duration = options[:duration] || 0.0
quality = options[:quality] || 10
filter = options[:filter]
reverse = options[:reverse] || false

convert(input, output, fps, scale, start, duration, quality, filter, reverse)
