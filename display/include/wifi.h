#pragma once

#include "display.h"

namespace wifi
{

    bool init(Display *display);
    void reset();
    void loop();
    bool connected();
    String timerId();
    float timezoneOffset();
} // namespace wifi
