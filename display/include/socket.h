#pragma once

#include <Arduino.h>

#include "display.h"
#include "timer.h"

namespace socket {

void init(String timerId);
void loop();

bool connected();
TIME offset();
int error();
} // namespace socket