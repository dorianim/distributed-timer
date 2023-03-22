#pragma once

#include <Adafruit_NeoMatrix.h>

namespace wifi {

bool init(Adafruit_NeoMatrix *matrix, bool reset);
void loop();
bool connected();
String timerId();
} // namespace wifi
