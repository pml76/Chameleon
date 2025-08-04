import os
import psutil

# Get the current process ID
pid = os.getpid()

# Get the process object
process = psutil.Process(pid)

# Print the loaded shared libraries before importing
print("Loaded libraries BEFORE importing the module:")
for lib in process.memory_maps():
    if lib.path.endswith(('.dll', '.so', '.dylib')):  # Extensions for shared libraries
        print(lib.path)

print("\nLoading the module...")
# Import the desired module
import pyarrow

# Print the loaded shared libraries after importing
print("\nLoaded libraries AFTER importing the module:")
for lib in process.memory_maps():
    if lib.path.endswith(('.dll', '.so', '.dylib')):
        print(lib.path)
