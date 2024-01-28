#pragma once

#include <Arduino.h>

#define TIME unsigned long long

namespace timer
{
  struct TimerSegment
  {
    bool valid;
    unsigned long time;
    unsigned long count_to;
    bool sound;
    uint32_t color;
    char label[32];
  };

  enum class PreStartBehaviour
  {
    SHOW_FIRST_SEGMENT,
    SHOW_LAST_SEGMENT,
    RUN_NORMALLY
  };

  struct DisplayOptions
  {
    PreStartBehaviour pre_start_behaviour;
    bool clock;
  };

  struct TimerData
  {
    bool valid;
    bool repeat;
    TIME start_at;
    TIME stop_at;
    DisplayOptions display_options;
    TimerSegment segments[10];
  };

  struct ActiveSegment
  {
    long seconds;
    uint32_t color;
    const char *label;
    TIME currentTime;
  };

  TimerData *timerData();
  ActiveSegment calculateCurrentSegment(TIME timeOffset);
}; // namespace timer