#include <time.h>

#include "timer.h"

namespace timer {

TimerData _timerData;

TimerData *timerData() { return &_timerData; }

TIME calculateTimeInCurrentRound(TIME currentTime) {

  if (currentTime < _timerData.start_at &&
      _timerData.display_options.pre_start_behaviour ==
          PreStartBehaviour::SHOW_FIRST_SEGMENT) {
    return 1;
  }

  long totalTimePerRound = 0;
  for (int i = 0; i < 10 && _timerData.segments[i].valid; i++) {
    totalTimePerRound += _timerData.segments[i].time;
  }

  if (currentTime < _timerData.start_at &&
      _timerData.display_options.pre_start_behaviour ==
          PreStartBehaviour::SHOW_LAST_SEGMENT) {
    return totalTimePerRound;
  }

  long elapsedTime = currentTime - _timerData.start_at;

  if (_timerData.stop_at != 0 && currentTime > _timerData.stop_at) {
    elapsedTime = _timerData.stop_at - _timerData.start_at;
  }

  if (!_timerData.repeat && elapsedTime > totalTimePerRound) {
    return totalTimePerRound;
  }

  return elapsedTime % totalTimePerRound;
}

ActiveSegment calculateCurrentSegment(TIME timeOffset) {
  if (!_timerData.valid || timeOffset == 0) {
    return {0, 0xfff, "", 0};
  }

  TIME currentTime = (TIME)millis() + timeOffset;
  long timeInCurrentRound = calculateTimeInCurrentRound(currentTime);

  int currentSegmentIndex = 0;
  long timeInCurrentSegment = 0;
  while (timeInCurrentRound > 0 &&
         _timerData.segments[currentSegmentIndex].valid) {
    timeInCurrentSegment =
        _timerData.segments[currentSegmentIndex].time - timeInCurrentRound;
    timeInCurrentRound -= _timerData.segments[currentSegmentIndex].time;
    currentSegmentIndex++;
  }

  timeInCurrentSegment += _timerData.segments[currentSegmentIndex - 1].count_to;

  return {timeInCurrentSegment / 1000,
          _timerData.segments[currentSegmentIndex - 1].color,
          _timerData.segments[currentSegmentIndex - 1].label, currentTime};
}

}; // namespace timer