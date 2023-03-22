#pragma once

#include <Arduino.h>

#define TIME unsigned long long

namespace timer {
struct TimerSegment {
  bool valid;
  unsigned long time;
  bool sound;
  uint32_t color;
};

struct TimerData {
  bool valid;
  bool repeat;
  TIME start_at;
  TIME stop_at;
  TimerSegment segments[10];
};

struct ActiveSegment {
  long seconds;
  uint32_t color;
};

ActiveSegment calculateCurrentSegment(TimerData timerData, TIME timeOffset);
}; // namespace timer