import glob
import shutil
import os
import subprocess
import pathlib

zip_filename = "Bitmapflow"
dst_dir = "bin/Bitmapflow"

zip_filename_debug = "Bitmapflow_debug"
dst_dir_debug = "bin/Bitmapflow_debug"

# Delete folders
if os.path.exists(dst_dir):
    shutil.rmtree(dst_dir)
if os.path.exists(zip_filename):
    shutil.rmtree(zip_filename)
if os.path.exists(dst_dir_debug):
    shutil.rmtree(dst_dir_debug)
if os.path.exists(zip_filename_debug):
    shutil.rmtree(zip_filename_debug)

# Make folders
pathlib.Path(dst_dir).mkdir(parents=True, exist_ok=True)
pathlib.Path(dst_dir_debug).mkdir(parents=True, exist_ok=True)

# Build GDNative library with production optimizations
subprocess.run(
    ["cargo", "build", "--release"],
    cwd="rust",
    env={"RUSTFLAGS": "-C codegen-units=1", "CARGO_PROFILE_RELEASE_LTO": "fat"}
)

# Copy all dlls
src_dir = "godot"
for dll in glob.iglob(os.path.join(src_dir, "*.dll")):
    shutil.copy(dll, dst_dir)
    shutil.copy(dll, dst_dir_debug)

# Export godot project (note: folder path is relative to the godot project folder, not current folder)
subprocess.run(["godot", "--no-window", "--export",
                "Windows Desktop", "../" + dst_dir + "/Bitmapflow.exe"], cwd="godot")
subprocess.run(["godot", "--no-window", "--export-debug",
                "Windows Desktop", "../" + dst_dir_debug + "/Bitmapflow_debug.exe"], cwd="godot")

# Zip it
print("Zipping...")
shutil.make_archive("bin/" + zip_filename, 'zip', dst_dir)

print("Success.")

print("Zipping debug...")
shutil.make_archive("bin/" + zip_filename_debug, 'zip', dst_dir_debug)

print("Success.")
