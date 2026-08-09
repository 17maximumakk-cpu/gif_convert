// gif_convert.cpp
#include <iostream>
#include <string>
#include <vector>
#include <cstdlib>
#include <fstream>
#include <sstream>
#include <unistd.h>

using namespace std;

bool checkFFmpeg() {
    return system("ffmpeg -version > /dev/null 2>&1") == 0;
}

string buildCmd(const string& input, const string& output, int fps, const string& scale,
                double start, double duration, int quality, const string& filter, bool reverse) {
    ostringstream cmd;
    cmd << "ffmpeg -i " << input << " -y";
    if (start > 0) {
        cmd << " -ss " << start;
    }
    if (duration > 0) {
        cmd << " -t " << duration;
    }
    vector<string> filters;
    if (fps > 0) {
        filters.push_back("fps=" + to_string(fps));
    }
    if (!scale.empty()) {
        filters.push_back("scale=" + scale);
    }
    if (!filter.empty()) {
        if (filter == "grayscale") {
            filters.push_back("hue=s=0");
        } else if (filter == "sepia") {
            filters.push_back("colorchannelmixer=.393:.769:.189:.349:.686:.168:.272:.534:.131");
        } else if (filter == "negate") {
            filters.push_back("negate");
        }
    }
    if (reverse) {
        filters.push_back("reverse");
    }
    if (!filters.empty()) {
        cmd << " -vf \"";
        for (size_t i=0; i<filters.size(); ++i) {
            if (i>0) cmd << ",";
            cmd << filters[i];
        }
        cmd << "\"";
    }
    if (quality < 1) quality = 1;
    if (quality > 31) quality = 31;
    cmd << " -q:v " << quality;
    cmd << " -c:v gif " << output;
    return cmd.str();
}

int main(int argc, char* argv[]) {
    if (argc < 2) {
        cerr << "Использование: " << argv[0] << " <input> [--output file] [--fps N] [--scale WxH] [--start S] [--duration D] [--quality Q] [--filter F] [--reverse]" << endl;
        return 1;
    }
    if (!checkFFmpeg()) {
        cerr << "Ошибка: ffmpeg не найден. Установите FFmpeg и добавьте в PATH." << endl;
        return 1;
    }
    string input = argv[1];
    string output = "output.gif";
    int fps = 10;
    string scale;
    double start = 0.0;
    double duration = 0.0;
    int quality = 10;
    string filter;
    bool reverse = false;

    for (int i=2; i<argc; ++i) {
        string arg = argv[i];
        if (arg == "--output" && i+1 < argc) {
            output = argv[++i];
        } else if (arg == "--fps" && i+1 < argc) {
            fps = stoi(argv[++i]);
        } else if (arg == "--scale" && i+1 < argc) {
            scale = argv[++i];
        } else if (arg == "--start" && i+1 < argc) {
            start = stod(argv[++i]);
        } else if (arg == "--duration" && i+1 < argc) {
            duration = stod(argv[++i]);
        } else if (arg == "--quality" && i+1 < argc) {
            quality = stoi(argv[++i]);
        } else if (arg == "--filter" && i+1 < argc) {
            filter = argv[++i];
        } else if (arg == "--reverse") {
            reverse = true;
        } else {
            cerr << "Неизвестный аргумент: " << arg << endl;
            return 1;
        }
    }
    // Проверка существования входного файла
    ifstream f(input.c_str());
    if (!f.good()) {
        cerr << "Файл " << input << " не найден." << endl;
        return 1;
    }
    string cmd = buildCmd(input, output, fps, scale, start, duration, quality, filter, reverse);
    cout << "Выполняется команда: " << cmd << endl;
    int res = system(cmd.c_str());
    if (res != 0) {
        cerr << "Ошибка конвертации." << endl;
        return 1;
    }
    cout << "✅ GIF создан: " << output << endl;
    return 0;
}
