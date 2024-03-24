cargo build --release
Copy-Item -Path ".\target\release\izi_directory_finder.exe" -Destination "E:\shellScripts\izi_directory_finder.exe" -Force
Copy-Item -Path ".\zd.ps1" -Destination "E:\shellScripts\zd.ps1" -Force