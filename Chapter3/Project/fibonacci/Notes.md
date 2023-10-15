# Fibonnaci notes

 - `u64` is apparently too small to hold anything larger than the 93rd fibonacci number.
 - I found this through experiment (Yay binary search!)
 - I should come back to this once I know a method to prevent this error.
 - Reusing a string that's been trimmed is bad, apparently
  - I don't know a way of fixing this other than to move the string declaration inside of the loop.