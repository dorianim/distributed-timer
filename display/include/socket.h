#pragma once

#include <Arduino.h>

#include "timer.h"

namespace socket {

void init(String timerId);
void loop();

timer::TimerData timerData();
TIME offset();
int error();
} // namespace socket