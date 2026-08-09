// gif_convert.js
const { exec } = require('child_process');
const fs = require('fs');
const path = require('path');
const yargs = require('yargs/yargs');
const { hideBin } = require('yargs/helpers');

function checkFFmpeg() {
    return new Promise((resolve) => {
        exec('ffmpeg -version', (err) => {
            if (err) {
                console.error('Ошибка: ffmpeg не найден. Установите FFmpeg и добавьте в PATH.');
                process.exit(1);
            }
            resolve();
        });
    });
}

function buildCmd(input, output, fps, scale, start, duration, quality, filter, reverse) {
    let args = ['ffmpeg', '-i', input, '-y'];
    if (start > 0) {
        args.push('-ss', String(start));
    }
    if (duration > 0) {
        args.push('-t', String(duration));
    }
    const filters = [];
    if (fps > 0) {
        filters.push(`fps=${fps}`);
    }
    if (scale) {
        filters.push(`scale=${scale}`);
    }
    if (filter) {
        switch (filter) {
            case 'grayscale': filters.push('hue=s=0'); break;
            case 'sepia': filters.push('colorchannelmixer=.393:.769:.189:.349:.686:.168:.272:.534:.131'); break;
            case 'negate': filters.push('negate'); break;
        }
    }
    if (reverse) {
        filters.push('reverse');
    }
    if (filters.length > 0) {
        args.push('-vf', filters.join(','));
    }
    let q = quality;
    if (q < 1) q = 1;
    if (q > 31) q = 31;
    args.push('-q:v', String(q));
    args.push('-c:v', 'gif', output);
    return args;
}

function convert(input, output, fps, scale, start, duration, quality, filter, reverse) {
    return new Promise((resolve, reject) => {
        checkFFmpeg().then(() => {
            const args = buildCmd(input, output, fps, scale, start, duration, quality, filter, reverse);
            const cmd = args.join(' ');
            console.log('Выполняется команда:', cmd);
            const proc = exec(cmd, (err, stdout, stderr) => {
                if (err) {
                    console.error('Ошибка конвертации:', err);
                    reject(err);
                } else {
                    console.log(`✅ GIF создан: ${output}`);
                    resolve();
                }
            });
            proc.stdout.pipe(process.stdout);
            proc.stderr.pipe(process.stderr);
        });
    });
}

async function main() {
    const argv = yargs(hideBin(process.argv))
        .usage('Использование: $0 <input> [опции]')
        .option('output', { alias: 'o', type: 'string', description: 'Выходной GIF', default: 'output.gif' })
        .option('fps', { type: 'number', description: 'Частота кадров', default: 10 })
        .option('scale', { type: 'string', description: 'Размер (WxH)' })
        .option('start', { type: 'number', description: 'Начало в секундах', default: 0 })
        .option('duration', { type: 'number', description: 'Длительность в секундах', default: 0 })
        .option('quality', { type: 'number', description: 'Качество (1-31)', default: 10 })
        .option('filter', { type: 'string', description: 'Фильтр (grayscale/sepia/negate)' })
        .option('reverse', { type: 'boolean', description: 'Реверсировать' })
        .help()
        .parse();

    const input = argv._[0];
    if (!input) {
        console.error('Не указан входной файл.');
        process.exit(1);
    }
    if (!fs.existsSync(input)) {
        console.error(`Файл ${input} не найден.`);
        process.exit(1);
    }
    await convert(input, argv.output, argv.fps, argv.scale, argv.start, argv.duration,
                  argv.quality, argv.filter, argv.reverse);
}

main().catch(console.error);
