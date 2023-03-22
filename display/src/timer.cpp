#include "timer.h"

namespace timer {

ActiveSegment calculateCurrentSegment(TimerData timerData, TIME timeOffset) {
  if (!timerData.valid || timeOffset == 0) {
    return {0, 0xfff};
  }

  TIME currentTime = (TIME)millis() + timeOffset;
  if (currentTime < timerData.start_at) {
    return {0, 0xfff};
  }

  if (timerData.stop_at != 0 && currentTime > timerData.stop_at) {
    currentTime = timerData.stop_at;
  }

  long elapsedTime = currentTime - timerData.start_at;

  long totalTimePerRound = 0;
  for (int i = 0; i < 10 && timerData.segments[i].valid; i++) {
    totalTimePerRound += timerData.segments[i].time;
  }

  if (!timerData.repeat && elapsedTime > totalTimePerRound) {
    return {0, 0xfff};
  }

  long timeInCurrentRound = elapsedTime % totalTimePerRound;

  int currentSegmentIndex = 0;
  long timeInCurrentSegment = 0;
  while (timeInCurrentRound > 0 &&
         timerData.segments[currentSegmentIndex].valid) {
    timeInCurrentSegment =
        timerData.segments[currentSegmentIndex].time - timeInCurrentRound;
    timeInCurrentRound -= timerData.segments[currentSegmentIndex].time;
    currentSegmentIndex++;
  }

  return {timeInCurrentSegment / 1000,
          timerData.segments[currentSegmentIndex - 1].color};
}

}; // namespace timer