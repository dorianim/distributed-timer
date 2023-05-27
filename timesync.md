# Timesync

This is how clients are supposed to synchronize their time with the server:

```
averageOffset := -1
averageFluctuation := -1
offsetHistory := []
fluctuationHistory := []

fn isOffsetInMargin(newOffset) {
  if(averageOffset == -1)
    return true
  
  newFluctuation := abs(averageOffset - newOffset)
  
  if (averageFluctuation > 0 && fluctuationHistory.length > 5 && newFluctuation > (averageFluctuation * 4))
    return false
    
  currentFluctuation = pushValueAndCaluclateAverage(fluctuationHistory, newFluctuation)
  
  if (fluctuationHistory.length < 10)
    return true
    
  return newFluctuation < currentFluctuation * 2;
}

fn handleNewOffset(newOffset) {
  if(!offsetIsInMargin(newOffset))
    return
  
  currentOffset = pushValueAndCaluclateAverage(offsetHistory, newOffset)
}

lastGetTimeSent := -1

while true {
  lastGetTimeSent = currentTime()
  reply := webSocket.sendAndWaitForReply({ type: 'GetTime' })
  newOffset := currentTime() - lastGetTimeSent
  handleNewOffset(newOffset)
}
```
