#pragma once

#include <Arduino.h>

#define TIME unsigned long long

namespace timer {
struct TimerSegment {
  bool valid;
  unsigned long time;
  unsigned long count_to;
  bool sound;
  uint32_t color;
};

enum class PreStartBehaviour { SHOW_ZERO, RUN_NORMALLY };

struct DisplayOptions {
  PreStartBehaviour pre_start_behaviour;
};

struct TimerData {
  bool valid;
  bool repeat;
  TIME start_at;
  TIME stop_at;
  DisplayOptions display_options;
  TimerSegment segments[10];
};

struct ActiveSegment {
  long seconds;
  uint32_t color;
};

ActiveSegment calculateCurrentSegment(TimerData timerData, TIME timeOffset);
}; // namespace timer