# Development Environment

## 1. Project

- Repository absolute path: `C:\programing\German`
- Repository URL: `git@github.com:johnson931128/German.git`
- Default branch: `main`

## 2. C++ Environment

- Compiler: MinGW-w64 GCC (`g++`)
- Compiler version: `14.2.0` (`x86_64-ucrt-posix-seh`, built by Brecht Sanders, r2)
- Compiler absolute path: `C:\mingw64\bin\g++.exe`
- C++ standard: `TBD` — 尚無 `CMakeLists.txt` 或專案編譯設定

## 3. Build System

- CMake version: `3.30.4`
- CMake absolute path: `C:\mingw64\bin\cmake.exe`
- 預計使用的 CMake generator: `TBD` — 尚未由專案設定決定；目前 CMake 實際可用 `Ninja`、`MinGW Makefiles`、`Unix Makefiles` 與 Visual Studio generators

## 4. SFML

- SFML version: `Not Installed`
- SFML installation path: `Not Installed`
- include path: `Not Installed`
- library path: `Not Installed`

偵測範圍包含 repository、`C:\mingw64`、`C:\msys64\mingw64`、`C:\vcpkg` 及標準 `Program Files` SFML 位置，未發現 SFML 安裝或 SFML headers/libraries。

## 5. Java Environment

- java version: `OpenJDK 21.0.10` (LTS)
- javac version: `21.0.10`
- JAVA_HOME: `C:\Program Files\Microsoft\jdk-21.0.10.7-hotspot\`
- java executable absolute path: `C:\Program Files\Microsoft\jdk-21.0.10.7-hotspot\bin\java.exe`

## 6. Runtime / Tools

- OS: `Microsoft Windows 10 企業版` (`10.0.19045`, build `19045`)
- architecture: `x64` (`AMD64`)
- Git version: `2.54.0.windows.1`
- Git absolute path: `C:\Program Files\Git\cmd\git.exe`
- VS Code path: `C:\Microsoft VS Code\Code.exe`

## 7. Planned Directory Layout

目前確定需要的高階目錄：

```text
apps/
src/
include/
config/
docs/
```

僅記錄目錄規劃，尚未建立這些空目錄。

## 8. Build Commands

以下命令目前均為 `TBD`，因為 repository 尚無 `CMakeLists.txt`：

```powershell
# Configure
TBD

# Build
TBD

# Run
TBD
```
