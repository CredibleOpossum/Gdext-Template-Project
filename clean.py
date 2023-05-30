import os
import shutil

if os.path.isfile("Unmanaged.gdextension"):
    for directory in ["export", "src/rust/target"]:
        try:
            shutil.rmtree(directory)
        except Exception as e:
            print(f"Failed to delete {directory}, {e}")
    os.mkdir("export")
else:
    print("Could not find Unmanged.gdextension, are you in the right directory?")
