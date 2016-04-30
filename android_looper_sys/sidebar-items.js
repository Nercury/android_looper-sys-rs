initSidebarItems({"enum":[["ALooper","ALooper"],["LooperPoll","Result from `ALooper_pollOnce()` and `ALooper_pollAll()`."],["LooperPrepareOpts","Option for for `ALooper_prepare()`."]],"fn":[["ALooper_acquire","Acquire a reference on the given ALooper object.  This prevents the object from being deleted until the reference is removed.  This is only needed to safely hand an ALooper from one thread to another."],["ALooper_addFd","Adds a new file descriptor to be polled by the looper. If the same file descriptor was previously added, it is replaced."],["ALooper_forThread","Returns the looper associated with the calling thread, or NULL if there is not one."],["ALooper_pollAll","Like ALooper_pollOnce(), but performs all pending callbacks until all data has been consumed or a file descriptor is available with no callback. This function will never return ALOOPER_POLL_CALLBACK."],["ALooper_pollOnce","Waits for events to be available, with optional timeout in milliseconds. Invokes callbacks for all file descriptors on which an event occurred."],["ALooper_prepare","Prepares a looper associated with the calling thread, and returns it. If the thread already has a looper, it is returned.  Otherwise, a new one is created, associated with the thread, and returned."],["ALooper_release","Remove a reference that was previously acquired with ALooper_acquire()."],["ALooper_removeFd","Removes a previously added file descriptor from the looper."],["ALooper_wake","Wakes the poll asynchronously."]],"mod":[["event","Flags for file descriptor events that a looper can monitor."]],"type":[["ALooper_callbackFunc","For callback-based event loops, this is the prototype of the function that is called when a file descriptor event occurs. It is given the file descriptor it is associated with, a bitmask of the poll events that were triggered (typically ALOOPER_EVENT_INPUT), and the data pointer that was originally supplied."]]});