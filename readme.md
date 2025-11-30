# 🧭 A* Pathfinding Visualization | Trực quan hóa thuật toán A*

<p align="center">
  <img src="https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white" alt="Rust"/>
  <img src="https://img.shields.io/badge/Macroquad-FF6B6B?style=for-the-badge" alt="Macroquad"/>
</p>

---

## 🇬🇧 English

### 📖 Description

A real-time visualization of the **A\* pathfinding algorithm** navigating through randomly generated mazes. Built with Rust and the Macroquad game library, this project demonstrates various heuristic functions and their effects on pathfinding efficiency.

### ✨ Features

- 🎲 **Random Maze Generation** - Uses recursive backtracking algorithm
- 🔍 **A\* Pathfinding** - Efficient shortest path algorithm
- 📊 **8 Different Heuristics** - Compare different distance calculation methods
- ⏱️ **Real-time Statistics** - Track time and steps taken
- 🎮 **Interactive Controls** - Switch heuristics and generate new mazes on the fly

### 🎯 Heuristics Available

| Key | Heuristic | Description |
|-----|-----------|-------------|
| `1` | Manhattan | Sum of absolute differences (L1 norm) |
| `2` | Euclidean | Straight-line distance (L2 norm) |
| `3` | Diagonal | Optimized for 8-directional movement |
| `4` | Uniform Cost Search | No heuristic (Dijkstra's algorithm) |
| `5` | Chebyshev | Maximum of absolute differences (L∞ norm) |
| `6` | Euclidean Squared | Euclidean without square root (faster) |
| `7` | Weighted Manhattan | Manhattan × 2 (aggressive) |
| `8` | Manhattan Tiebreaker | Manhattan × 1.001 (breaks ties) |

### 🚀 Getting Started

#### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (1.70+)

#### Installation

```bash
# Clone the repository
git clone <repository-url>
cd Project_I

# Build and run
cargo run --release
```

### 🎮 Controls

| Key | Action |
|-----|--------|
| `1-8` | Change heuristic function |
| `Space` | Generate new maze |

### 🛠️ Project Structure

```
Project_I/
├── src/
│   ├── main.rs        # Entry point & game loop
│   ├── algorithm.rs   # A* algorithm implementation
│   ├── maze.rs        # Maze generation (recursive backtracker)
│   ├── heuristic.rs   # Heuristic functions
│   ├── node.rs        # Node data structure
│   └── constants.rs   # Configuration constants
├── Cargo.toml
└── readme.md
```

### ⚙️ Configuration

You can modify constants in `src/constants.rs`:

```rust
pub const MAZE_HEIGH: usize = 101;        // Maze height
pub const MAZE_WIDTH: usize = 201;        // Maze width (2 * height - 1)
pub const STEP_DELAY_SEC: f64 = 0.0;      // Delay between steps
pub const STEPS_PER_FRAME: usize = 120;   // Steps per frame
```

### 🎨 Color Legend

| Color | Meaning |
|-------|---------|
| ⬛ Black | Wall |
| ⬜ White | Path |
| 🟩 Green | Start / Final path |
| 🔴 Red | Goal |
| 🔵 Cyan | Explored cells |

---

## 🇻🇳 Tiếng Việt

### 📖 Mô tả

Ứng dụng trực quan hóa **thuật toán tìm đường A\*** trong thời gian thực, di chuyển qua các mê cung được tạo ngẫu nhiên. Được xây dựng bằng Rust và thư viện game Macroquad, dự án này minh họa các hàm heuristic khác nhau và ảnh hưởng của chúng đến hiệu quả tìm đường.

### ✨ Tính năng

- 🎲 **Tạo mê cung ngẫu nhiên** - Sử dụng thuật toán quay lui đệ quy
- 🔍 **Tìm đường A\*** - Thuật toán đường đi ngắn nhất hiệu quả
- 📊 **8 heuristic khác nhau** - So sánh các phương pháp tính khoảng cách
- ⏱️ **Thống kê thời gian thực** - Theo dõi thời gian và số bước
- 🎮 **Điều khiển tương tác** - Chuyển đổi heuristic và tạo mê cung mới

### 🎯 Các heuristic có sẵn

| Phím | Heuristic | Mô tả |
|------|-----------|-------|
| `1` | Manhattan | Tổng hiệu tuyệt đối (chuẩn L1) |
| `2` | Euclidean | Khoảng cách đường thẳng (chuẩn L2) |
| `3` | Diagonal | Tối ưu cho di chuyển 8 hướng |
| `4` | Uniform Cost Search | Không dùng heuristic (thuật toán Dijkstra) |
| `5` | Chebyshev | Giá trị lớn nhất của hiệu tuyệt đối (chuẩn L∞) |
| `6` | Euclidean Squared | Euclidean không căn bậc 2 (nhanh hơn) |
| `7` | Weighted Manhattan | Manhattan × 2 (tích cực) |
| `8` | Manhattan Tiebreaker | Manhattan × 1.001 (phá vỡ đồng điểm) |

### 🚀 Bắt đầu

#### Yêu cầu

- [Rust](https://www.rust-lang.org/tools/install) (1.70+)

#### Cài đặt

```bash
# Clone repository
git clone <repository-url>
cd Project_I

# Build và chạy
cargo run --release
```

### 🎮 Điều khiển

| Phím | Hành động |
|------|-----------|
| `1-8` | Đổi hàm heuristic |
| `Space` | Tạo mê cung mới |

### 🛠️ Cấu trúc dự án

```
Project_I/
├── src/
│   ├── main.rs        # Điểm vào & vòng lặp game
│   ├── algorithm.rs   # Triển khai thuật toán A*
│   ├── maze.rs        # Tạo mê cung (quay lui đệ quy)
│   ├── heuristic.rs   # Các hàm heuristic
│   ├── node.rs        # Cấu trúc dữ liệu Node
│   └── constants.rs   # Các hằng số cấu hình
├── Cargo.toml
└── readme.md
```

### ⚙️ Cấu hình

Bạn có thể chỉnh sửa các hằng số trong `src/constants.rs`:

```rust
pub const MAZE_HEIGH: usize = 101;        // Chiều cao mê cung
pub const MAZE_WIDTH: usize = 201;        // Chiều rộng mê cung (2 * chiều cao - 1)
pub const STEP_DELAY_SEC: f64 = 0.0;      // Độ trễ giữa các bước
pub const STEPS_PER_FRAME: usize = 120;   // Số bước mỗi frame
```

### 🎨 Chú thích màu

| Màu | Ý nghĩa |
|-----|---------|
| ⬛ Đen | Tường |
| ⬜ Trắng | Đường đi |
| 🟩 Xanh lá | Điểm bắt đầu / Đường đi cuối cùng |
| 🔴 Đỏ | Đích |
| 🔵 Xanh dương | Ô đã khám phá |

---

## 📝 License | Giấy phép

This project is open source and available under the [MIT License](LICENSE).

Dự án này là mã nguồn mở và có sẵn theo [Giấy phép MIT](LICENSE).

---

<p align="center">
  Made with ❤️ using Rust | Được tạo bằng ❤️ với Rust
</p>
