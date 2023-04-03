#pragma once

#include <Adafruit_NeoMatrix.h>

#include "display.h"

namespace wifi {

bool init(Display *display, bool reset);
void loop();
bool connected();
String timerId();
} // namespace wifi
