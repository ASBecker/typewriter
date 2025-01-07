The aim is to play a sound PRECISELY ~100ms before a character is revealed. It should feel in sync with what's shown on the screen.

Here are some guidelines for how to implement this:

1. Audio System Architecture
- Use a thread-safe audio queue system to handle sound requests
- Create a dedicated audio thread separate from the main rendering thread
- Implement a bounded channel (e.g., using tokio::mpsc or crossbeam-channel) between threads

2. Sound Request Management
- Create an enum for different sound types (KeyPress, SpaceBar, Return, etc.)
- Implement a small pool of pre-loaded sound buffers to avoid loading from disk during playback
- Use a priority system where newer sounds can interrupt older queued sounds if backup occurs

3. Timing Synchronization
- Track the visual character reveal timestamp
- Schedule sound playback to align with the 300ms character reveal delay
- Add a small offset (perhaps 10-20ms) before the visual reveal to account for audio system latency

4. Buffer Management
- Implement a ring buffer for sound requests with a reasonable capacity (e.g., 8-16 requests)
- When buffer fills up, use a strategy like:
  - Drop oldest unplayed sounds while keeping newest
  - Or merge multiple quick keypresses into a single sound
- Keep track of currently playing sounds to manage memory

5. Error Handling
- Implement graceful fallbacks if audio device is unavailable
- Handle buffer underruns without crashing
- Provide recovery mechanisms if audio thread falls behind

6. Performance Considerations
- Pre-allocate all audio resources during startup
- Use lock-free data structures for inter-thread communication
- Implement sound mixing in the audio thread rather than queuing multiple separate plays
- Monitor audio thread CPU usage to prevent system overload


Additional considerations:

- In the folder "sounds" there are six different "click{n}.wav" files, which should be mapped to different groups of characters.
- Slightly randomize the sound pitch and volume to make it feel more natural.
- Additionally, a "classic-return.wav" file is provided, which should be played when the user presses Enter at 20% volume.
